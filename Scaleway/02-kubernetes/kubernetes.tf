resource "scaleway_k8s_cluster" "banana" {
  name                      = "banana-cluster"
  version                   = "1.30.14"
  cni                       = "cilium"
  region                    = "fr-par"
  private_network_id        = data.scaleway_vpc_private_network.private-net.id
  delete_additional_resources = true

  autoscaler_config {
    disable_scale_down = true
  }
}

resource "scaleway_k8s_pool" "banana" {
  cluster_id = scaleway_k8s_cluster.banana.id
  name       = "banana-pool"
  node_type  = "DEV1-M"
  size       = 1
  min_size   = 1
  max_size   = 2
  autoscaling = true
}