# htop

[![Crates.io][crates-badge]][crates-url]
![Code coverage][coverage-badge]
![build Linux][build-badge-linux]
![build Windows][build-badge-windows]
![build MacOs][build-badge-macos]
[![MIT licensed][mit-badge]][mit-url]
[![Apache 2.0 licensed][apache-badge]][apache-url]
[![Contributor Covenant][coc-badge]](https://github.com/wisbery/htop/blob/main/CODE_OF_CONDUCT.md)

[crates-badge]: https://img.shields.io/crates/v/htop.svg
[crates-url]: https://crates.io/crates/htop
[mit-badge]: https://img.shields.io/badge/License-MIT-blue.svg
[mit-url]: https://github.com/wisbery/htop/blob/main/LICENSE-MIT
[apache-badge]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[apache-url]: https://github.com/wisbery/htop/blob/main/LICENSE-APACHE
[build-badge-linux]: https://github.com/wisbery/htop/actions/workflows/build-linux.yml/badge.svg
[build-badge-windows]: https://github.com/wisbery/htop/actions/workflows/build-windows.yml/badge.svg
[build-badge-macos]: https://github.com/wisbery/htop/actions/workflows/build-macos.yml/badge.svg
[coverage-badge]: https://img.shields.io/badge/Code%20coverage-100%25-green.svg
[coc-badge]: https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg

## Overview

HTML to PDF converter based on [headless_chrome](https://crates.io/crates/headless_chrome) crate.
Inspired by [html2pdf](https://crates.io/crates/html2pdf) crate.

## Installation

```
$ cargo install htop
```

## Usage

Display short usage description:

```
$ htop -h
```

Display detailed usage description:

```
$ htop --help
```

## Basic examples

Convert single HTML file into single PDF file:

```
$ htop single input_file.html output_file.pdf
```

Convert multiple HTML files into multiple PDF files:

```
$ htop multiple input_dir output_dir
```

Convert web-page into single PDF file:

```
$ htop url https://dmntk.io
```

More examples can be found in [User Guide](https://github.com/wisbery/htop/blob/main/user_guide/README.md).

## License

Licensed under either of

- [MIT license](https://opensource.org/licenses/MIT) ([LICENSE-MIT](https://github.com/wisbery/htop/blob/main/LICENSE-MIT)), or
- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) ([LICENSE-APACHE](https://github.com/wisbery/htop/blob/main/LICENSE-APACHE))

at your option.

## Contribution

All contributions intentionally submitted for inclusion in the work by you,
shall be dual licensed as above, without any additional terms or conditions.
