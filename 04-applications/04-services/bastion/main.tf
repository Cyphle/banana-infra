
# Groupe de sécurité pour le bastion
resource "scaleway_instance_security_group" "bastion_sg" {
  name        = "${var.bastion_name}-sg"
  description = "Security group for bastion host"

  inbound_rule {
    action   = "accept"
    protocol = "TCP"
    port     = 22
    ip_range = "0.0.0.0/0"
  }

  outbound_rule {
    action   = "accept"
    protocol = "TCP"
    port_range = "1-65535"
    ip_range = "0.0.0.0/0"
  }

  outbound_rule {
    action   = "accept"
    protocol = "UDP"
    port_range = "1-65535"
    ip_range = "0.0.0.0/0"
  }

  outbound_rule {
    action   = "accept"
    protocol = "ICMP"
    ip_range = "0.0.0.0/0"
  }
}

# IP publique pour le bastion
resource "scaleway_instance_ip" "bastion_ip" {
  type = "nat"
}

# Instance bastion
resource "scaleway_instance_server" "bastion" {
  name              = var.bastion_name
  type              = local.server_type
  image             = data.scaleway_instance_image.debian.id
  ip_id             = scaleway_instance_ip.bastion_ip.id
  security_group_id = scaleway_instance_security_group.bastion_sg.id
  
  # Enable dynamic IP
  enable_dynamic_ip = true

  # Configuration du réseau privé
  private_network {
    pn_id = data.scaleway_vpc_private_network.private_net.id
  }

  # Script d'initialisation pour configurer le bastion
  user_data = {
    cloud-init = <<-EOF
      #cloud-config
      package_update: true
      package_upgrade: true
      packages:
        - htop
        - curl
        - wget
        - vim
        - net-tools
        - tcpdump
      
      # Inject SSH public key
      ssh_authorized_keys:
        - ${var.ssh_public_key}
      
      # Configuration SSH sécurisée
      write_files:
        - path: /etc/ssh/sshd_config.d/99-bastion.conf
          content: |
            PermitRootLogin no
            PasswordAuthentication no
            PubkeyAuthentication yes
            X11Forwarding no
            AllowTcpForwarding yes
            GatewayPorts no
            ClientAliveInterval 300
            ClientAliveCountMax 2
          permissions: '0644'
      
      runcmd:
        - systemctl restart sshd
        - echo "Bastion configuré avec succès" > /var/log/bastion-setup.log
    EOF
  }

  tags = [
    "bastion",
    "infrastructure",
    "security"
  ]
}
