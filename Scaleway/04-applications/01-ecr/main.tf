resource "scaleway_registry_namespace" "banana_back" {
  name        = "banana-back"
  description = "Registry for banana-back, a Node application"
  is_public   = false   # 🔒 reste privé
  region      = "fr-par"
}