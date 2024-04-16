# euchre
NEAT (almost) for Euchre entirely (mostly?) in stack memory.

## TODO
- improve tutor mode

## To generate flamegraphs
perf record -g -- cargo flamegraph --bin euchre -- evolve --thread-count 4 --population-size 16 --generations 10 --out-dir out
