# ScrambleFlow Demo

An extension based on the work if the BMBF ATLAS project.

## Prejudices

Before one can run the demo, [rust](https://www.rust-lang.org/tools/install) and [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) need to be installed. This can be done by running the following commands or by following the instructions from the links. You may need to restart the shell in between the commands.
```shell
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

## Running the Demo

First change the working directory to `scrambledb`
```shell
$ cd atlas-spec/scrambledb/
```

At the `scrambledb` crate root, run the following to build the WASM
version of the project in `./pkg`:

```shell
$ wasm-pack build --target web --features wasm
```

To serve the project locally, copy `index.html` to the right directory
and start e.g. a simple Python web server there:

```shell
$ cp ./wasm_demo/index.html ./pkg/
$ cd ./pkg
$ python3 -m http.server
// Windows alternative: python -m http.server
```