# Banana scaleway

Here is the IaC to deploy Banana infrastructure on Scaleway.

The chosen target is Kubernetes Kapsule with managed PostgreSQL.

There is a goal of cost minimization as it is a personal project. There is also a learning objective.

## Commands
### Ingress controller and certificate
- To uninstall an helm chart: `helm uninstall cert-manager -n cert-manager`
- To check certificate: `kubectl get cert -n banana`
- To check certificate: `kubectl describe certificate banana-tls -n banana`

## Tips
- To use `.tfvars` file to file Terraform variables, use command `terraform apply -var-file="secrets.tfvars"`
- Use defined back-end : `terraform init -backend-config=backend.hcl`

## Resources
- Ingress controller & HTTPS: https://hbayraktar.medium.com/installing-cert-manager-and-nginx-ingress-with-lets-encrypt-on-kubernetes-fe0dff4b1924

## TODO
- A revoir ce readme