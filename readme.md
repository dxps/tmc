## TM Community

An open source solution for our community of TM practicants and the ecosystem around it.

This solution is a Rust based web app using Dioxus Fullstack and its Server Functions.<br/>
This makes the architecture a classic (three-tier) one:

```
╭──────────────────╮      ╭──────────────────╮        ╭──────────────────╮
│      Web UI      │      │     Back-end     │        │     Database     │
│    (tmc-fsrs)    ├──────▶    (tmc-fsrs)    ├────────▶   (postgresql)   │
╰──────────────────╯      ╰──────────────────╯        ╰──────────────────╯
```

with the following benefits:

-   SSR (server side routing)
    -   Being able to navigate to a specific route (as in "deep links").
-   CSR (client side routing)
    -   The view/page is rendered by the UI only, no need for a round trip to the back-end.
    -   URL reflects the path while navigating through pages
-   Using server functions that talk with the database.
    -   A nice feature provided by Dioxus Fullstack that allowed faster development,<br/>
        as there is no need to expose an API operation and then call it from UI.

The implementation is in [tmc-fsrs](./tmc-fsrs/) directory.

Old stuff (some previous iterations) are kept in [old](./old/) directory.
