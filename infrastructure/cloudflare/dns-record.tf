resource "cloudflare_record" "l1_jur_node_cloudflare_record" {
  zone_id = var.zone_id
  name    = var.sub_domain
  value   = var.ip_address
  type    = "A"
  proxied = true
  ttl     = "1"
}
