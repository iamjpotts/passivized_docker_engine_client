#!/bin/bash

set -e

pushd output

../cfssl gencert \
    -ca ca.pem \
    -ca-key ca-key.pem \
    -config ../profiles.json \
    -profile=server \
    ../testregistry.locallan.json \
    | ../cfssljson -bare testregistry.locallan

cat testregistry.locallan.pem ca.pem > testregistry.locallan.crt

popd