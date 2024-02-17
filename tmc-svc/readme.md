## TMC Service

This is the server side of the solution.

<br/>

### Prerequisites and Setup

Install the `sqlx-cli` using `cargo install --version=0.7.3 sqlx-cli --no-default-features --features native-tls,postgres`. If the compilation fails due to missing `openssl.pc` file, then do `sudo apt install libssl-dev` and rerun `cargo install` it.

Run `./ops/db_init.sh` script that:

-   starts a PostgreSQL instance as a Docker container
-   runs the database migrations within.

<br/>

### Development

#### Run in "dev mode"

Use `./run_dev_watch.sh` to start the server and have it automatically restarted on detected code changes.

#### Database Model Changes

When database model (DDL related) changes need to be added:

1. Run `./ops/db_add_change.sh {change-name}`\
   (such as `./ops/db_add_change.sh create_table_access_classifiers`).
2. Populate the content of the generated sql file.
3. Run `./ops/db_apply_changes.sh` to apply the change.
