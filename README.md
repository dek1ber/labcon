# labcon
Homelabbing console - configurable as an sync/async TUI or a prometheus data stream

All node connections are hooked through ansible, with optional capabilities for importing ansible jobs

# Building
With cargo 
```bash
cargo run -- <FLAGS>
```

As standalone executable
```bash
cargo build --release
```
(this will output a binary into `./target/release/labcon`)

# Usage 
``` bash
cargo run -- --interface <MON_IFACE> --group <ANSIBLE_GROUP> <OPT_ARGS>
```

<MON_IFACE> specifies the monitoring interface onto which labcon will operate
<ANSIBLE_GROUP> specifies an ansible group (in `/etc/ansible/hosts` or otherwise) to connect to 
<OPT_ARGS> tbd...
