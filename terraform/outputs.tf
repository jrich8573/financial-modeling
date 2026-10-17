output "public_ip" {
  description = "Public IP of the EC2 instance (not directly accessible)"
  value       = aws_instance.app.public_ip
}

output "cloudfront_url" {
  description = "HTTPS URL via CloudFront"
  value       = "https://${aws_cloudfront_distribution.app.domain_name}"
}

output "ssm_command" {
  description = "SSM Session Manager command"
  value       = "aws ssm start-session --target ${aws_instance.app.id}"
}
