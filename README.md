# Hnefatafl CLI

TLDR

```bash
cargo run --release
```

Welcome to a small command line interface for Hnefatafl. 
We implemented two bots: MCTS and AlphaBeta-Pruning. 
Both algorithms can be run with any evaluation function you provide. 
By default, AlphaBeta will use a human evaluation function and MCTS will use random rollouts. 

You can also add new evaluation function and see how the bots perform using these. 

This directory also contains a failed experiment with neural networks. 
The initial idea was to use MuZero or AlphaZero to study this game. 
Unfortunately, this did not work. 

There is also an implementation of Magic Bitboards. 
Unfortunately, this also did not work. 

## Running the Code

For the code to run, you must be in an enviroment that has python and pytorch 2.3.0 installed. 
Moreover, you must set the following enviroment variable: 

```
LIBTORCH_USE_PYTORCH=1
```

Then everything should be running. 
If it does not work, just delete the folder `./synthesis` and every reference to tchrs and it should work. 

## MISC

This folder contains some micellanious files. 
Among those are files with the extension `.ot`. 
These are failed neural networks. 

There is also a flamegraph which we is a result of our profiling efforts. 
The current flamegraph was a direct result of the magic number generation. 

The folders `./results` and `./replays` contain the benchmarking data we used to generate the plots in the write up and presentation. 