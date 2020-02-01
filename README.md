# dns-client

Based on [rust-dnsclient](https://github.com/jedisct1/rust-dnsclient).

## Usage

```shell script
dns-client 0.0.1
Torben Carstens
Lookup of a given website via different DNS servers (default is 8.8.8.8 and 1.1.1.1)

USAGE:
    dns-client [OPTIONS] [target]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -u, --upstreams <upstreams>

ARGS:
    <target>
```

```shell script
dns-client <target> [-u <dnsserver>[,...dnsserver]]
```

### Example

```shell script
$ dns-client -u 1.1.1.1 example.com
93.184.216.34
```