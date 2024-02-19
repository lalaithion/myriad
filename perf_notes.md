# Comparing performance

## Number of particles

Using a fixed, seed, and recording average timings over 30 seconds, we compare different
scenarios that differ only in number of particles.

This was before I started to optimize transferring.

### 10,000 Particles

```
Simulation: 1,321 microseconds, Transferring: 10,038 microseconds,      Rendering: 5,129 microseconds
Simulation: 1,446 microseconds, Transferring: 9,921 microseconds,       Rendering: 9,730 microseconds
Simulation: 1,463 microseconds, Transferring: 9,737 microseconds,       Rendering: 9,710 microseconds
```

### 20,000 Particles

```
Simulation: 1,315 microseconds, Transferring: 11,966 microseconds,      Rendering: 4,516 microseconds
Simulation: 1,487 microseconds, Transferring: 11,992 microseconds,      Rendering: 8,820 microseconds
Simulation: 1,396 microseconds, Transferring: 11,634 microseconds,      Rendering: 5,625 microseconds
```

## 40,000 Particles

```
Simulation: 1,573 microseconds, Transferring: 41,266 microseconds,      Rendering: 4,193 microseconds
Simulation: 1,510 microseconds, Transferring: 35,371 microseconds,      Rendering: 3,844 microseconds
Simulation: 1,428 microseconds, Transferring: 31,446 microseconds,      Rendering: 3,773 microseconds
```

### 80,000 Particles

```
Simulation: 1,930 microseconds, Transferring: 177,669 microseconds,     Rendering: 5,463 microseconds
Simulation: 1,651 microseconds, Transferring: 166,384 microseconds,     Rendering: 5,473 microseconds
Simulation: 1,654 microseconds, Transferring: 155,347 microseconds,     Rendering: 5,479 microseconds
```

## 160,000 Particles

```
Simulation: 3,044 microseconds, Transferring: 712,823 microseconds,     Rendering: 7,358 microseconds
Simulation: 1,703 microseconds, Transferring: 697,345 microseconds,     Rendering: 7,538 microseconds
```

### 240,000 Particles

Segfault! Interesting.

## Number of GPU steps

All done with 40,000 particles, timings are 10 second averages.

### One step

```
Simulation: 1,514 microseconds, Transferring: 49,798 microseconds,      Rendering: 4,146 microseconds
Simulation: 1,506 microseconds, Transferring: 49,387 microseconds,      Rendering: 4,600 microseconds
Simulation: 1,422 microseconds, Transferring: 49,104 microseconds,      Rendering: 10,233 microseconds
Simulation: 1,293 microseconds, Transferring: 48,681 microseconds,      Rendering: 3,435 microseconds
```

### Two steps

```
Simulation: 51,768 microseconds,        Transferring: 49,750 microseconds,      Rendering: 4,144 microseconds
Simulation: 50,945 microseconds,        Transferring: 49,567 microseconds,      Rendering: 4,536 microseconds
Simulation: 50,423 microseconds,        Transferring: 49,325 microseconds,      Rendering: 4,436 microseconds
```

...I mean, what? This is like 30 times slower.

### Three steps

```
Simulation: 104,021 microseconds,       Transferring: 50,906 microseconds,      Rendering: 4,253 microseconds
Simulation: 102,903 microseconds,       Transferring: 50,661 microseconds,      Rendering: 4,522 microseconds
Simulation: 101,210 microseconds,       Transferring: 50,318 microseconds,      Rendering: 4,632 microseconds
Simulation: 99,820 microseconds,        Transferring: 49,922 microseconds,      Rendering: 4,495 microseconds
```

This is insane.

## In a loop, part two; put the loop on the CPU side?

### One step

This is just the original normal code.

```
Simulation: 1,700 microseconds, Transferring: 49,578 microseconds,      Rendering: 9,858 microseconds
Simulation: 1,461 microseconds, Transferring: 49,117 microseconds,      Rendering: 4,062 microseconds
Simulation: 1,384 microseconds, Transferring: 48,808 microseconds,      Rendering: 3,766 microseconds
```

### Two steps

```
Simulation: 52,607 microseconds,        Transferring: 50,529 microseconds,      Rendering: 3,921 microseconds
Simulation: 51,010 microseconds,        Transferring: 50,253 microseconds,      Rendering: 3,807 microseconds
Simulation: 50,267 microseconds,        Transferring: 49,873 microseconds,      Rendering: 3,909 microseconds
```

WHAT WHY. HOW CAN IT POSSIBLY TAKE 30 TIMES AS LONG TO DO IT TWICE THAN ONCE.
