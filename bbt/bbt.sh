#!/usr/bin/env bash

CURRENT_DIR=$(pwd)
PARENT_DIR=$(realpath "$CURRENT_DIR/..")
HTOP_PATH=$PARENT_DIR/target/debug
HTOP_PATH_BINARY=$HTOP_PATH/htop

if [ ! -f "$HTOP_PATH_BINARY" ] ; then
  echo "htop binary not found in path: $HTOP_PATH"
  exit 1
fi

PATH=$HTOP_PATH:$PATH ./run.sh "$@"
