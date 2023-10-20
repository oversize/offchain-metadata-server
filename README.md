# Cardano off-chain metadata API

A simple implementation of the offchain metadata server proposed in
[CIP-26](https://cips.cardano.org/cips/cip26/) in Rust.

## Abstract

The metadata registry is a github repository that holds a bunch of json files.
See <https://github.com/cardano-foundation/cardano-token-registry> for more details.

The endpoint that servers these is located here <https://tokens.cardano.org/>.
There are various implementations of this:

* <https://github.com/input-output-hk/offchain-metadata-tools>
* <https://github.com/cardano-foundation/cf-metadata-server>

## Motivation

Why another?

Existing implementations are complicated and/or implement features the CIP did not
ask for. Also, they are somewhat difficult to operate because they are made up
of different services.

My goal here is to have a super simple implementation that is easy to operate and
scale. The metadata registry is a bunch of json files. These files are read into
memory and served. The only external service there currently is, is something
that automatically pulls the changes from the registry. In my current deployment
i have a systemd timer running every X hours that runs a script roughly like this:

```bash
#!/bin/sh
readonly registry_path="/path/to/cardano-token-registry"
# If the repository does not exist, clone it and exit
if [ ! -d $registry_path ]; then
    mkdir -p $registry_path
    git clone --depth 1 https://github.com/cardano-foundation/cardano-token-registry.git $registry_path
    exit 0
fi
# If it does exist, pull the contents and trigger reread
if [ -d $registry_path ]; then
    cd $registry_path || exit 1
    git pull
    curl http://localhost:8080/reread
fi
```

Idealy this would be part of the api at some point.

## Build & run

Build it using cargo. To run it you need to provide the path to the mappings
folder of the registry via the `MAPPINGS` environment variable.

## Implementation

Written in rust using the actix web framework. THe layout of the project is
taken from the book Zero2Production in Rust. I had no idea of how to set
this up so i took what others have said is a good idea. The main.rs offers the
entry point for the app when run directly while lib.rs implements the actual
"run" that is invoked from main. The reason to that is so that tests can actually
run requests against a real instance of the app, see tests/health_check.rs.

The handlers for the various views are all implemented in api.rs.