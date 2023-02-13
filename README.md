# pain

A Rust library to manage ISO 20022 pain.001 payment initiation messages widely used for corporate to bank payments.

[![Made With Love][made-with-rust]][6]
[![Crates.io][crates-badge]][8]
[![Lib.rs][libs-badge]][10]
[![Docs.rs][docs-badge]][9]
[![License][license-badge]][2]

![divider][divider]

## Welcome to pain 👋

![pain Banner][banner]

<!-- markdownlint-disable MD033 -->
<center>

**[Website][0]
• [Documentation][9]
• [Report Bug][3]
• [Request Feature][3]
• [Contributing Guidelines][4]**

</center>

<!-- markdownlint-enable MD033 -->

## Overview 📖

Payment Initiation (PI) is a library that provides a set of structs
and enums that can be used to serialize and deserialize SEPA payment
information using the `serde` library.

The structs include Payment, OriginalGroupInfoAndStatus, PaymentInfo,
PaymentTypeInfo, ServiceLevel, Debtor, PostalAddress, DebtorAccount,
AccountId, DebtorAgent, FinancialInstitutionId,
CreditTransferTransactionInfo, PaymentId, Amount, InstructedAmount,
Creditor, CreditorAccount, RemittanceInfo, and CreditorAgent.

The enum is PaymentFormat and has four variants: Pain001_001_02,
Pain001_001_03, Pain001_001_04, and Pain001_001_05.

## Features ✨

Coming soon...

## Installation 📦

It takes just a few minutes to get up and running with `pain`.

### Requirements

`pain` requires Rust **1.67.0** or later.

### Documentation

> ℹ️ **Info:** Please check out our [website][0] for more information
and find our documentation on [docs.rs][9], [lib.rs][10] and
[crates.io][8].

## Usage 📖

To use `pain` in your project, add the following to your
`Cargo.toml` file:

```toml
[dependencies]
pain = "0.0.1"
```

Add the following to your `main.rs` file:

```rust
extern crate pain;
use pain::*;
```

then you can use the functions in your application code.

### Examples

`PAIN` comes with a set of examples that you can use to get started. The
examples are located in the `examples` directory of the project. To run
the examples, clone the repository and run the following command in your
terminal from the project root directory.

```shell
cargo run --example pain
```

## Semantic Versioning Policy 🚥

For transparency into our release cycle and in striving to maintain
backward compatibility, `PAIN` follows [semantic versioning][7].

## License 📝

The project is licensed under the terms of both the MIT license and the
Apache License (Version 2.0).

- [Apache License, Version 2.0][1]
- [MIT license][2]

## Contribution 🤝

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms
or conditions.

![divider][divider]

## Acknowledgements 💙

A big thank you to all the awesome contributors of [Mini Functions][6]
for their help and support. A special thank you goes to the
[Rust Reddit](https://www.reddit.com/r/rust/) community for providing a
lot of useful suggestions on how to improve this project.

[0]: https://minifunctions.com/pain
[1]: http://www.apache.org/licenses/LICENSE-2.0
[2]: http://opensource.org/licenses/MIT
[3]: https://github.com/sebastienrousseau/pain/issues
[4]: https://raw.githubusercontent.com/sebastienrousseau/pain/main/.github/CONTRIBUTING.md
[6]: https://github.com/sebastienrousseau/pain/graphs/contributors
[7]: http://semver.org/
[8]: https://crates.io/crates/pain
[9]: https://docs.rs/pain
[10]: https://lib.rs/crates/pain

[banner]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/pain/banners/banner-pain-1597x377.svg "Pain Banner"
[crates-badge]: https://img.shields.io/crates/v/pain.svg?style=for-the-badge 'Crates.io'
[divider]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/elements/divider.svg "divider"
[docs-badge]: https://img.shields.io/docsrs/pain.svg?style=for-the-badge 'Docs.rs'
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.2-orange.svg?style=for-the-badge 'Lib.rs'
[license-badge]: https://img.shields.io/crates/l/pain.svg?style=for-the-badge 'License'
[made-with-rust]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust 'Made With Rust'
