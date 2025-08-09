resource "scaleway_k8s_pool" "banana" {
  cluster_id = data.scaleway_k8s_cluster.banana.id
  name       = "banana-pool"
  node_type  = "DEV1-M"
  size       = 1
  min_size   = 1
  max_size   = 2
  autoscaling = true
}