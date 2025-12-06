## The idea

I learned recently about the one billion rows challenge and thought it would be a good opportunity to practice a bit of Rust and learn
how to use MacOS Instruments for profiling - I usually do my coding in Linux and use perf.

I start from a very naive implementation and used profiling to decide what to attack next. I committed each step so it could work as a
bit of a guide. This was the contribution of each of the steps:

```
> hyperfine --warmup 2 ./hash ./integer ./mmap ./naive ./threaded
Benchmark 1: ./hash
  Time (mean ± σ):     27.834 s ±  0.269 s    [User: 26.915 s, System: 0.893 s]
  Range (min … max):   27.362 s … 28.202 s    10 runs

Benchmark 2: ./integer
  Time (mean ± σ):     10.865 s ±  0.053 s    [User: 10.026 s, System: 0.829 s]
  Range (min … max):   10.784 s … 10.963 s    10 runs

Benchmark 3: ./mmap
  Time (mean ± σ):     16.114 s ±  0.123 s    [User: 15.245 s, System: 0.862 s]
  Range (min … max):   15.965 s … 16.346 s    10 runs

Benchmark 4: ./naive
  Time (mean ± σ):     36.823 s ±  0.349 s    [User: 35.859 s, System: 0.936 s]
  Range (min … max):   36.468 s … 37.591 s    10 runs

Benchmark 5: ./threaded
  Time (mean ± σ):      1.492 s ±  0.005 s    [User: 11.277 s, System: 0.359 s]
  Range (min … max):    1.487 s …  1.499 s    10 runs

Summary
  ./threaded ran
    7.28 ± 0.04 times faster than ./integer
   10.80 ± 0.09 times faster than ./mmap
   18.65 ± 0.19 times faster than ./hash
   24.68 ± 0.25 times faster than ./naive
```

## The challenge

See https://www.morling.dev/blog/one-billion-row-challenge/

The text file contains temperature values for a range of weather stations.
Each row is one measurement in the format `<string: station name>;<double: measurement>`, with the measurement value having exactly one fractional digit.
The following shows ten rows as an example:

```
Hamburg;12.0
Bulawayo;8.9
Palembang;38.8
St. John's;15.2
Cracow;12.6
Bridgetown;26.9
Istanbul;6.2
Roseau;34.4
Conakry;31.2
Istanbul;23.0
```

The task is to write a Java program which reads the file, calculates the min, mean, and max temperature value per weather station, and emits the results on stdout like this
(i.e. sorted alphabetically by station name, and the result values per station in the format `<min>/<mean>/<max>`, rounded to one fractional digit):

```
{Abha=-23.0/18.0/59.2, Abidjan=-16.2/26.0/67.3, Abéché=-10.0/29.4/69.0, Accra=-10.1/26.4/66.4, Addis Ababa=-23.7/16.0/67.0, Adelaide=-27.8/17.3/58.5, ...}
```

## Rules and limits

- No external library dependencies may be used
- Implementations must be provided as a single source file
- The computation must happen at application _runtime_, i.e. you cannot process the measurements file at _build time_
- Input value ranges are as follows:
  - Station name: non null UTF-8 string of min length 1 character and max length 100 bytes, containing neither `;` nor `\n` characters. (i.e. this could be 100 one-byte characters, or 50 two-byte characters, etc.)
  - Temperature value: non null double between -99.9 (inclusive) and 99.9 (inclusive), always with one fractional digit
- There is a maximum of 10,000 unique station names
- Line endings in the file are `\n` characters on all platforms
- Implementations must not rely on specifics of a given data set, e.g. any valid station name as per the constraints above and any data distribution (number of measurements per station) must be supported
- The rounding of output values must be done using the semantics of IEEE 754 rounding-direction "roundTowardPositive"
