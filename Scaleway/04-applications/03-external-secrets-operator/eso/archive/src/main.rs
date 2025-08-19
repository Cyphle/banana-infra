mod command_runner;

use anyhow::{anyhow, Result};
use colored::*;
use std::env;
use std::process::Command;
use crate::command_runner::{run_command, run_shell_command};

#[tokio::main]
async fn main() -> Result<()> {
    // Charger les variables d'environnement depuis .env
    dotenv::dotenv().ok();

    println!("{}", "🚀 Setting up External Secrets Operator...".green().bold());

    // Vérifier les variables requises
    let required_vars = vec![
        "SCW_ACCESS_KEY_ID",
        "SCW_SECRET_ACCESS_KEY",
        "K8S_NAMESPACE"
    ];

    for var in &required_vars {
        env::var(var).map_err(|_| anyhow!("❌ Missing environment variable: {}", var))?;
    }

    let namespace = env::var("K8S_NAMESPACE")?;
    let scw_access_key = env::var("SCW_ACCESS_KEY_ID")?;
    let scw_secret_key = env::var("SCW_SECRET_ACCESS_KEY")?;

    // 1. Vérifier kubectl
    println!("{}", "🔧 Checking kubectl connection...".blue());
    run_command("kubectl", &["cluster-info"])
        .map_err(|_| anyhow!("❌ kubectl not configured or cluster unreachable"))?;

    // 2. Créer le namespace
    // println!("{}", format!("📁 Creating namespace {}...", namespace).blue());
    // let create_ns_cmd = format!(
    //     "kubectl create namespace {} --dry-run=client -o yaml | kubectl apply -f -",
    //     namespace
    // );
    // run_shell_command(&create_ns_cmd)
    //     .map_err(|_| anyhow!("❌ Failed to create namespace"))?;

    // 3. Installer ESO
    println!("{}", "⚙️ Installing External Secrets Operator...".blue());

    // Ajouter le repo Helm
    run_command("helm", &["repo", "add", "external-secrets", "https://charts.external-secrets.io"])
        .map_err(|_| anyhow!("❌ Failed to add helm repo"))?;

    run_command("helm", &["repo", "update"])
        .map_err(|_| anyhow!("❌ Failed to update helm repos"))?;

    // Vérifier si ESO est déjà installé
    let eso_check = Command::new("kubectl")
        .args(&["get", "namespace", "external-secrets-system"])
        .output();

    match eso_check {
        Ok(output) if output.status.success() => {
            println!("{}", "✅ ESO already installed".green());
        }
        _ => {
            println!("{}", "Installing ESO...".yellow());
            run_command("helm", &[
                "upgrade", "--install", "external-secrets", "external-secrets/external-secrets",
                "--namespace", "external-secrets-system",
                "--create-namespace",
                "--wait",
                "--timeout=300s"
            ]).map_err(|_| anyhow!("❌ Failed to install ESO"))?;
        }
    }

    // 4. Créer les credentials Scaleway
    println!("{}", "🔑 Creating Scaleway credentials...".blue());
    let create_secret_cmd = format!(
        "kubectl create secret generic scaleway-credentials \
         --from-literal=access-key=\"{}\" \
         --from-literal=secret-key=\"{}\" \
         --namespace={} \
         --dry-run=client -o yaml | kubectl apply -f -",
        scw_access_key, scw_secret_key, namespace
    );
    run_shell_command(&create_secret_cmd)
        .map_err(|_| anyhow!("❌ Failed to create Scaleway credentials"))?;
    
    // Ajouter le label requis pour le provider webhook
    let add_label_cmd = format!(
        "kubectl label secret scaleway-credentials -n {} external-secrets.io/type=webhook --overwrite",
        namespace
    );
    run_shell_command(&add_label_cmd)
        .map_err(|_| anyhow!("❌ Failed to add webhook label to Scaleway credentials"))?;

    // 5. Déployer la configuration ESO
    println!("{}", "📋 Deploying External Secrets configuration...".blue());
    run_command("kubectl", &["apply", "-f", "k8s/", "-n", &namespace])
        .map_err(|_| anyhow!("❌ Failed to deploy ESO configuration"))?;

    // 6. Attendre la synchronisation
    println!("{}", "⏳ Waiting for secrets synchronization...".blue());
    run_command("kubectl", &[
        "wait", "--for=condition=Ready", "externalsecret/postgres-secrets",
        "-n", &namespace, "--timeout=300s"
    ]).map_err(|_| anyhow!("❌ Postgres secrets failed to sync"))?;

    // 7. Vérification finale
    println!("{}", "✅ Verifying deployment...".green());

    let verify_secrets_cmd = format!("kubectl get secrets -n {} | grep postgres-secrets", namespace);
    run_shell_command(&verify_secrets_cmd)?;

    println!("{}", "🎉 External Secrets Operator setup complete!".green().bold());
    println!("{}", "📊 Status:".blue());

    run_command("kubectl", &["get", "externalsecrets", "-n", &namespace])?;
    run_command("kubectl", &["describe", "externalsecret/postgres-secrets", "-n", &namespace])?;

    Ok(())
}