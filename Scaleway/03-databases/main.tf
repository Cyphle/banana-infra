resource "scaleway_rdb_instance" "banana-db" {
  name          = "banana-db"
  engine        = "PostgreSQL-16"
  node_type     = "DB-DEV-S"
  is_ha_cluster = true
  user_name     = var.db_user
  password      = var.db_password

  private_network {
    pn_id      = data.scaleway_vpc_private_network.private_net.id
    enable_ipam = true
  }
}

resource "scaleway_rdb_database" "banana-db" {
  name        = "bananadb"
  instance_id = scaleway_rdb_instance.banana-db.id
}

resource "scaleway_redis_cluster" "banana_redis" {
  name         = "banana-redis"
  version      = "7.0.5"
  node_type    = "RED1-MICRO"
  cluster_size = 1
  user_name    = var.redis_user
  password     = var.redis_password

  private_network {
    id = data.scaleway_vpc_private_network.private_net.id
  }
}