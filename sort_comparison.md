# Futhark has a number of sorts

Here is the sorting library we are using: https://github.com/diku-dk/sorts?tab=readme-ov-file#futhark-sorting-implementations--. It has a number of sorting
options. Here are their performance results.

## Merge sort

```
Number of Particles: 50000
Number of Pixels: 1000000
=====================================
Simulation: 72 ms,	Rendering: 3336 microseconds
Simulation: 69 ms,	Rendering: 3056 microseconds
Simulation: 69 ms,	Rendering: 3092 microseconds
Simulation: 69 ms,	Rendering: 3151 microseconds
Simulation: 70 ms,	Rendering: 2855 microseconds
Simulation: 69 ms,	Rendering: 2897 microseconds
Simulation: 68 ms,	Rendering: 3031 microseconds
Simulation: 58 ms,	Rendering: 2947 microseconds
Simulation: 55 ms,	Rendering: 3117 microseconds
Simulation: 54 ms,	Rendering: 3181 microseconds
Simulation: 54 ms,	Rendering: 3033 microseconds
```

## Radix sort

```
Number of Particles: 50000
Number of Pixels: 1000000
=====================================
Simulation: 19 ms,	Rendering: 3232 microseconds
Simulation: 20 ms,	Rendering: 2881 microseconds
Simulation: 20 ms,	Rendering: 2771 microseconds
Simulation: 20 ms,	Rendering: 3024 microseconds
Simulation: 20 ms,	Rendering: 2990 microseconds
Simulation: 20 ms,	Rendering: 2899 microseconds
Simulation: 20 ms,	Rendering: 2891 microseconds
Simulation: 20 ms,	Rendering: 2874 microseconds
Simulation: 19 ms,	Rendering: 2794 microseconds
Simulation: 20 ms,	Rendering: 3009 microseconds
Simulation: 20 ms,	Rendering: 3045 microseconds
```

## Bubble Sort

Takes forever to load first page, despite it already being sorted! As the docs say:

> This may be useful if you have almost-sorted data that you want to make fully-sorted in parallel. Obviously very slow for non-sorted data.

Well, we do, and it's still useless.

## Quick Sort

compilation fails!

## Bitonic Sort

```
Number of Particles: 50000
Number of Pixels: 1000000
=====================================
Simulation: 21 ms,	Rendering: 2630 microseconds
Simulation: 20 ms,	Rendering: 2601 microseconds
Simulation: 20 ms,	Rendering: 2749 microseconds
Simulation: 20 ms,	Rendering: 2910 microseconds
Simulation: 21 ms,	Rendering: 2706 microseconds
Simulation: 22 ms,	Rendering: 3037 microseconds
Simulation: 22 ms,	Rendering: 3113 microseconds
Simulation: 21 ms,	Rendering: 3021 microseconds
Simulation: 20 ms,	Rendering: 3246 microseconds
Simulation: 20 ms,	Rendering: 3022 microseconds
Simulation: 21 ms,	Rendering: 3096 microseconds
Simulation: 21 ms,	Rendering: 3105 microseconds
```

## Conclusion

Looks like Radix is the fastest!
