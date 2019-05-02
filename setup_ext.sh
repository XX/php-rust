#!/usr/bin/env bash

echo "extension=rust_example_ext.so" | sudo tee /etc/php/7.0/mods-available/rust_example_ext.ini > /dev/null &&
sudo ln -s /etc/php/7.0/mods-available/rust_example_ext.ini /etc/php/7.0/cli/conf.d/30-rust_example_ext.ini