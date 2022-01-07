# libewon 

`libewon` is a wrapper around the [M2Web API](https://developer.ewon.biz/content/m2web-api-0) of the [Talk2M](https://www.ewon.biz/products/talk2m) industrial cloud.

## Project status

The project is currently in **alpha** state and cannot be considered stable. Breaking changes could happen at any time, use it at your own risk! Nevertheless, it is usable and used internally at [SMAG Graphique](https://www.smag-graphique.com).

<div align="center">
  <a href="https://www.smag-graphique.com"><img alt="SMAG Graphique" src="https://www.smag-graphique.com/build/assets/images/img-logo.2bb13ee3.svg"></a>

  Made with :heart: by [SMAG Graphique](https://www.smag-graphique.com).
</div>

## Installation

The `libewon` is not yet available from crates.io. It will be published once at least all API endpoints have been wrapped around. Currently, to add `libewon` as a dependency, add the following line to `Cargo.toml`:

```toml
libewon = {git = "https://gitlab.com/francois-de-mareschal/libewon", branch = "develop"}
```

:warning: Follow the `develop` branch!

## Documentation

### Examples

#### Get the list of all registered eWONs

```rust
use libewon::m2web::{client::ClientBuilder, ewon::Ewon};

let client = ClientBuilder::default()
    .t2m_account("account2")
    .t2m_username("username2")
    .t2m_password("password2")
    .t2m_developer_id("795f1844-2f5e-4d8b-9922-25c45d3e1c47")
    .build()?;
// Call API to get all eWONs registered for this corporate account.
let ewons: Vec<Ewon> = client.get_ewons(None).await?;
```

#### Get the list of the registered eWONs belonging to a specific pool

```rust
use libewon::m2web::{client::ClientBuilder, ewon::Ewon};

let client = ClientBuilder::default()
    .t2m_account("account2")
    .t2m_username("username2")
    .t2m_password("password2")
    .t2m_developer_id("795f1844-2f5e-4d8b-9922-25c45d3e1c47")
    .build()?;
// Call API to get all eWONs of a specific pool for this corporate account.
let ewons_emea_pool: Vec<Ewon> = client.get_ewons(Some("emea")).await?;
```

#### Get a specific eWON

```rust
use libewon::m2web::{client::ClientBuilder, ewon::Ewon};

let client = ClientBuilder::default()
    .t2m_account("account2")
    .t2m_username("username2")
    .t2m_password("password2")
    .t2m_developer_id("795f1844-2f5e-4d8b-9922-25c45d3e1c47")
    .build()?;
// Call API to get an eWON by id.
let ewon_4242: Ewon = client.get_ewon_by_id(4242).await?;
let ewon_ewon42: Ewon = client.get_ewon_by_name("ewon42").await?;
```

#### :warning: **LEGACY - DO NOT USE**: stateful login/logout

```rust
use libewon::m2web::client::ClientBuilder;

let mut client = ClientBuilder::default()
    .t2m_account("account2")
    .t2m_username("username2")
    .t2m_password("password2")
    .t2m_developer_id("795f1844-2f5e-4d8b-9922-25c45d3e1c47")
    .stateful_auth(true)
    .build()?;

let _ = client.login().await?;
// Do something useful, then:
client.logout().await?; // client is consumed by logout().
```

### Integrated documentation

The documentation is integrated to the repository, thanks to the built-in documentation feature. To open it, go inside the repository and type:

```sh
cargo doc --open
```

This will open your default browser at the crate documentation page.

## License

    Licensed under the EUPL
