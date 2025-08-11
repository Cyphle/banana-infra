# AWS EKS (Elastic Kubernetes Service)

Ce dossier contient la configuration Terraform pour créer un cluster Kubernetes managé sur AWS via EKS.

## 🏗️ Ressources créées

### Cluster EKS
- **Ressource** : `aws_eks_cluster.banana`
- **Nom** : `banana-cluster`
- **Version Kubernetes** : `1.30`
- **Fonction** : Cluster Kubernetes managé par AWS
- **Configuration** : 
  - Accès privé et public activé
  - Logs d'audit activés (API, audit, authenticator, controllerManager, scheduler)
  - Utilise les subnets publics et privés du VPC

### Node Group EKS
- **Ressource** : `aws_eks_node_group.banana`
- **Nom** : `banana-node-group`
- **Fonction** : Groupe de nœuds worker pour exécuter les pods
- **Configuration** :
  - **Instance type** : `t3.medium` (2 vCPU, 4 GB RAM)
  - **Taille** : 1-2 nœuds avec autoscaling
  - **Subnets** : Uniquement les subnets privés pour la sécurité
  - **Mise à jour** : Maximum 1 nœud indisponible pendant les mises à jour

### IAM Roles et Policies
#### Role Cluster EKS
- **Ressource** : `aws_iam_role.eks_cluster`
- **Fonction** : Permissions pour le service EKS
- **Policies attachées** :
  - `AmazonEKSClusterPolicy` : Permissions de base pour EKS
  - `AmazonEKSVPCResourceController` : Gestion des ressources VPC

#### Role Node Group
- **Ressource** : `aws_iam_role.eks_node_group`
- **Fonction** : Permissions pour les nœuds worker
- **Policies attachées** :
  - `AmazonEKSWorkerNodePolicy` : Permissions de base pour les nœuds
  - `AmazonEKS_CNI_Policy` : Gestion du réseau (CNI)
  - `AmazonEC2ContainerRegistryReadOnly` : Lecture des images ECR

### Security Group
- **Ressource** : `aws_security_group.eks_cluster`
- **Fonction** : Contrôle d'accès réseau pour le cluster
- **Règles** :
  - **Entrant** : Port 443 (HTTPS) depuis partout pour l'API Kubernetes
  - **Sortant** : Tous les ports et protocoles

## 🔄 Fonctionnement des ressources entre elles

### Architecture EKS et flux de communication

#### 1. **Communication API Kubernetes**
```
Internet → Security Group (Port 443) → API Server EKS → Control Plane AWS
```

**Détail du processus :**
- L'**API Server EKS** écoute sur le port 443 (HTTPS)
- Le **Security Group** autorise le trafic entrant depuis Internet
- Les **subnets publics** permettent l'accès à l'API
- Le **Control Plane AWS** gère l'état du cluster

#### 2. **Communication entre nœuds et pods**
```
Node Group → Subnets Privés → VPC interne → Communication inter-pods
```

**Détail du processus :**
- Les **nœuds worker** sont déployés dans les subnets privés
- Le **CNI (Container Network Interface)** gère l'adressage des pods
- La **communication inter-pods** se fait via le réseau VPC interne
- Pas d'exposition directe à Internet pour les nœuds

#### 3. **Gestion des workloads**
```
kubectl → API Server → Scheduler → Node Group → Pods
```

**Détail du processus :**
- **kubectl** communique avec l'API Server EKS
- Le **Scheduler** décide où placer les pods
- Les **nœuds** reçoivent et exécutent les pods
- **Autoscaling** ajuste automatiquement le nombre de nœuds

### Dépendances et ordre de création

#### **Ordre de création Terraform :**
1. **IAM Roles** : Créés en premier car EKS en a besoin
2. **Security Group** : Définit les règles de communication
3. **Cluster EKS** : Utilise les IAM roles et security groups
4. **Node Group** : Déployé après le cluster et utilise ses ressources

#### **Dépendances critiques :**
- **Cluster EKS** → **IAM Role Cluster** : Le cluster a besoin des permissions
- **Cluster EKS** → **Security Group** : Contrôle d'accès réseau
- **Node Group** → **Cluster EKS** : Les nœuds rejoignent le cluster existant
- **Node Group** → **Subnets Privés** : Placement des nœuds

### Intégration avec le VPC

#### **Utilisation des subnets :**
- **Subnets Publics** : API Server EKS accessible depuis Internet
- **Subnets Privés** : Nœuds worker isolés et sécurisés
- **Route Tables** : NAT Gateway pour l'accès Internet sortant des nœuds

#### **Avantages de cette architecture :**
- **Sécurité** : Nœuds dans des subnets privés
- **Accessibilité** : API accessible depuis Internet pour la gestion
- **Performance** : Communication interne optimisée via VPC
- **Scalabilité** : Autoscaling basé sur la charge

### Gestion des identités et permissions

#### **IAM Roles et Trust Relationships :**
```
Cluster EKS ← Trusts → aws-eks-cluster.amazonaws.com
Node Group ← Trusts → ec2.amazonaws.com
```

