terraform {
  required_providers {
    cloudflare = {
      source  = "cloudflare/cloudflare"
      version = "4.3.0"
    }
  }
}

provider "cloudflare" {
  api_token = var.api_token
}
