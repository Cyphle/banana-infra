resource "scaleway_vpc" "main" {
  provider = scaleway.main
  name = "cyphle-vpc"
}

resource "scaleway_vpc_private_network" "private_net" {
  provider = scaleway.main
  name        = "private-net"
  vpc_id      = scaleway_vpc.main.id
  region      = "fr-par"
}
