We use `sea-orm-cli` to generate entities, and for the convenience of management, we pull the entities out and put them in a separate directory.

Command：

```shell

sea generate entity -o crates/entry_x/src/domain/entity --with-serde=both --serde-skip-hidden-column --date-time-crate=time
```
