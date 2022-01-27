# cpus

Minimal cmd line tool to see CPU core usage

## Why

- I'd use this myself -- to quickly measure parallel program performance
- I bet this could get reasonably big

## Design

- Just show current core state
- Design: 1 line that refreshes -- maybe just outputs a line every refresh rate (maybe with timestamp)
- Don't refresh. Just output a current snapshot. Then use other tools to output repeatedly (or include a flag to also do this).

```
5 12 6 7 55 100 87 100
```
