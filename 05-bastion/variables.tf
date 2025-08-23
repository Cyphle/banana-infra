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

variable "bastion_name" {
  type        = string
  default     = "bastion"
  description = "Name of the bastion instance"
}

variable "ssh_public_key" {
  type        = string
  description = "SSH public key for bastion access"
  sensitive   = true
}
