output "public_ip" {
  description = "Public IP of the EC2 instance"
  value       = aws_instance.app.public_ip
}

output "public_dns" {
  description = "Public DNS of the EC2 instance"
  value       = aws_instance.app.public_dns
}

output "app_url" {
  description = "URL to access the application"
  value       = "http://${aws_instance.app.public_dns}"
}

output "ssh_command" {
  description = "SSH command to connect to the instance"
  value       = var.key_name != "" ? "ssh -i ~/.ssh/${var.key_name}.pem ec2-user@${aws_instance.app.public_ip}" : "Use SSM Session Manager: aws ssm start-session --target ${aws_instance.app.id}"
}
