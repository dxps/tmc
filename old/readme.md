## TM Community

An open source solution for our dear community of TM practicants and ecosystem.

It uses the classic three-tier architecture:

```
╭──────────────────╮      ╭──────────────────╮        ╭──────────────────╮
│      Web UI      │      │     Back-end     │        │     Database     │
│     (tmc-ui)     ├──────▶    (tmc-svc)     ├────────▶   (postgresql)   │
╰──────────────────╯      ╰──────────────────╯        ╰──────────────────╯
```

with the following components (and their stack):

-   Web UI
    -   Implemented as a React.js SPA
    -   Considered in the future.
-   Back-end ([tmc-svc](./tmc-svc/readme.md))
    -   Implemented as a Rust based REST API service.
    -   Using Axum as web server.
    -   Server-side rendered UI using Askama or Leptos
        (_to be evaluated and decided_).
    -   sqlx for an easier database interactions, incl db migrations.
-   Database
    -   Using PostgreSQL.
