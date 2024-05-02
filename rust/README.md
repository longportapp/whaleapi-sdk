# LongPort OpenAPI SDK for Rust

<div align="center">
  <a href="https://crates.io/crates/longport">
    <img src="https://img.shields.io/crates/v/longport.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <a href="https://docs.rs/longport">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
  <a href="https://github.com/rust-secure-code/safety-dance/">
    <img src="https://img.shields.io/badge/unsafe-forbidden-success.svg?style=flat-square"
      alt="Unsafe Rust forbidden" />
  </a>
  <a href="https://blog.rust-lang.org/2021/11/01/Rust-1.56.1.html">
    <img src="https://img.shields.io/badge/rustc-1.56.1+-ab6000.svg"
      alt="rustc 1.56.1+" />
  </a>
</div>


`LongportWhale` provides an easy-to-use interface for invokes [`LongPort Whale OpenAPI`]
## Quickstart

_Add dependencies to `Cargo.toml`_

```toml
[dependencies]
longportwhale = "1.0.0"
```

_Setting environment variables(MacOS/Linux)_

```bash
export LONGPORT_APP_KEY="App Key get from user center"
export LONGPORT_APP_SECRET="App Secret get from user center"
export LONGPORT_ACCESS_TOKEN="Access Token get from user center"
```

_Setting environment variables(Windows)_

```powershell
setx LONGPORT_APP_KEY "App Key get from user center"
setx LONGPORT_APP_SECRET "App Secret get from user center"
setx LONGPORT_ACCESS_TOKEN "Access Token get from user center"
```

## Crate features

To avoid compiling unused dependencies, longport gates certain features, all of which are disabled by default:

| Feature  | Description                         |
|----------|-------------------------------------|
| blocking | Provides the `blocking` client API. |

## License

Licensed under either of

* Apache License, Version 2.0,([LICENSE-APACHE](./LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](./LICENSE-MIT) or <http://opensource.org/licenses/MIT>) at your option.
