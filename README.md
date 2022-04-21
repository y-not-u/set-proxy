# set-proxy

a CLI to set proxy for npm, yarn, git or some other dev CLI more easily

## Usage

### Set Proxy

```bash
# set proxy for yarn
setproxy yarn http://127.0.0.1:7890
```

### Unset Proxy

```bash
setproxy -d yarn
```

## Custom

you can write more commands in `cargo.toml`

Off course, **setproxy** will read the specified file via command `-c` or `--config`

*config.toml*

```toml
[git]
set = [
  "git config http.proxy {url}"
]
unset = [
  "git config --unset http.proxy"
]

[npm]
set = [
  "npm config set proxy {url}",
  "npm config set https-proxy {url}"
]
unset = [
  "npm config delete proxy",
  "npm config delete https-proxy",
]
```

