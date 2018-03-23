# Rust API

Basic setup for Rust API project with Rocket.


## Install

```bash
$ curl https://sh.rustup.rs -sSf | sh
```

## Setup

```bash
$ rustup default nightly

$ rustup override set nightly

$ rustup update && cargo update
```

## Rust Multi-stage build

The dockerized image is only `13MB` in size:

```bash
# Build the docker image
$ docker build -t alextanhongpin/rust-api .

$ docker images | grep rust
alextanhongpin/rust-api                                                      latest              8f4359bacb1c        16 hours ago        13MB
```