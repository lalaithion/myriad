# Futhark has a number of sorts

Here is the sorting library we are using: https://github.com/diku-dk/sorts?tab=readme-ov-file#futhark-sorting-implementations--. It has a number of sorting
options. Here are their performance results.

## Merge sort

```
Number of Particles: 50000
Number of Pixels: 1000000
=====================================
Simulation: 19 ms,	Rendering: 2101 microseconds
Simulation: 19 ms,	Rendering: 2178 microseconds
Simulation: 19 ms,	Rendering: 2188 microseconds
Simulation: 19 ms,	Rendering: 2199 microseconds
Simulation: 19 ms,	Rendering: 2173 microseconds
```

## Radix sort

```
Number of Particles: 50000
Number of Pixels: 1000000
=====================================
Simulation: 19 ms,	Rendering: 2301 microseconds
Simulation: 19 ms,	Rendering: 2277 microseconds
Simulation: 19 ms,	Rendering: 2301 microseconds
Simulation: 19 ms,	Rendering: 2282 microseconds
Simulation: 19 ms,	Rendering: 2294 microseconds
```

## Bubble Sort

Takes forever to load first page, despite it already being sorted! As the docs say:

> This may be useful if you have almost-sorted data that you want to make fully-sorted in parallel. Obviously very slow for non-sorted data.

Well, we do, and it's still useless.

## Quick Sort

compilation fails!

## Bitonic Sort

This one is weird; I think there's a bug in it, it deletes? moves? values out of the bottom half of the screen.

## Conclusion

Looks like merge or radix is the way to go.
