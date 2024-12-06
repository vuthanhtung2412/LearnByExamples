#!/bin/bash

set -e # try disable this

parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
cd $parent_path

./s1.sh 
# or 
# . s1.sh

echo success
