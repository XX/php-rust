#!/usr/bin/env bash

BUILD_MODE=release
PHP_EXT_DIR=`php -i | grep extension_dir | awk '{print $NF}'`

cargo build --${BUILD_MODE} --example ${1} &&
sudo cp target/${BUILD_MODE}/examples/lib${1}.so ${PHP_EXT_DIR}/rust_example_ext.so