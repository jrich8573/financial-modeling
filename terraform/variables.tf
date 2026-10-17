variable "aws_region" {
  description = "AWS region to deploy into"
  type        = string
  default     = "us-east-1"
}

variable "instance_type" {
  description = "EC2 instance type"
  type        = string
  default     = "t3.small"
}

variable "key_name" {
  description = "Name of an existing EC2 key pair for SSH access (optional — leave empty to disable SSH key auth)"
  type        = string
  default     = ""
}

variable "ghcr_token" {
  description = "GitHub personal access token with read:packages scope for pulling GHCR images"
  type        = string
  sensitive   = true
}

variable "ghcr_username" {
  description = "GitHub username for GHCR authentication"
  type        = string
  default     = "jrich8573"
}
