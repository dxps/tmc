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
    -   This is future planned.
-   Back-end ([tmc-svc](./tmc-svc/readme.md))
    -   Implemented as a Rust based REST API service.
    -   Using Axum as web server.
    -   Server-side rendered UI using Askama.
    -   sqlx for an easy database interactions, incl migration.
-   Database as PostgreSQL.
