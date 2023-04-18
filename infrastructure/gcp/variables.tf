variable "name_prefix" {
  description = "Prefix to use for naming resources"
  type        = string
}
variable "environment" {
  description = "Environment to deploy the GCP resources"
  type        = string
}
variable "project" {
  description = "Project Id of the GCP account"
  type        = string
}

variable "region" {
  description = "Region to deploy the GCP resources"
  type        = string
}

variable "zone" {
  description = "Zone to deploy the GCP resources"
  type        = string
}
