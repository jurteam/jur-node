terraform {
  backend "gcs" {
    bucket = "terraform-statefiles-never-delete-this-bucket"
    prefix = "l1-jur-node"
  }
}
