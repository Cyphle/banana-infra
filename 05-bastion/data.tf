data "scaleway_vpc" "main" {
  name = "cyphle-vpc"
}

data "scaleway_vpc_private_network" "private_net" {
  name   = "private-net"
  vpc_id = data.scaleway_vpc.main.id
}

# Image Ubuntu LTS 22.04 (Jammy) - plus stable et disponible
data "scaleway_instance_image" "debian" {
  architecture = "x86_64"
  name         = "Ubuntu 22.04 Jammy Jellyfish"
  latest       = true
}
