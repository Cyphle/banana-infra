output "bastion_public_ip" {
  description = "Public IP address of the bastion host"
  value       = scaleway_instance_ip.bastion_ip.address
}

output "bastion_ssh_command" {
  description = "SSH command to connect to the bastion"
  value       = "ssh -i ~/.ssh/your_private_key ubuntu@${scaleway_instance_ip.bastion_ip.address}"
}

output "bastion_instance_id" {
  description = "Instance ID of the bastion host"
  value       = scaleway_instance_server.bastion.id
}
