terraform {
  required_providers {
    scaleway = {
      source  = "scaleway/scaleway"
      version = "~> 2.28"
      configuration_aliases = [scaleway.main]
    }
  }
}
