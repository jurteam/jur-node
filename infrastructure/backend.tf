terraform {
  backend "gcs" {
    bucket = "terraform-statefiles-never-delete-this-bucket"
    prefix = "local-l1-jur-node"
  }
}
