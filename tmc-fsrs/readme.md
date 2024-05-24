## TMC - A fullstack Rust project

<br/>

### Prerequisites & Setup

The followings are the required tools and steps to have the proper setup for the app to run.

#### Front-end related

Note: These are needed during development. In other words, if you don't change any code in the components (within `rsx` blocks), then there is no need to run the Tailwind CSS compiler in parallel with the back-end.

1. Install [npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm).
2. Install the Tailwind CSS [CLI](https://tailwindcss.com/docs/installation).
3. Start the Tailwind CSS compiler using `./run_css.sh` script.

#### Back-end related

1. Install the `sqlx-cli` using `cargo install --version=0.7.4 sqlx-cli --no-default-features --features native-tls,postgres`.<br/>
   If the compilation fails due to a missing `openssl.pc` file, then do `sudo apt install libssl-dev` and `cargo install` it.
2. Run `./ops/db_init.sh` that:
    - Starts a PostgreSQL database instance as a Docker container.
    - Runs the database migrations within.
3. Install the [`dioxus-cli`](https://crates.io/crates/dioxus-cli) using `cargo install dioxus-cli --version=0.5.4`.

<br/>

### Run

Launch the app using `./run-dev.sh`.

In case of compilation error that remain hidden behind Dioxus CLI, run `cargo check --features server,web` to reveal them.
