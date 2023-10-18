#!/usr/bin/env bash

CURRENT_DIR=$(pwd)

PATH=$CURRENT_DIR/../target/debug:$PATH ./run.sh "$@"
