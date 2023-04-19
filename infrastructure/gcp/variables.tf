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

variable "instance_1_number_of_cores" {
  description = "Number of cores for the instance 1"
  type        = string
}

variable "instance_1_memory_mb" {
  description = "RAM for the instance 1"
  type        = string
}

variable "instance_1_disk_size_gb" {
  description = "Disk size for the instance 1"
  type        = string
}

variable "instance_2_number_of_cores" {
  description = "Number of cores for the instance 2"
  type        = string
}

variable "instance_2_memory_mb" {
  description = "RAM for the instance 2"
  type        = string
}

variable "instance_2_disk_size_gb" {
  description = "Disk size for the instance 2"
  type        = string
}

variable "instance_3_number_of_cores" {
  description = "Number of cores for the instance 3"
  type        = string
}

variable "instance_3_memory_mb" {
  description = "RAM for the instance 3"
  type        = string
}

variable "instance_3_disk_size_gb" {
  description = "Disk size for the instance 3"
  type        = string
}
