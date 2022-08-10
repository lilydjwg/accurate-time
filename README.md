## Introduction

Show accurate time in tmux status, i.e. update exactly on every second change.

This resolves the issue when you have multiple such time statuses in sight,
they update in a randam order: even different sessions of the same tmux
instance update at different time!

In addition, this program reads the system load for tmux and color it so you
can tell if the system is overloaded at a glance.

## Installation and Configuration

Compile it like other Rust programs (install cargo first if you haven't yet):

```sh
cargo build --release
```

and put the built `target/release/accurate-time` to one of you `$PATH` directories.

You can use the following in your `~/.tmux.conf` to fallback if accurate-time isn't available:

```
if-shell "accurate-time tmux" {
  set -g status-interval 0
} {
  set -g status-interval 1
  set -g status-right "#[fg=red]#(awk '{print $1, $2, $3}' /proc/loadavg) #[fg=colour15]%Y-%m-%d %H:%M:%S"
}
```

Note that accurate-time only updates `status-right`, so you need to configure other parts elsewhere.
