#!/bin/bash

set -e

pushd output

rm -f htpasswd

# Docker Registry requires bcrypt; does not support the insecure md5 default.
htpasswd -bcB htpasswd foo bar

popd