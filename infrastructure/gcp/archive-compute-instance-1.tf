
resource "google_compute_instance" "l1_jur_chain_archive_compute_instance_1" {
  name         = "${var.environment}-${var.name_prefix}-archive-compute-instance-${var.zone}-1"
  machine_type = "e2-standard-4"

  tags = [var.name_prefix, var.environment]

  boot_disk {
    initialize_params {
      image = "ubuntu-os-cloud/ubuntu-2204-lts" #project/family
      size  = 1000
    }
  }

  # network_interface
  network_interface {
    network = "default"

    access_config {
      nat_ip = google_compute_address.l1_jur_chain_archive_static_compute_address_1.address
    }
  }

  metadata_startup_script = file("gcp/scripts/install-docker.sh")


  depends_on = [
    google_compute_address.l1_jur_chain_archive_static_compute_address_1
  ]
}
