# htop

[![Crates.io][crates-badge]][crates-url]
![Code coverage][coverage-badge]
![build Linux][build-badge-linux]
![build Windows][build-badge-windows]
![build MacOs][build-badge-macos]
[![MIT licensed][mit-badge]][mit-license-url]
[![Apache 2.0 licensed][apache-badge]][apache-license-url]
[![Contributor Covenant][cc-badge]][cc-url]

[crates-badge]: https://img.shields.io/crates/v/htop.svg
[crates-url]: https://crates.io/crates/htop
[mit-badge]: https://img.shields.io/badge/License-MIT-blue.svg
[mit-url]: https://opensource.org/licenses/MIT
[mit-license-url]: https://github.com/EngosSoftware/htop/blob/main/LICENSE-MIT
[apache-badge]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[apache-url]: https://www.apache.org/licenses/LICENSE-2.0
[apache-license-url]: https://github.com/EngosSoftware/htop/blob/main/LICENSE
[apache-notice-url]: https://github.com/EngosSoftware/htop/blob/main/NOTICE
[build-badge-linux]: https://github.com/EngosSoftware/htop/actions/workflows/build-linux.yml/badge.svg
[build-badge-windows]: https://github.com/EngosSoftware/htop/actions/workflows/build-windows.yml/badge.svg
[build-badge-macos]: https://github.com/EngosSoftware/htop/actions/workflows/build-macos.yml/badge.svg
[coverage-badge]: https://img.shields.io/badge/Code%20coverage-100%25-green.svg
[cc-badge]: https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg
[cc-url]: https://github.com/EngosSoftware/htop/blob/main/CODE_OF_CONDUCT.md

## Overview

HTML to PDF converter based on [headless_chrome](https://crates.io/crates/headless_chrome) crate.
Inspired by [html2pdf](https://crates.io/crates/html2pdf) crate.

## Installation

```
cargo install htop
```

## Usage

Display short usage description:

```
htop -h
```

Display detailed usage description:

```
htop --help
```

## Basic examples

Convert single HTML file into single PDF file:

```
htop single input_file.html output_file.pdf
```

Convert multiple HTML files into multiple PDF files:

```
htop multiple input_dir output_dir
```

Convert web-page into single PDF file:

```
htop url https://dmntk.io
```

More examples can be found in [user guide](https://github.com/EngosSoftware/htop/blob/main/user_guide/README.md).

## License

Licensed under either of

- [MIT license][mit-url] (see [LICENSE-MIT][mit-license-url]) or
- [Apache License, Version 2.0][apache-url] (see [LICENSE][apache-license-url] and [NOTICE][apache-notice-url])

at your option.

## Contribution

All contributions intentionally submitted for inclusion in the work by you,
shall be dual licensed as above, without any additional terms or conditions.
