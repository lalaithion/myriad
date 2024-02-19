# Perf notes

Number of Particles: 40000

## Base

Simulation: 59,758 microseconds, Transferring: 1,378 microseconds, Rendering: 2,953 microseconds

## Sigmdrag + if only on dx

Simulation: 67,861 microseconds, Transferring: 1,715 microseconds, Rendering: 3,686 microseconds

## Single If

Simulation: 61,468 microseconds, Transferring: 1,666 microseconds, Rendering: 3,680 microseconds

## Double If

Simulation: 59,769 microseconds, Transferring: 1,655 microseconds, Rendering: 3,656 microseconds

## Hypot

Simulation: 66,844 microseconds, Transferring: 1,707 microseconds, Rendering: 3,644 microseconds

## Early if + sqrt

Simulation: 61,049 microseconds, Transferring: 1,603 microseconds, Rendering: 3,504 microseconds

## Reduce (not comm)

Simulation: 60,094 microseconds, Transferring: 1,593 microseconds, Rendering: 3,514 microseconds

## use d to exclude self

Simulation: 54,594 microseconds, Transferring: 1,690 microseconds, Rendering: 3,529 microseconds

## Sigmoid drag

Simulation: 59,582 microseconds, Transferring: 1,636 microseconds, Rendering: 8,056 microseconds

## Tan drag

Simulation: 61,399 microseconds, Transferring: 1,705 microseconds, Rendering: 3,685 microseconds

## If in sigm

Simulation: 57,944 microseconds, Transferring: 1,672 microseconds, Rendering: 3,645 microseconds

## Sigm higher threshold

Simulation: 56,457 microseconds, Transferring: 1,615 microseconds, Rendering: 3,521 microseconds
