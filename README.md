# Cardano off-chain metadata API

A simple implementation of the metadata server proposed in
[CIP-26](https://cips.cardano.org/cips/cip26/#recommendationsformetadataservers) in Rust.

## Abstract

[CIP-26](https://cips.cardano.org/cips/cip26/) proposes a standard for off-chain
metadata management. However it does not provide details on the actual implementation.

Currently the metadata is stored in a github repository where new entries can be
made via pull requests. One for [mainnet](https://github.com/cardano-foundation/cardano-token-registry) and one for the [testnets](https://github.com/input-output-hk/metadata-registry-testnet). While everyone can easily just grab the git repo
and work with the data as needed, a public API provides (most) of the endpoints
for even easier consumption. That api for mainnet(!) is located here: <https://tokens.cardano.org/>.
I am not aware of a public service for the testnets.

It only implements the endpoints to retrieve token metadata and
intenionally does not provide endpoints to change the data. That is to be done
through the github.com repository located [here](https://github.com/cardano-foundation/cardano-token-registry)
for cardano mainnet.

## Implementation


### /metadata/{subject}/properties

The CIP suggests to return a json list of strings of the names of the given
subject. However neither the IOG nor the CF implementation acutlly return
that. IOGs simply returns the same as the call to /metadata/{subject} would
and CFs version does not implement that endpooint at all

## Other Implementations

### IO

API Url <>

### Java

APIUrl:  <https://api.metadata.staging.cf-deployments.org/>

APIDocs: <https://api.metadata.staging.cf-deployments.org/apidocs>

<https://api.metadata.staging.cf-deployments.org/v2/mainnet/subjects/00109530994ea381c0bfe0936c85ea01bfe2765c24ef6dad5740c33e486f646c657220436f616c6974696f6e20436f696e>
