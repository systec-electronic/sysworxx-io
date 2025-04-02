# Development Setup

Install rustup, the stable compiler (e.g. 1.4.0) and install the toolchain.

Rustup can be installed by following the instructions here
[rustup.rs](https://rustup.rs/). Some Linux distributions also allow
installation via the default package manager.

The following commands are used to install the stable compiler and ARM
toolchain.

~~~sh
rustup toolchain install stable
rustup target add armv7-unknown-linux-gnueabihf
~~~

## Setup for Poky GCC-Linker

The linker settings are derived from the settings of the Poky toolchain.
See `.cargo/config`

To be able to use it, one has to source the Poky SDK environment. With a
default installation this will look like the following.

~~~sh
/opt/poky/2.5.3/environment-setup-cortexa7hf-neon-poky-linux-gnueabi
~~~

Hint: This is also where the settings in `.cargo/config` are derived from. Execute
`echo $CC` to get the settings used.

## Build the shared library and I/O daemon

The feature `shm` enables building the `iodaemon`, which can handle setting and
getting of different I/Os in a separate process.

~~~sh
cargo build --release --target armv7-unknown-linux-gnueabihf
cargo build --release --target armv7-unknown-linux-gnueabihf
~~~

## Build the shared library with docker

To build the library in a docker container, the sysworxxIO directory and the
rootfs_dev have to be linked with the container. A working rootfs_dev can be
found here:
[](http://srv-gitlab.intern.systec-electronic.com/121903/debian-var/-/pipelines)

~~~sh
docker run --rm -it -v PATH_TO_SYSWORXX_IO:/usr/src/sysworxx-io -v PATH_TO_ROOTFS_DEV:/usr/src/rootfs debian-builder
~~~

Inside the container, switch to the sysworxx-io directory and build the library.

~~~sh
cargo build --target armv7-unknown-linux-gnueabihf
cargo build --workspace --target armv7-unknown-linux-gnueabihf
~~~

The first build has to be without the ```--workspace``` parameter. This builds
the necessary files, which are needed for the complete build with this
parameter.
All supported targets are build into the *.so files.
The build *.so files are inside the folder `~/sysworxx-io/target/armv7-unknown-linux-gnueabihf/debug/`.

### Build examples with docker

~~~sh
cargo build --example demo --target armv7-unknown-linux-gnueabihf
~~~

## Build the shared library with cross
To use cross, one has to first install and start the docker engine and then
install cross on the system.
~~~sh
cargo install cross --git https://github.com/cross-rs/cross
~~~

Then build for a target system:
~~~sh
cross build --release --target ${TARGET}
cross build --release --target ${TARGET} --workspace
~~~

The placeholder ${TARGET} has to be replaced with the desired architecture
which the library has to be build for. E.g. "aarch64-unknown-linux-gnu"
or "armv7-unknown-linux-gnueabihf". Cross will download any dependencies.

## Build sysworxx-io-js

For this, the docker container needs a new user without root rights. This user
also needs to get rust installed separately, as well as nodejs and npm.

## Debugging the library

To debug the library, the following command must be used in the terminal:
`export IO_LOG=debug`.

Calling the application with `RUST_BACKTRACE=full`,
e.g. `RUST_BACKTRACE=full ./test` will add further debug information.

# Generate C headers

## Install cbindgen

~~~sh
cargo install cbindgen
~~~

## Generate C-API header

~~~sh
cbindgen --config cbindgen.toml --output <DESTINATION_PATH>/sysworxx_io.h
~~~

## Generate bitbake recipe

~~~sh
cargo install cargo-bitbake
cargo bitbake
sed -i \
    -e 's/git@//' \
    -e 's/protocol=ssh/protocol=http/' \
    -e 's/\${WORKDIR}\/git/${WORKDIR}\/git\/sysworxx-io/' sysworxx-io_0.1.0.bb
~~~

The sed command is used to:

- be able to access the internal gitlab server
- respect the fact that the actual crate source lives in the subdirectory
  sysworxx-io and not in the root directory
  TODO: This may be changed

The must be used together with the file `sysworxx-io.inc`. This has to contain the
target feature which chooses the device the crate is build for. The content of
the file should look like the following.

~~~sh
CARGO_BUILD_FLAGS += "--features def_<DEVICE>"
~~~

## TODO

- use Ini::section() and pass Properties Object for calibration data
- maybe: Use std::panic::set_hook() to not print any text in case of an panic,
  but this leaves the question, how we handle the error information anyway

## Java Bindings to the board driver for CTR-700

To build the driver library and the demo application, perform the following steps:

~~~sh
cd Project
make java
~~~

To run the demo application, perform the following steps:

~~~sh
cd ../Bindings/Java/
./run_demo.sh
~~~

This will run a simple CLI application, which runs several demo operations on
the I/O's of the CTR-700. The Demo application can be terminated by pressing
CTRL-D.

The jar-file created in the first step is created by using two Java files. The
file Ctr700Drv.java contains a class, which uses JNA to access the native driver
library (libctr700drv.so). This file is called Ctr700Drv.java. The second file
Demo.java contains some application code, which shows the usage of the driver
class contained in the first Java file.

To be able to use the driver class in other projects one needs to:

- add the Java file Ctr700Drv.java to the project
- add the jna.jar dependency to the project
- set the jna.library.path property when running the application

(See run_demo.sh on how this can be accomplished)

### Java Script Bindings to the board driver for CTR-700

- Install requirements (needed to build node.js packages):

~~~sh
apt install -y build-essential
~~~

- Install node.js FFI packages

~~~sh
cd <PATH_TO_DEMO>
npm install
~~~

  Hint: 'npm install' does not work on NFS-mounted directories!

- Run the demo:

~~~sh
cd <PATH_TO_DEMO>
LD_LIBRARY_PATH=. node demo.js
~~~

TODO: Implement counter functions for other language bindings

```sh
clear \
  && cargo build --release --target armv7-unknown-linux-gnueabihf \
  && clear \
  && cargo build --release --workspace --target armv7-unknown-linux-gnueabihf \
  && clear \
  && cargo deb -v --no-build --variant ctr-700 --target armv7-unknown-linux-gnueabihf --output sysworxx-io-ctr-700.deb
```
