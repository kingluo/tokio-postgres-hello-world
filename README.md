# tokio-postgres-hello-world
hello world example for tokio-postgres.

The tokio-postgres has no offical exmaple so far, so I copy some test codes into standalone project.

I hope that it is referential example for people who also are interested in tokio-postgres. After all, `async/.await` database query is necessary for most backend apps.

## Run
Given you have postgresql running in local host (port=5432), and create a user `tmp`.

git clone this project in the same host, then just run it:

```
cargo run
```
