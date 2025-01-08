# proclist

![Project badge](https://img.shields.io/badge/language-Rust-blue.svg)
![Crates.io License](https://img.shields.io/crates/l/proclist)
![GitHub Release](https://img.shields.io/github/v/release/PlexSheep/proclist)
![GitHub language count](https://img.shields.io/github/languages/count/PlexSheep/proclist)
[![Rust CI](https://github.com/PlexSheep/proclist/actions/workflows/cargo.yaml/badge.svg)](https://github.com/PlexSheep/hedu/actions/workflows/cargo.yaml)

a tiny list of os processes

- [GitHub](https://github.com/PlexSheep/proclist)
- [crates.io](https://crates.io/crates/proclist)
- [docs.rs](https://docs.rs/crate/proclist/)

## What?

It just prints out the os processes.

```
% ./target/release/proclist net
UID             PID       NAME                          CMD
0               6         kworker/R-netns
0               113       kworker/R-inet_
0               942       networkd-dispat               /usr/bin/python3 /usr/bin/networkd-dispatcher --run-startup-triggers
998             34920     netpulsed                     /usr/local/bin/netpulsed --daemon
```
