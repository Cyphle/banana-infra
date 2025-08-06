resource "scaleway_rdb_instance" "pg" {
  name          = "banana-db"
  engine        = "PostgreSQL-15"
  node_type = "DB-DEV-S"
  is_ha_cluster = true
  user_name     = var.db_user
  password      = var.db_password

  private_network {
    pn_id = data.scaleway_vpc_private_network.private_net.id
  }
}

resource "scaleway_rdb_database" "app" {
  name        = "bananadb"
  instance_id = scaleway_rdb_instance.pg.id
}