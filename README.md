## Building

Build via instructions at https://github.com/aws/aws-sam-cli

First install `cargo-lambda`

Next clone this repo and build it
```shell
git clone https://github.com/SpicyRicecaker/ucr-dining-lambda
cargo lambda build --release --arm64
# next, add credentials via
# https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-files.html
# before deploying below
cargo lambda deploy 
# you can also specify a server region closest to you that
# also supporst arm64, e.g. -r us-west-2
```