#### **Policies et permissions :**
- **Cluster Policy** : Gestion du cluster, logs, monitoring
- **VPC Resource Controller** : Gestion des ENI (Elastic Network Interfaces)
- **Worker Node Policy** : Permissions de base pour les nœuds
- **CNI Policy** : Gestion du réseau des pods
- **ECR Read Policy** : Accès en lecture aux images Docker

### Communication réseau détaillée

#### **Ports et protocoles :**
- **Port 443** : API Kubernetes (HTTPS)
- **Port 10250** : Kubelet sur les nœuds (interne)
- **Port 10255** : Read-only Kubelet (optionnel)
- **Port 30000-32767** : NodePort services (dynamiques)

#### **Security Group Rules :**
```
Ingress Rules:
- Port 443 depuis 0.0.0.0/0 (API access)

Egress Rules:
- All traffic vers 0.0.0.0/0 (Internet access via NAT)
```

### Autoscaling et gestion des ressources

#### **Cluster Autoscaler :**
- **Fonctionnement** : Surveille les pods en attente
- **Décision** : Ajoute des nœuds si nécessaire
- **Limites** : Respecte min_size et max_size
- **Cooldown** : Évite les changements trop fréquents

#### **Node Group Scaling :**
```
Scaling Up:
Pod en attente → Autoscaler → Nouveau nœud → Pod déployé

Scaling Down:
Nœud sous-utilisé → Drain des pods → Suppression du nœud
```

## 🚀 Déploiement

```bash
# Initialiser Terraform
terraform init

# Voir le plan de déploiement
terraform plan -var-file="secrets.tfvars"

# Déployer l'infrastructure
terraform apply -var-file="secrets.tfvars"

# Détruire l'infrastructure
terraform destroy -var-file="secrets.tfvars"
```

## 📋 Variables requises

Créez un fichier `secrets.tfvars` avec :
```hcl
aws_access_key = "VOTRE_ACCESS_KEY"
aws_secret_key = "VOTRE_SECRET_KEY"
aws_region     = "eu-west-3"  # Optionnel, défaut: eu-west-3
```

## 🔧 Configuration kubectl

Après le déploiement, configurez kubectl :

```bash
# Mettre à jour le kubeconfig
aws eks update-kubeconfig --region eu-west-3 --name banana-cluster

# Vérifier la connexion
kubectl get nodes
kubectl get pods --all-namespaces
```

## 🔒 Sécurité

- **Nœuds dans subnets privés** : Pas d'accès direct depuis Internet
- **IAM Roles** : Permissions minimales nécessaires
- **Security Groups** : Accès restreint à l'API Kubernetes uniquement
- **Logs d'audit** : Traçabilité complète des actions

## 💰 Coûts estimés

- **Cluster EKS** : ~$0.10/heure (~$73/mois)
- **Nœuds t3.medium** : ~$0.0416/heure (~$30/mois par nœud)
- **Total estimé** : ~$133/mois pour 1 nœud, ~$163/mois pour 2 nœuds

## 📊 Architecture

```
Internet
    ↓
Load Balancer (si configuré)
    ↓
API Server EKS (subnet public)
    ↓
Worker Nodes (subnets privés)
    ↓
Pods Kubernetes
```

## 🚨 Dépendances

**IMPORTANT** : Ce module dépend du module VPC (`01-vpc`). Assurez-vous de déployer le VPC en premier.

## 🔄 Autoscaling

Le cluster est configuré avec :
- **Min** : 1 nœud
- **Max** : 2 nœuds
- **Désiré** : 1 nœud
- **Autoscaling** : Activé pour s'adapter à la charge

## 📝 Notes importantes

- Les nœuds sont déployés dans les subnets privés pour la sécurité
- L'API EKS est accessible depuis Internet pour la gestion
- Les logs d'audit sont activés pour la conformité
- Le cluster utilise la version LTS de Kubernetes (1.30)

## 🔍 Monitoring et surveillance

### **Métriques CloudWatch disponibles :**
- **Cluster EKS** : CPU, mémoire, pods, nœuds
- **Node Group** : Utilisation des ressources par nœud
- **Security Groups** : Trafic réseau, connexions

### **Logs EKS :**
- **API Server** : Toutes les requêtes API
- **Audit** : Actions d'authentification et d'autorisation
- **Controller Manager** : Gestion des contrôleurs
- **Scheduler** : Décisions de placement des pods

## 🚨 Points d'attention

### **Limitations actuelles :**
1. **API publique** : Accessible depuis Internet (peut être restreint)
2. **Nœuds dans une seule AZ** : Pour les subnets privés
3. **Security Group basique** : Règles minimales

### **Améliorations possibles :**
1. **API privée uniquement** : Restriction d'accès à l'API
2. **Nœuds multi-AZ** : Distribution sur plusieurs AZ
3. **Security Groups avancés** : Règles plus granulaires
4. **Cluster Autoscaler** : Configuration d'autoscaling avancée
5. **Monitoring avancé** : Prometheus, Grafana, etc.
