# sysworxx-io-js

This project was bootstrapped by [create-neon](https://www.npmjs.com/package/create-neon).

## Building and Installing sysworxx-io-js

Installing sysworxx-io-js requires a [supported version of Node and Rust](https://github.com/neon-bindings/neon#platform-support).

`npm run build-target` uses the
[cargo-cp-artifact](https://github.com/neon-bindings/cargo-cp-artifact)
utility to run the Rust build and copy the built library into `./index.node`.

## Exploring sysworxx-io-js

After building sysworxx-io-js, you can explore its exports at the Node REPL:

```sh
node
> require('.').get_hw_info()
```

## Learn More

* [Neon documentation](https://neon-bindings.com).
* [Rust documentation](https://www.rust-lang.org).
* [Node documentation](https://nodejs.org).
