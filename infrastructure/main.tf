
module "gcp" {
  source = "./gcp"

  name_prefix = var.gcp_name_prefix
  project     = var.gcp_project
  region      = var.gcp_region
  zone        = var.gcp_zone
  environment = var.DEPLOYMENT_ENVIRONMENT

  instance_1_number_of_cores = var.INSTANCE_1_NUMBER_OF_CORES
  instance_1_memory_mb       = var.INSTANCE_1_MEMORY_MB
  instance_1_disk_size_gb    = var.INSTANCE_1_DISK_SIZE_GB

  instance_2_number_of_cores = var.INSTANCE_2_NUMBER_OF_CORES
  instance_2_memory_mb       = var.INSTANCE_2_MEMORY_MB
  instance_2_disk_size_gb    = var.INSTANCE_2_DISK_SIZE_GB

  instance_3_number_of_cores = var.INSTANCE_3_NUMBER_OF_CORES
  instance_3_memory_mb       = var.INSTANCE_3_MEMORY_MB
  instance_3_disk_size_gb    = var.INSTANCE_3_DISK_SIZE_GB
}

module "cloudflare" {
  source = "./cloudflare"

  ip_address = module.gcp.ip_address
  zone_id    = var.cloudflare_zone_id
  sub_domain = var.SUB_DOMAIN
  api_token  = var.CLOUDFLARE_API_TOKEN
}
