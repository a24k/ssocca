[![Check and Test](https://github.com/a24k/ssocca/actions/workflows/ct.yml/badge.svg)](https://github.com/a24k/ssocca/actions/workflows/ct.yml)

# ssocca - SSO Credential Cookie Acquirer

Credential Cookie Acquirer for SSO Authentication Flow works with Google Chrome.

## Usage

```
SSO Credential Cookie Acquirer

Usage: ssocca [OPTIONS] [TOML]

Arguments:
  [TOML]  Specify path to a configuration file

Options:
  -l, --headless       Use browser in headless mode
  -v, --verbose...     More output per occurrence
  -q, --quiet...       Less output per occurrence
      --chrome <PATH>  Specify path to a Chrome executable
      --timeout <SEC>  Timeout duration in secs [default: 10]
      --cookie <NAME>  Cookie name to acquire
      --url <URL>      Url to initiate authentication
  -h, --help           Print help
  -V, --version        Print version
```
