#!/bin/bash

#rm -rfd target
# Use a subshell so that we don't modify the user environment-variables
#cargo test --no-run

for TEST in $(find ./target/debug/deps -executable -type f | grep test) 
do 
    mkdir -p ./target/debug/cov/$TEST 
    kcov --exclude-pattern=/.rustup,/.cargo ./target/debug/cov/$(basename $TEST) $TEST ;\
done; 

kcov --merge ./target/debug/cov/total $(find ./target/debug/cov/ -type d -maxdepth 1 -mindepth 1) 

$(OPEN_COMMAND) ./target/debug/cov/total/index.html > /dev/null &