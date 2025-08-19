# A propos
Ces instructions et ce code a été généré par Claude mais ça marche pas. Doc officielle à utiliser...

## Utilisation
1. Setup initial (une seule fois)
   - bashcd banana-infra
   - cargo build --release
   - ./target/release/setup-eso
2. Déploiement de l'application
   - bashcd banana-back
   - kubectl apply -f k8s/ -n production
   - kubectl rollout status deployment/banana-back -n production
3. Vérification
   - bashkubectl get pods -n production
   - kubectl get secrets -n production | grep postgres
   - kubectl logs -f deployment/banana-back -n production