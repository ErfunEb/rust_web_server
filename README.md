# Rust Web Server

A web server written in vanila Rust using standard library!

## How to run

**Note:** You need to have rust and cargo installed in your terminal or command line.

Clone the repository:

```sh
git clone https://github.com/ErfunEb/rust_web_server.git
```

After cloning the repo, go to the `rust_web_server` directory:

```sh
cd cmd_todos
```

At the end, run the following command for build and run:

```sh
cargo build
target/debug/server # this is the executable created after build
```

Or alternatively, instead of build and manually run the executable, you can run the following command:

```sh
cargo run
```

## Endpoints

- `/` Loads a static HTML file from public directory
- `/json` returns a JSON output with related header
- `/headers` returns all the headers client sent to the server
- `/query` returns all query strings client requested from url
- `/query/foo` returns the single query string called `foo` value
- Not found on other routes
