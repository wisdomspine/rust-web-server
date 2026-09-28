# Web Server

A small multithreaded HTTP server. It listens on `127.0.0.1:7878`, hands each connection to a worker in a thread pool, and shuts down after a fixed number of requests.

Run it from the project root. The handlers read `static/hello.html`, `static/sleep.html`, and `static/404.html` relative to the current directory.

## Requirements

- Rust and Cargo (edition 2024)

## Run

Both arguments are required. The first is the maximum number of worker threads. The second is the maximum number of connections the server will accept before it exits.

```bash
cargo run -- <max_workers> <max_requests>
```

Example: eight workers, then exit after 200 connections.

```bash
cargo run -- 8 200
```

The `--` is required. Cargo treats everything after it as arguments to the program.

### Limits

| Argument | Position | Meaning | Floor |
| --- | --- | --- | --- |
| `max_workers` | 1st | Worker threads in the pool | 4 |
| `max_requests` | 2nd | Connections accepted, then the process exits | 100 |

A value at or below the floor is raised to that floor. `cargo run -- 2 10` still starts 4 workers and accepts 100 connections. A value that is not a number is treated the same way. Omitting either argument stops the process with a panic.

`max_workers` is also the number of requests that can run at the same time. Further connections wait in the job queue until a worker is free.

## Routes

| Request | Response |
| --- | --- |
| `GET /` | `static/hello.html` |
| `GET /sleep` | Waits 5 seconds, then `static/sleep.html`. The worker stays busy for those 5 seconds. |
| Anything else | `static/404.html` with status `404 Not Found` |

Try them while the server is running:

```bash
curl -i http://127.0.0.1:7878/
curl -i http://127.0.0.1:7878/sleep
curl -i http://127.0.0.1:7878/missing
```

After `max_requests` connections, the listener loop ends, the pool drops its workers, and the process exits.
