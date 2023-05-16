output "ip_address" {
  description = "The IP address of the GCP compute instance"
  value       = google_compute_address.l1_jur_chain_archive_static_compute_address_1.address
}
