#!/usr/bin/env bash
set -euo pipefail

ACCOUNT_ID="990860994492"
PROFILE_NAME="financial-models"
REGION="us-east-1"

echo "=== AWS Profile Setup for account ${ACCOUNT_ID} ==="
echo ""
echo "This configures an IAM access key profile for a standalone AWS account."
echo "You can create access keys in the AWS Console:"
echo "  IAM > Users > your-user > Security credentials > Create access key"
echo ""

read -rp "Enter your AWS Access Key ID: " AWS_ACCESS_KEY_ID
read -rsp "Enter your AWS Secret Access Key: " AWS_SECRET_ACCESS_KEY
echo ""

read -rp "Default region [${REGION}]: " INPUT_REGION
REGION="${INPUT_REGION:-${REGION}}"

echo ""
echo "Configuring profile [${PROFILE_NAME}]..."

aws configure set aws_access_key_id "${AWS_ACCESS_KEY_ID}" --profile "${PROFILE_NAME}"
aws configure set aws_secret_access_key "${AWS_SECRET_ACCESS_KEY}" --profile "${PROFILE_NAME}"
aws configure set region "${REGION}" --profile "${PROFILE_NAME}"
aws configure set output "json" --profile "${PROFILE_NAME}"

echo "Profile [${PROFILE_NAME}] written to ~/.aws/credentials and ~/.aws/config"
echo ""
echo "Verifying credentials..."
aws sts get-caller-identity --profile "${PROFILE_NAME}"

echo ""
echo "=== Done! ==="
echo ""
echo "To use this profile:"
echo "  export AWS_PROFILE=${PROFILE_NAME}"
echo ""
echo "Or pass it to Terraform:"
echo "  cd terraform && AWS_PROFILE=${PROFILE_NAME} terraform plan"
