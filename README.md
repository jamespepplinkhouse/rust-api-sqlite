# rust-api-sqlite

## Overview

I made this in 20 minutes or so by asking ChatGPT-4 things like:

- Write me a high performance REST api server in rust code
- How do I add sqlite support to read users from a database

## Getting Started

```sh
$ ./scripts/create-db.sh
$ cargo run
```

- Optionally, use a DB tool like DBeaver to add some rows to the DB (users.db)
- Open http://127.0.0.1:8080/user/1 in a browser
- Be amazed that it returns data from SQLite in <5 ms! (on my Macbook Pro M2) :-D

## Tips

If you run into trouble:

- Ask ChatGPT to explain the code
- Ask ChatGPT to solve compiler errors/warnings
