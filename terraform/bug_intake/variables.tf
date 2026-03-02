variable "aws_region" {
  description = "AWS region for all resources"
  type        = string
  default     = "us-east-1"
}

variable "name_prefix" {
  description = "Prefix used for resource naming"
  type        = string
  default     = "catie"
}

variable "github_owner" {
  description = "GitHub org/user that will receive bug issues"
  type        = string
}

variable "github_repo" {
  description = "GitHub repo that will receive bug issues"
  type        = string
}

variable "github_token" {
  description = "GitHub token with issues write access"
  type        = string
  sensitive   = true
}

variable "ingest_api_key" {
  description = "Optional API key enforced by Lambda (x-api-key). Leave empty to disable."
  type        = string
  default     = ""
  sensitive   = true
}

variable "default_labels" {
  description = "Default labels applied to created issues"
  type        = string
  default     = "bug"
}

variable "bug_reports_ttl_days" {
  description = "TTL retention (days) for DynamoDB bug reports"
  type        = number
  default     = 90
}

variable "raw_reports_s3_bucket" {
  description = "Optional S3 bucket for raw JSON archival"
  type        = string
  default     = ""
}

variable "raw_reports_s3_prefix" {
  description = "Optional S3 key prefix for raw JSON archival"
  type        = string
  default     = "bug-reports/"
}
