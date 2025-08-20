resource "scaleway_rdb_instance" "banana-keycloak-db" {
  name          = "banana-keycloak-db"
  engine        = "PostgreSQL-16"
  node_type     = "DB-DEV-S"
  is_ha_cluster = true
  user_name     = var.keycloak_db_user
  password      = var.keycloak_db_password

  private_network {
    pn_id      = data.scaleway_vpc_private_network.private_net.id
    enable_ipam = true
  }
}

resource "scaleway_rdb_database" "banana-keycloak-db" {
  name        = "bananadb"
  instance_id = scaleway_rdb_instance.banana-keycloak-db.id
}
