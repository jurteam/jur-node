resource "google_compute_firewall" "l1_jur_node_compute_firewall" {
  name    = "${var.environment}-${var.name_prefix}-compute-firewall-${var.zone}"
  network = "default"

  # SSH access
  allow {
    protocol = "tcp"
    ports    = ["22"]
  }

  # P2P bootnode access
  allow {
    protocol = "tcp"
    ports    = ["30333"]
  }

  # ICMP access (ping)
  allow {
    protocol = "icmp"
  }

  # HTTP access
  allow {
    protocol = "tcp"
    ports    = ["80", "443"]
  }

  source_ranges = ["0.0.0.0/0"]
  target_tags   = [var.name_prefix]
}
