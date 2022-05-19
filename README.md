# Copy-dt

Copies the date and time to the clipboard using the following format string:

```
"%Y-%m-%d %H-%M-%S"
```

To use:

```bash
cargo build --release
cp target/release/copy-dt [your destination directory]
```

Then, just run `copy-dt` to copy the date and time into the clipboard!