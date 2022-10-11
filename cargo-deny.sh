#!/bin/bash

set -e

cargo deny check
cargo deny check sources

