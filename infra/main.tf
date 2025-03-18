variable "aws_profile" {
  type        = string
  default     = "april_hiro"
}

terraform {
  required_version = "~>1"

  required_providers {
    aws = {
        source = "hashicorp/aws"
        version = "~>5"
    }
  }

    backend "s3" {
        bucket = "rust-api-terra-form"
        region = "ap-northeast-1"
        profile = "april_hiro"
        key = "rust-api.tfstate"
        encrypt = true
    }
}
