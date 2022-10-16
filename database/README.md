## Building

Build via instructions at https://github.com/aws/aws-sam-cli

First install `cargo-lambda`

Next clone this repo and build it

```shell
git clone https://github.com/SpicyRicecaker/ucr-dining
cd ucr-dining/database
cargo lambda build --release --arm64
# next, add credentials via
# https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-files.html
# before deploying below
cargo lambda deploy
# you can also specify a server region closest to you that
# also supporst arm64, e.g. -r us-west-2
```

## Notes

Remember not to configure cross origin requests in the amazon console as we already defined things in code. Adding them to the console will add a duplicate cross-origin headers. These duplicate headers (e.g. `*,*`) will restrict access instead of granting everyone access.