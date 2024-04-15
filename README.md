# euchre
NEAT (almost) for Euchre entirely (mostly?) in stack memory.

## TODO
- decouple NeuralNetwork from euchre to support user player
- tutor mode (asks for input, feed it in, tells you what it wants to do)
- manual breeding command
- neural network stats command

## To generate flamegraphs
perf record -g -- cargo flamegraph --bin euchre -- evolve --thread-count 4 --population-size 16 --generations 10 --out-dir out
