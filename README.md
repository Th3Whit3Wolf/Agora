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

Agora will be an isomorphic SPA forum. It will take advantage of actix's ([high performance server](https://www.techempower.com/benchmarks/#section=data-r19&hw=ph&test=composite)) ability to handle a lot of request and the fact SPA reduce total number of request to hopefully achieve a program that runs well no matter what it's being ran on. I am also betting on wasm advancements making the frontend faster with time.

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
