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

variable "INSTANCE_1_NUMBER_OF_CORES" {
  description = "Number of cores for the instance 1"
  type        = string
}

variable "INSTANCE_1_MEMORY_MB" {
  description = "RAM for the instance 1"
  type        = string
}

variable "INSTANCE_1_DISK_SIZE_GB" {
  description = "Disk size for the instance 1"
  type        = string
}

variable "INSTANCE_2_NUMBER_OF_CORES" {
  description = "Number of cores for the instance 2"
  type        = string
}

variable "INSTANCE_2_MEMORY_MB" {
  description = "RAM for the instance 2"
  type        = string
}

variable "INSTANCE_2_DISK_SIZE_GB" {
  description = "Disk size for the instance 2"
  type        = string
}

variable "INSTANCE_3_NUMBER_OF_CORES" {
  description = "Number of cores for the instance 3"
  type        = string
}

variable "INSTANCE_3_MEMORY_MB" {
  description = "RAM for the instance 3"
  type        = string
}

variable "INSTANCE_3_DISK_SIZE_GB" {
  description = "Disk size for the instance 3"
  type        = string
}
