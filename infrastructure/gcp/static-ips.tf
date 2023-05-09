
resource "google_compute_address" "l1_jur_chain_archive_static_compute_address_1" {
  name = "${var.environment}-${var.name_prefix}-archive-static-address-${var.zone}-1"
}


resource "google_compute_address" "l1_jur_chain_static_compute_address_2" {
  name = "${var.environment}-${var.name_prefix}-static-address-${var.zone}-2"
}

resource "google_compute_address" "l1_jur_chain_static_compute_address_3" {
  name = "${var.environment}-${var.name_prefix}-static-address-${var.zone}-3"
}
