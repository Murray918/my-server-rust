
# my-server-rust

Simple Rest Api using Rust. This project will use Axium and Postgres to stand up basic crud operations.

## Running Locally

To get this repo running you will need to clone the repo onto your system.

ensure that you have docker installed
for linux

```bash
sudo apt install docker.io
```

For docker compose you can eiteher install

```bash
sudo apt install docker-compose-v2
```

or

```bash
sudo apt install docker-compose-plugin
```

To start the server:

```bash
cd my-server rust

docker compose up -d

cargo build
cargo run
```
