# Utilisation
## 1. Build et déploiement
```
cd banana-back

# Build de l'image Docker
docker build -t rg.fr-par.scw.cloud/banana/example-app:latest .

# Push vers le registry
docker push rg.fr-par.scw.cloud/banana/example-app:latest

# Déploiement
kubectl apply -f k8s/ -n banana
```

## 2. Vérification des logs
```
# Voir les logs de l'application
kubectl logs -f deployment/banana-back -n banana

# Tester les endpoints
kubectl port-forward service/banana-back-service 8080:80 -n banana

# Dans un autre terminal
curl http://localhost:8080/health
curl http://localhost:8080/secrets
curl http://localhost:8080/env
curl http://localhost:8080/logs
```
