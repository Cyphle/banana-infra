use std::process::Command;
use anyhow::anyhow;

/*
- Exécute directement un programme avec des arguments séparés
- Équivalent à kubectl cluster-info en ligne de commande
- Plus sûr (pas d'injection de commandes)
- Arguments échappés automatiquement
 */
pub fn run_command(program: &str, args: &[&str]) -> anyhow::Result<()> {
    let output = Command::new(program)
        .args(args)
        .output()
        .map_err(|e| anyhow!("Failed to execute {}: {}", program, e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("Command failed: {}", stderr));
    }

    // Afficher la sortie stdout si elle existe
    if !output.stdout.is_empty() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        print!("{}", stdout);
    }

    Ok(())
}

/*
- Passe par le shell (sh -c "commande complète")
- Permet d'utiliser des pipes |, redirections, etc.
- Équivalent à sh -c "kubectl create... | kubectl apply..."
- Plus risqué (injection possible si mal utilisé)
 */
pub fn run_shell_command(command: &str) -> anyhow::Result<()> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .map_err(|e| anyhow!("Failed to execute shell command: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("Shell command failed: {}", stderr));
    }

    // Afficher la sortie stdout si elle existe
    if !output.stdout.is_empty() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        print!("{}", stdout);
    }

    Ok(())
}