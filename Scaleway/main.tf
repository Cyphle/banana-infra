# Configuration principale des providers Scaleway
# Ce fichier doit être exécuté en premier pour configurer l'alias

# Configuration du provider principal avec alias
provider "scaleway" {
  alias = "main"
  access_key      = var.scaleway_access_key
  secret_key      = var.scaleway_secret_key
  organization_id = var.organization_id
  project_id      = var.project_id
  region          = var.region
  zone            = var.zone
}

# Variables pour le provider
variable "scaleway_access_key" {
  type        = string
  sensitive   = true
  description = "Scaleway access key"
}

variable "scaleway_secret_key" {
  type        = string
  sensitive   = true
  description = "Scaleway secret key"
}

variable "organization_id" {
  type        = string
  sensitive   = true
  description = "Scaleway organization ID"
}

variable "project_id" {
  type        = string
  sensitive   = true
  description = "Scaleway project ID"
}

variable "region" {
  type        = string
  default     = "fr-par"
  description = "Scaleway region"
}

variable "zone" {
  type        = string
  default     = "fr-par-1"
  description = "Scaleway zone"
}
