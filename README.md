# postgresql-example

This is an experiment with PostgreSQL and Rust.

## Prerequisites

* Docker and docker-compose

## Usage

Start the PostgreSQL instance using `docker-compose`:

```console
docker compose up -d
```

This will launch the container as a background process.

You can now launch the program with `cargo run`:

```console
$ cargo run
   Compiling postgresql-example v0.1.0 (/home/yozhgoor/repos/postgresql-example)
    Finished dev [unoptimized + debuginfo] target(s) in 1.63s
     Running `target/debug/postgresql-example`

Connecting to the database
Creating table called `users`

Populating `users`
found user: 1) user1 | pass1 | user1@test.com
found user: 2) user2 | pass2 | user2@test.com
found user: 3) user3 | pass3 | user3@test.com

Updating username of the user with id `2`
found user: 1) user1 | pass1 | user1@test.com
found user: 3) user3 | pass3 | user3@test.com
found user: 2) jack1 | pass2 | user2@test.com

Delete user with id `1` and `3`
found user: 2) jack1 | pass2 | user2@test.com
```

To stop and remove containers created by `docker compose up`, you can use `docker compose down`.
