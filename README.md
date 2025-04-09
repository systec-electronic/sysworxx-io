<!--toc:start-->
- [sysworxx-io](#sysworxx-io)
  - [Development Dependencies](#development-dependencies)
  - [libsysworxx-io](#libsysworxx-io)
    - [Cross compile the shared library and I/O daemon using cross](#cross-compile-the-shared-library-and-io-daemon-using-cross)
    - [Debugging](#debugging)
    - [Generate C headers](#generate-c-headers)
  - [Install cbindgen](#install-cbindgen)
  - [Generate C-API header](#generate-c-api-header)
  - [Language Bingings](#language-bingings)
    - [C\#](#c)
  - [Running CODESYS connector](#running-codesys-connector)
    - [Device setup](#device-setup)
<!--toc:end-->

# sysworxx-io

This workspace contains:

- crate to build `libsysworxx-io` and `iodaemon`
  - systemd services for `iodaemon`
- compatibility layer for legacy `ctr700drv`

## Development Dependencies

- Rust Version 1.75
- Install *Docker* and *cross* for generic cross compile
- Install *Yocto SDK* for platform/version specific cross compile
- Install *libiio* and *libsensor* for native build (compile test)
- Install `cargo deb` to build Debian package
- C# Bindings: .Net core SDK
- Node.js Bindings: Node.js and npm

## libsysworxx-io

Build the shared library and I/O daemon:

~~~sh
cargo build --release
~~~

### Cross compile the shared library and I/O daemon using cross

To use cross, one has to first install and start the docker engine and then
install cross on the system.

~~~sh
cargo install cross --git https://github.com/cross-rs/cross
~~~

Then build for a target system:

~~~sh
# for 32-bit ARM
cross build --release --target armv7-unknown-linux-gnueabihf --workspace
# for 64-bit ARM
cross build --release --target aarch64-unknown-linux-gnu --workspace
~~~

### Debugging

To debug the library, the following command must be used in the terminal:
`export IO_LOG=debug`.

### Generate C headers

## Install cbindgen

~~~sh
cargo install cbindgen
~~~

## Generate C-API header

~~~sh
cbindgen --config cbindgen.toml --output <DESTINATION_PATH>/sysworxx_io.h
~~~

## Language Bingings

### C\#

Example applications can be build with following commands:

```sh
cd ./Bindings/CSharp/Demo
dotnet publish -r linux-arm64 --self-contained --configuration Release
# deploy binaries to the target, e.g.
scp -r bin/Release/net8.0/linux-arm64/publish root@sysworxx:/tmp
```

On the target device the executable will be located in `/tmp/publish/Demo`.

For `Runlight` example use analogous commands.

## Running CODESYS connector

### Device setup

- Install packages for "CODESYS Virtual Control for Linux ARM64 SL" in IDE
- Tools - Control SL ausrollen
  - Kommunikation: SSH Login on the device
  - Bereitstellung:
    - Produkt: CODESYS Virtual Control for Linux SL
    - Version: 4.13.0 (arm64) - or newer
    - Click Installieren
  - Operation:
    - In window VPLCs click on plus symbol, select image and give any name
    - Click in window VPLCs on the image - settings window opens to the right
    - Enter following:
      - Mounts: `/var/opt/codesysvcontrol/instances/CODESYS/conf/codesyscontrol:/conf/codesyscontrol/, /var/opt/codesysvcontrol/instances/CODESYS/data/codesyscontrol:/data/codesyscontrol/,  /var/run/codesysextension/extfuncs/: /var/run/codesysextension/extfuncs/`
      - Ports: `11740:11740`
