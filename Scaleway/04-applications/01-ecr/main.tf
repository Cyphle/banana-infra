resource "scaleway_registry_namespace" "banana" {
  name        = "banana"
  description = "Namespace registry for banana application"
  is_public   = false  
  region      = "fr-par"
}