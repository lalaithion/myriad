# Ventrella Clusters

This is a GPU accelerated variant of Jeffrey Ventrella's Clusters
algorithm (see https://www.ventrella.com/Clusters/). I created
this because (1) I didn't see any other GPU accelerated variants, and
(2) I wanted to experiment with the programming language futhark, and (3)
I wanted to see what a simulation with tens of thousands of cells would do

The implementation is purposefully not identical; I am playing with a number
of different force models to see what produces interesting interactions.

## Dependencies

This requires python3, futhark, pyopencl, numpy, and pysdl2 to be installed.

It works on my machine! (macOS 12.6, Air M1)

./run.sh recompiles and runs the project.
