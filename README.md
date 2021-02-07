# Agora [![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](https://www.reddit.com/r/opensource/comments/81n4e2/dual_licensing_with_mit_and_apache/)

**Very much a work in progress**

> The agora (/ˈæɡərə/; Ancient Greek: ἀγορά agorá) was a central public space in ancient Greek city-states. It is the best representation of city form’s response to accommodate the social and political order of the polis. The literal meaning of the word is "gathering place" or "assembly". The agora was the center of the athletic, artistic, spiritual and political life in the city. The Ancient Agora of Athens is the best-known example.
>
> -- Wikipedia

The main goals of Agora is:

* To be easy to install
* To be performant
* (Eventually) to work with everyones favorite database
* (Also eventually) have all the features everyone might need

## Easy to install

This project started from having a difficult time installing other forums due to complicated install (and imperfect) install scripts, forums that can't be easily installed via docker, forums that can't be easily installed natively because "Hosting Rails applications is complicated".

It was also in part inspired from [lemmy](https://github.com/LemmyNet/lemmy), a federated reddit alternative, because it was so easy to install via docker and natively. They also have an install method for kubernetes though I haven't tried that one out.

## Performance

My first and favorite programming language is [**Rust**](https://www.rust-lang.org/) so obviously I had to write Agora in rust. Agora is built on:

* [Actix](https://actix.rs/)
* [SQLX](https://github.com/launchbadge/sqlx)
* [Seed](https://seed-rs.org/)

Agora should perform well on cheap VPSs.

## Contributing

Want to join us? Take a look at some of these issues:

* [Issues labeled "good first issue"][good-first-issue]
* [Issues labeled "help wanted"][help-wanted]

## Design Notes 

### Frontends

Will use a Web Assembly(wasm) and javascript frontend until I decide which I prefer. Wasm can be written in rust,
allowing for an isomorphic SPA that can share code between the frontend and backend. This should be easier to write and
maintain. However right now wasm interaction with the DOM is expensive, compiled wasm can be quite large(even with
optimizations), and the ecosystem for wasm frontends is quite new. Javascript does not share these issues. Solid-js is
the current javascript performance king in this domain, trading blows with hand optimized vanillajs, and may provide an
overall better user experience and will integrate with current web design tools.

* [Seed](https://github.com/seed-rs/seed) (Web Assembly) 
* [Solid](https://github.com/ryansolid/solid) (Javascript)

#### CSS

Leaning towards [tailwindcss](https://github.com/tailwindlabs/tailwindcss) with
[linaria](https://github.com/callstack/linaria) to write css in javscript with no runtime cost. Tailwinds is designed to
be used with PostCSS to create the smallest possible css for application.

My plan is to: 

1. Design the wasm fronted with a shared library with backed 
2. Design the javscript frontend as similar as possible to wasm 
3. Write CSS in the javascript frontend(hopefully js and wasm are similar enough that the css works for both
   frontends)

### Backend 

- [Actix](https://github.com/actix/actix-web) 
- [Diesel](https://github.com/diesel-rs/diesel) or [sqlx](https://github.com/launchbadge/sqlx) (undecided)
- [Argon2](https://github.com/RustCrypto/password-hashes/tree/master/argon2)

### Conduct

The Tide project adheres to the [Contributor Covenant Code of
Conduct](https://github.com/http-rs/tide/blob/master/.github/CODE_OF_CONDUCT.md).
This describes the minimum behavior expected from all contributors.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
