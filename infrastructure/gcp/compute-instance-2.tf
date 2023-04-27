
resource "google_compute_instance" "l1_jur_chain_compute_instance_2" {
  name         = "${var.environment}-${var.name_prefix}-compute-instance-${var.zone}-2"
  machine_type = "custom-${var.instance_2_number_of_cores}-${var.instance_2_memory_mb}"

  tags = [var.name_prefix, var.environment]

  boot_disk {
    initialize_params {
      image = "ubuntu-os-cloud/ubuntu-2204-lts" #project/family
      size  = var.instance_2_disk_size_gb
    }
  }

  # network_interface
  network_interface {
    network = "default"

    access_config {
      nat_ip = google_compute_address.l1_jur_chain_static_compute_address_2.address
    }
  }

  # metadata_startup_script = "echo  BOOT_NODE_IP=${google_compute_address.l1_jur_chain_archive_static_compute_address_1.address} >> /etc/profile"
  metadata_startup_script = file("gcp/scripts/install-docker.sh")

  depends_on = [
    google_compute_address.l1_jur_chain_static_compute_address_2,
    google_compute_instance.l1_jur_chain_archive_compute_instance_1
  ]
}
