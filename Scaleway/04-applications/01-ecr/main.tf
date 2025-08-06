resource "scaleway_registry_namespace" "banana_back" {
  name        = "banana-back"
  description = "Registry for banana-back, a Node application"
  is_public   = false  
  region      = "fr-par"
}

resource "scaleway_registry_namespace" "banana_front" {
  name        = "banana-front"
  description = "Registry for banana-front, a React application"
  is_public   = false   
  region      = "fr-par"
}