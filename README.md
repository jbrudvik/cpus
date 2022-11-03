# cpus

Print CPU usage (%) for each core

## Usage

### Print help information

```
$ cpus -h
Print CPU usage (%) for each core

Usage: cpus [OPTIONS]

Options:
  -w, --watch    Print CPU usage (%) for each core once per second, forever
  -c, --clear    Clear the screen after printing (watch-mode only)
  -h, --help     Print help information
  -V, --version  Print version information
```

### Print CPU usage (%) for each core once

```
$ cpus
 16  15  13  10   6   0   0   0
```

### Print CPU usage (%) for each core once per second, forever (until killed)

```
$ cpus -w
 16  16  13  10   2   2   0   0
 12   9   8   6   1   0   0   0
 20  20  19  14   0   0   0   1
 22  19  18  11   1   0   0   0
 24  18  17  17   0   1   0   0
...
```

## Install

```
$ git clone https://github.com/jbrudvik/cpus
$ cd cpus
$ cargo install --path .
```
