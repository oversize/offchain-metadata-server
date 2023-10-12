# Cardano off-chain metadata API

A simple implementation of the metadata server proposed in
[CIP-26](https://cips.cardano.org/cips/cip26/) in Rust.

## Abstract

The metadata is currently stored in github repositories where new submissions can be
made via pull requests. One for [mainnet](https://github.com/cardano-foundation/cardano-token-registry) and one for the [testnets](https://github.com/input-output-hk/metadata-registry-testnet). While everyone can easily just grab the git repo
and work with the data as needed, a public API provides endpoints for even easier
consumption. That api for mainnet(!) is located here: <https://tokens.cardano.org/>.

There are other implementations of this api. This one aims to be as simple
as possible without adding any extra features and just providing the bare
minimum of the recommendations from the CIP.

## Deployment

This code currently runs on an ec2 instance in the "Ungoverned IO Account" (643981526071)
on an EC2 instance. The AWS Infrastructure is created using this cdktf project:
[ops-deploy-offchain-metadata-server](https://github.com/cf-ops-team/ops-deploy-offchain-metadata-server)

If you have changed the code, create a new Docker image from it and upload
it to ECR using make. See the Makefile.


* Run `make build` to build the image and tag it with the ecr tag
* Before being able to push to ecr, you need to login first. Use `make login`
    for that but you will need to have your cli logged in to aws using
    `aws sso login` first
* Run `make push` to push the new image to ecr.

The service definition on the instance will download the new image on start.

## Implementation

Written in rust using the actix web framework. THe layout of the project is
taken from the book Zero2Production in Rust. I had no idea of how to set
this up so i took what others have said is a good idea. The main.rs offers the
entry point for the app when run directly while lib.rs implements the actual
"run" that is invoked from main. The reason to that is so that tests can actually
run requests against a real instance of the app, see tests/health_check.rs.

The handlers for the various views are all implemented in api.rs.