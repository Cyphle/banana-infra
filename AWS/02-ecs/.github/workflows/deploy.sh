#!/bin/bash

# Script de déploiement pour GitHub Actions
# Ce script met à jour le service ECS avec une nouvelle image

set -e

# Variables d'environnement (à définir dans GitHub Actions)
AWS_REGION=${AWS_REGION:-"eu-west-3"}
ECS_CLUSTER_NAME=${ECS_CLUSTER_NAME:-"banana-cluster"}
ECS_SERVICE_NAME=${ECS_SERVICE_NAME:-"banana-front-service"}
ECS_TASK_DEFINITION_FAMILY=${ECS_TASK_DEFINITION_FAMILY:-"banana-front"}
NEW_IMAGE=${NEW_IMAGE:-"rg.fr-par.scw.cloud/banana/banana-front:latest"}

echo "🚀 Déploiement de l'application sur ECS..."
echo "Cluster: $ECS_CLUSTER_NAME"
echo "Service: $ECS_SERVICE_NAME"
echo "Nouvelle image: $NEW_IMAGE"
echo "Région: $AWS_REGION"

# 1. Récupérer la définition de tâche actuelle
echo "📋 Récupération de la définition de tâche actuelle..."
TASK_DEFINITION=$(aws ecs describe-task-definition \
  --task-definition "$ECS_TASK_DEFINITION_FAMILY" \
  --region "$AWS_REGION" \
  --query 'taskDefinition' \
  --output json)

# 2. Créer une nouvelle définition de tâche avec la nouvelle image
echo "🔄 Création d'une nouvelle définition de tâche..."
NEW_TASK_DEFINITION=$(echo "$TASK_DEFINITION" | \
  jq --arg IMAGE "$NEW_IMAGE" \
  '.containerDefinitions[0].image = $IMAGE | del(.taskDefinitionArn, .revision, .status, .requiresAttributes, .placementConstraints, .compatibilities, .registeredAt, .registeredBy)')

# 3. Enregistrer la nouvelle définition de tâche
echo "💾 Enregistrement de la nouvelle définition de tâche..."
NEW_TASK_DEFINITION_ARN=$(aws ecs register-task-definition \
  --cli-input-json "$NEW_TASK_DEFINITION" \
  --region "$AWS_REGION" \
  --query 'taskDefinition.taskDefinitionArn' \
  --output text)

echo "✅ Nouvelle définition de tâche créée: $NEW_TASK_DEFINITION_ARN"

# 4. Mettre à jour le service ECS
echo "🔄 Mise à jour du service ECS..."
aws ecs update-service \
  --cluster "$ECS_CLUSTER_NAME" \
  --service "$ECS_SERVICE_NAME" \
  --task-definition "$NEW_TASK_DEFINITION_ARN" \
  --region "$AWS_REGION"

echo "✅ Service ECS mis à jour avec la nouvelle définition de tâche"

# 5. Attendre que le déploiement soit terminé
echo "⏳ Attente de la fin du déploiement..."
aws ecs wait services-stable \
  --cluster "$ECS_CLUSTER_NAME" \
  --services "$ECS_SERVICE_NAME" \
  --region "$AWS_REGION"

echo "🎉 Déploiement terminé avec succès!"

# 6. Afficher le statut du service
echo "📊 Statut du service:"
aws ecs describe-services \
  --cluster "$ECS_CLUSTER_NAME" \
  --services "$ECS_SERVICE_NAME" \
  --region "$AWS_REGION" \
  --query 'services[0].{ServiceName:serviceName,Status:status,DesiredCount:desiredCount,RunningCount:runningCount,PendingCount:pendingCount}' \
  --output table

# 7. Afficher l'URL de l'application
echo "🌐 URL de l'application:"
aws elbv2 describe-load-balancers \
  --region "$AWS_REGION" \
  --query 'LoadBalancers[?contains(LoadBalancerName, `banana-front`)].DNSName' \
  --output text | sed 's/^/http:\/\//'
