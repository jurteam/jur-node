variable "ip_address" {
  description = "The IP address of the GCP compute instance"
  type        = string
}

variable "zone_id" {
  description = "Zone Id of the Cloudflare account"
  type        = string
}

variable "sub_domain" {
  description = "Subdomain to use for the Cloudflare DNS record"
  type        = string
}

variable "api_token" {
  description = "API token for the Cloudflare account"
  type        = string
}
