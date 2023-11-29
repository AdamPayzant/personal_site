# Personal Site

My personal site. Developed using Leptos.

## Running Locally

### Requirements

* Rust installed via rustup.

* If you don't have `wasm32-unknown-unknown` already, run:

```
rustup target add wasm32-unknown-unknown
```
* If you don't have `trunk` installed, run the following command and verify `trunk` is on your $PATH
```
cargo install trunk
```

### Execution

Once you've met the requirements, just run:
```
trunk serve
```

Now you can connect to the webpage at `http://localhost:8080/`
