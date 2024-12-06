#!/bin/bash

# echo $$
# test1 run source test.sh or . test.sh or exec test.sh
# test2 run bash test.sh or ./test.sh

echo "Sub script PID: $$"

# Change directory to demonstrate the impact
cd /tmp || exit
