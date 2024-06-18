# wasmi-demo

This is a proof of concept of a Unikernel to run WebAssembly code.
It is derived from the blog post [SUSE Hackweek 2023](https://hackweek.opensuse.org/22/projects/build-a-unikernel-that-runs-webassembly).

## The demo application

The demo calls a WebAssembly function to calculate a fibonacci number.
The source code of the wasm module is published in the directory `examples\fib`.

## Requirements

* [rustup](https://www.rust-lang.org/tools/install)
* [QEMU](https://www.qemu.org/)

## Usage

### Build the unikernel

The unikernel can be built using the following Makefile target:

```console
make build
```

### Run the application

```console
make run
```

> **Note:** this demo has been tested only on a Linux host.
