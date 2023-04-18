terraform {
  backend "gcs" {
    bucket = "terraform-statefiles-never-delete-this-bucket"
    prefix = "staging-l1-jur-node"
  }
}
