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
cd rust_web_server
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

## Environment Variables

You can set environment variables:

- `PUBLIC_PATH` for setting the public path for you server, by defualt it's `/public` folder in the app
- `PORT` for the port which webserver runs on it, by default it's `8080`

## Endpoints

- `/` Loads a static HTML file from public directory
- `/json` returns a JSON output with related header
- `/headers` returns all the headers client sent to the server
- `/query` returns all query strings client requested from url
- `/query/foo` returns the single query string called `foo` value
- Not found on other routes
