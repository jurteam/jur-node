
data "template_file" "default" {
  template = file("gcp/scripts/install-docker.sh")
  vars = {
    is_boot_node = "TRUE"
    key_prefix   = "INSTANCE_1"
    boot_node_ip = google_compute_address.l1_jur_chain_archive_static_compute_address_1.address
  }
}

resource "google_compute_instance" "l1_jur_chain_archive_compute_instance_1" {
  name         = "${var.environment}-${var.name_prefix}-archive-compute-instance-${var.zone}-1"
  machine_type = "custom-${var.instance_1_number_of_cores}-${var.instance_1_memory_mb}"

  tags = [var.name_prefix, var.environment]

  boot_disk {
    initialize_params {
      image = "ubuntu-os-cloud/ubuntu-2204-lts" #project/family
      size  = var.instance_1_disk_size_gb
    }
  }

  # network_interface
  network_interface {
    network = "default"

    access_config {
      nat_ip = google_compute_address.l1_jur_chain_archive_static_compute_address_1.address
    }
  }

  metadata_startup_script = data.template_file.default.rendered


  depends_on = [
    google_compute_address.l1_jur_chain_archive_static_compute_address_1
  ]
}
