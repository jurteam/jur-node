
module "gcp" {
  source = "./gcp"

  name_prefix = var.gcp_name_prefix
  project     = var.gcp_project
  region      = var.gcp_region
  zone        = var.gcp_zone
  environment = var.DEPLOYMENT_ENVIRONMENT
}

module "cloudflare" {
  source = "./cloudflare"

  ip_address = module.gcp.ip_address
  zone_id    = var.cloudflare_zone_id
  sub_domain = var.SUB_DOMAIN
  api_token  = var.CLOUDFLARE_API_TOKEN
}
