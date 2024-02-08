## uts-enable

Test project trying to enable GPIO pins for UTS Matilda payload on WS-1 cubesat.

## Cross-compiling for BeagleBone

This project is set up to compile for Mac and for the Beaglebone Black (BBB). To build the code for all supported
platforms, just run `cargo build`.

This will fail until you have correctly set up the cross-compiler for the BBB.

If you want to build just for your current platform, pass the platform "triple" as the target for cargo.
For example, on Mac:

```sh
$ cargo build --target aarch64-apple-darwin
```

You can also edit the default target platforms in `.cargo/config.toml`.

### On Mac

To set up cross-compiling for BBB:

* Add the target for your Rust environment with:
  `rustup target add arm-unknown-linux-gnueabihf`
* Install the ARM Linux cross-compiler `arm-unknown-linux-gnueabihf` from
  [osx-arm-linux-toolchains](https://github.com/thinkski/osx-arm-linux-toolchains), and symlink
  the `bin/arm-unknown-linux-gnueabihf-gcc` command to your PATH as `arm-linux-gcc`. (It needs to match the
  linker configuration in `.cargo/config.toml`.)
* Install the BBB toolchain from [Cube-OS toolchains](https://github.com/Cube-OS/toolchains/), and add its absolute
  path to the BBB_TOOLCHAIN in your environment. (The relevant library path is added for the linker by `build.rs`,
  which will fail if something is wrong here.)

### On Linux

* Add the target for your Rust environment with:
  `rustup target add arm-unknown-linux-gnueabihf`
* Install the BBB toolchain from [Cube-OS toolchains](https://github.com/Cube-OS/toolchains/), and add its absolute
  path to the BBB_TOOLCHAIN in your environment. (The relevant library path is added for the linker by `build.rs`,
  which will fail if something is wrong here.)
* Either add `$BBB_TOOLCHAIN/usr/bin/` to your PATH, or symlink `$BBB_TOOLCHAIN/usr/bin/arm-linux-gcc` to a directory
  on your PATH, like `~/bin`. This will be used as the linker, as configured in `.cargo/config.toml`.

### With Docker

The CUAVA team have provided a Docker configuration which can be used for compiling for both the
satellite primary computer and the payload computer.

You can get it from [Cube-OS/cubeos-dev](https://github.com/Cube-OS/cubeos-dev).

To get it working on Mac, you need to:

* Edit the dockerfile to disable the 32-bit library installation
* Build the image: `docker build -t cubeos-dev .`
* Connect your SSH agent when starting the container like this:

```sh
docker run -it -v "$PWD":/usr/cubeos \
    -v /run/host-services/ssh-auth.sock:/run/host-services/ssh-auth.sock:ro \
    -e SSH_AUTH_SOCK="/run/host-services/ssh-auth.sock" \
    -w /usr/cubeos cubeos-dev bash
```
