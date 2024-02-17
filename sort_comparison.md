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
Simulation: 113 ms,	Rendering: 3565 microseconds
Simulation: 110 ms,	Rendering: 3480 microseconds
Simulation: 105 ms,	Rendering: 3387 microseconds
Simulation: 102 ms,	Rendering: 3404 microseconds
Simulation: 103 ms,	Rendering: 3330 microseconds
Simulation: 104 ms,	Rendering: 3420 microseconds
Simulation: 106 ms,	Rendering: 3339 microseconds
Simulation: 106 ms,	Rendering: 3358 microseconds
Simulation: 104 ms,	Rendering: 3390 microseconds
Simulation: 106 ms,	Rendering: 3444 microseconds
Simulation: 108 ms,	Rendering: 3520 microseconds
Simulation: 102 ms,	Rendering: 3432 microseconds
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

Looks like Bitonic is the fastest!
