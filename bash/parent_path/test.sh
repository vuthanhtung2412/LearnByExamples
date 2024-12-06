#!/bin/bash

set -e # try disable this

parent_path=$(
  cd "$(dirname "${BASH_SOURCE[0]}")"
  pwd -P
)
cd $parent_path

./inner.sh
# or
# . inner.sh
