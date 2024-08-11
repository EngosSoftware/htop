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

[htop]: https://github.com/EngosSoftware/htop
[headless_chrome]: https://crates.io/crates/headless_chrome
[html2pdf]: https://crates.io/crates/html2pdf
[report a bug]: https://github.com/EngosSoftware/htop/issues

## Overview

HTML to PDF converter based on [headless_chrome], inspired by [html2pdf].

In case of any problems while using **[htop]**, please see the [Troubleshooting](#Troubleshooting) section or [report a bug].

## Installation

```shell
cargo install htop
```

## Usage

### Display shortened usage description

```shell
htop -h
```

### Display detailed usage description

```shell
htop --help
```

### Display the detailed description of a selected command

```shell
htop help <command>
```

### Convert a single HTML file into a single PDF file

```shell
htop single input_file.html output_file.pdf
```

### Convert multiple HTML files into multiple PDF files

```shell
htop multiple input_directory output_directory
```

### Convert a single web page into a single PDF file

```shell
htop url https://dmntk.io
```

## User guide

A detailed user guide is currently being prepared.

## Troubleshooting

### Printing PDF hangs forever

When **htop** is used in a multiuser environment (or in cloud), it may happen that the printing process hangs forever.
The reason is that in Linux the crash report is created in directory **/tmp/Crashpad**. 
When another user have already used **htop**, then such directory already exists and the access rights
are set only for the other user. [headless_chrome] hangs while trying to get access to this directory.

The simplest workaround is to delete this directory before running **htop**:

```shell
sudo rm -rf /tmp/Crashpad 
```

This might not work when multiple **htop** instances are started simultaneously.

To avoid creating the directory with crash reports, run all simultaneous **htop** instances
with the option `--no-crash-reports` set:

```shell
htop --no-crash-reports url https://dmntk.io
```

### SELinux

It might happen, that SELinux will prevent chrome from using the 'execheap' accesses on a process.

For Fedora Linux, this [bug was reported here](https://bugzilla.redhat.com/show_bug.cgi?id=2254434). 

**VERY INSECURE** workaround is to call:

```shell
sudo setsebool -N selinuxuser_execheap 1
```  

See `man setsebool` for more details.  

## License

Licensed under either of

- [MIT license][mit-url] (see [LICENSE-MIT][mit-license-url]) or
- [Apache License, Version 2.0][apache-url] (see [LICENSE][apache-license-url] and [NOTICE][apache-notice-url])

at your option.

## Contribution

Any contributions to **[htop]** are greatly appreciated.
All contributions intentionally submitted for inclusion in the work by you,
shall be dual licensed as above, without any additional terms or conditions.
