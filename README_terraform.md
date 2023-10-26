# creating an instance with terraform


```hcl
provider "aws" {
    region = "us-east-1"
}

resource "aws_instance" "example" {
    ami           = "ami-0c94855ba95c71c99"
    instance_type = "t2.micro"
    key_name      = "my-key-pair"
    security_groups = [
        "default"
    ]

    tags = {
        Name = "example-instance"
    }
}
```
