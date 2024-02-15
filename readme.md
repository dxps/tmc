## TM Community

An open source solution for our dear community of TM practicants and ecosystem.

This uses the classic three-tier architecture:

```
╭──────────────────╮      ╭──────────────────╮        ╭──────────────────╮
│      Web UI      │      │     Back-end     │        │     Database     │
│     (tmc-ui)     ├──────▶    (tmc-svc)     ├────────▶   (postgresql)   │
╰──────────────────╯      ╰──────────────────╯        ╰──────────────────╯
```

with:

-   Web UI (to be) implemented as a React.js SPA
-   Back-end implemented as a Rust based REST API service.
