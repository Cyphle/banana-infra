terraform {
  required_version = ">= 1.12.2"

  required_providers {
    scaleway = {
      source  = "scaleway/scaleway"
      version = "~> 2.28"
    }
  }

  backend "s3" {
  }
}
