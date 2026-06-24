# Yōso Project （要素Ｐｒｏｊｅｃｔ）

[![Cargo Build & Test](https://github.com/Anatoly03/Yooso/actions/workflows/cargo.yml/badge.svg)](https://github.com/Anatoly03/Yooso/actions/workflows/cargo.yml)
[![Rust Documentation](https://badges.ws/badge?icon=rust&value=Documentation)](https://anatoly03.github.io/Yooso/yooso/index.html)

Yōso is a backend platform inspired by ECS data design. The data is structured as flexible entities with reusable components rather than tabular or schematic presets.

<p align="center">
  <img src="https://anatoly03.github.io/Yooso/preview/yooso-studio.png" />
</p>

### Running

```
cargo run
```

To start the development backend, execute `cargo run`. In a separate console, run `npm run dev` to start the front end server. Afterwards you can access the admin UI at `http://localhost:8080/`.

### Documentation

To build the documentation locally, run the following command.

```sh
cargo doc --no-deps --workspace
```

### Generating the OpenAPI file

When compiling Rust, the openapi files are automatically fragmentedly generated in the respective directories.
To compile a single OpenAPI file, we provide a script to merge openapi data. An example can be seen below.
The `npm run openapi-merge` command takes a list of openapi scripts and merges them intelligently.

```sh
npm run openapi-merge yooso-api/openapi/openapi.json yooso-storage/openapi/openapi.json
```
