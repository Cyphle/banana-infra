resource "scaleway_registry_namespace" "banana" {
  provider = scaleway.main
  name        = "banana"
  description = "Namespace registry for banana application"
  is_public   = false  
  region      = "fr-par"
}