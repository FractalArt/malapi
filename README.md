# MalAPI

Conveniently interact with the [`Malshare`](https://malshare.com) API from the command-line.

## Usage

Being written in [`Rust`](https://www.rust-lang.org/), the compilation of this project requires a `Rust` compiler.
Instructions on how to install the `Rust` toolchain can be found [here](https://www.rust-lang.org/tools/install).

After that, `malapi` can be compiled through the command:

```sh
> cargo b --release
```

Calling the `Malshare` API requires an API key. To communicate the API key to `malapi` it can be provided on the 
command-line using the flags `--api-key` or `-k` but it is probably more convenient to store it in an environment 
variable according to `MALSHARE_API_KEY=<api-key>`.

To get information on the daily api-call limit and the remaining calls, you can run:

```sh
> ./target/release/malapi --api-limit
2000
> ./target/release/malapi --api-remaining
```

To download a file with known hash, you can type 

```sh
> ./target/release/malapi -d <hash> -o /parent/name.vir
```

If the output file is not specified (or if the parent directory does not exist), the file will be stored in the current directory under the name
`<hash>.vir`.

For more information on the usage of the library run:

```sh
> ./target/release/malapi --help
```

## Documentation

```sh
> cargo doc --no-deps --open
```

## Licensing

`malapi` is made available under the terms of either the MIT License or the Apache License 2.0, at your option.

See the [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) files for license details.
