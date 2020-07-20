# rscan [![CircleCI](https://circleci.com/gh/oliverdaff/rscan.svg?style=shield)](https://circleci.com/gh/oliverdaff/rscan) ![GitHub release (latest by date)](https://img.shields.io/github/v/release/oliverdaff/rscan?style=plastic)

Takes a comma separated list hostnames and IP addresses and comma separated list of ports.

The project is written in Rust, using asynchronous requests making it light weight and fast.

## Installation
While this library is in initial state of development installation is done using cargo.

```bash
git checkout https://github.com/oliverdaff/rscan
cargo test 
cargo install --path .
```

## Usage

### Basic

Reads a list of domains from stdin.

```bash
hprobe scanme.nmap.com -p 0-200,250
```

### Flags And Options
```bash
rscan  0.1.0
Oliver Daff
A simple TCP port scanner

USAGE:
    rscan [OPTIONS] <HOSTS>... -p <PORTS>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c <CONCURRENCY>        The number of conccurent TCP connections to attempt [default: 100]
    -p <PORTS>...           The ports to scan.
    -t <TIMEOUT>            The timeout to establish a TCP connection [default: 1000]

ARGS:
    <HOSTS>...    The hosts to scan seperated by commas.
```


## Tests
The tests can be invoked with `cargo test`.

## License
MIT © Oliver Daff