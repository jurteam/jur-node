# GCP variables for the infrastructure
variable "gcp_name_prefix" {
  description = "Prefix to use for naming GCP resources"
  type        = string
}

variable "gcp_project" {
  description = "Project Id of the GCP account"
  type        = string
}

variable "gcp_region" {
  description = "Region to deploy the GCP resources"
  type        = string
}

variable "gcp_zone" {
  description = "Zone to deploy the GCP resources"
  type        = string
}

# Cloudflare variables for the infrastructure
variable "cloudflare_zone_id" {
  description = "Zone Id of the Cloudflare account"
  type        = string
}

variable "SUB_DOMAIN" {
  description = "Subdomain to use for the Cloudflare DNS record"
  type        = string
}

variable "CLOUDFLARE_API_TOKEN" {
  description = "API token for the Cloudflare account"
  type        = string
}

variable "DEPLOYMENT_ENVIRONMENT" {
  description = "Environment to deploy the infrastructure"
  type        = string
}

