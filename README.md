# php-rust
Library to build PHP extensions with Rust.
**It is not yet ready for production**.

Run example: 

```
$ ./setup_ext.sh

$ ./build_and_deploy_example.sh return_string
$ php -r 'echo hello_from_rust() . "\n";'

Hello from Rust!
```

