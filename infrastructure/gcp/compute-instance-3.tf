data "template_file" "template_file_instance_3" {
  template = file("gcp/scripts/install-docker.sh")
  vars = {
    is_boot_node           = "FALSE"
    key_prefix             = "INSTANCE_3"
    boot_node_ip           = google_compute_address.l1_jur_chain_archive_static_compute_address_1.address
    deployment_environment = "${var.environment}"
  }
}

resource "google_compute_instance" "l1_jur_chain_compute_instance_3" {
  name         = "${var.environment}-${var.name_prefix}-compute-instance-${var.zone}-3"
  machine_type = "custom-${var.instance_3_number_of_cores}-${var.instance_3_memory_mb}"

  tags = [var.name_prefix, var.environment]

  boot_disk {
    initialize_params {
      image = "ubuntu-os-cloud/ubuntu-2204-lts" #project/family
      size  = var.instance_3_disk_size_gb
    }
  }

  # network_interface
  network_interface {
    network = "default"

    access_config {
      nat_ip = google_compute_address.l1_jur_chain_static_compute_address_3.address
    }
  }

  metadata_startup_script = data.template_file.template_file_instance_3.rendered

  depends_on = [
    google_compute_address.l1_jur_chain_static_compute_address_3
  ]
}
