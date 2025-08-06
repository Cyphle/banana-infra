# Banana scaleway

Here is the IaC to deploy Banana infrastructure on Scaleway.

The chosen target is Kubernetes Kapsule with managed PostgreSQL.

There is a goal of cost minimization as it is a personal project. There is also a learning objective.

## Commands


## Tips
- To use `.tfvars` file to file Terraform variables, use command `terraform apply -var-file="secrets.tfvars"`
- Use defined back-end : `terraform init -backend-config=backend.hcl`
