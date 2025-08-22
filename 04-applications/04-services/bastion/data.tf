data "scaleway_vpc" "main" {
  name = "cyphle-vpc"
}

data "scaleway_vpc_private_network" "private_net" {
  name   = "private-net"
  vpc_id = data.scaleway_vpc.main.id
}

data "scaleway_instance_image" "debian" {
  architecture = "x86_64"
  name         = "Debian 12 (Bookworm)"
}
