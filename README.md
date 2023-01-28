# Benchmarking async_recursion with Disk I/O

`du_fs` and `du_async` are programs which behave like `du -bs`.

- `du_fs` is implemented using Rust std
- `du_async` is implemented using tokio

## Build

```shell
cargo build --release
```

## Run

```shell
DIR=/neuro/labs/grantlab/research/Ai_Others
hyperfine --warmup 3 \
    "du -bs $DIR" \
    "fd --unrestricted --threads 1 . $DIR" \
    "fd --unrestricted . $DIR" \
    "target/release/du_fs $DIR" \
    "target/release/du_async $DIR"
```

## Results

#### 85GB HDD

```
Benchmark 1: du -bs /hdd/jennings
  Time (mean ± σ):      1.005 s ±  0.030 s    [User: 0.311 s, System: 0.689 s]
  Range (min … max):    0.983 s …  1.086 s    10 runs
 
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet PC without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
 
Benchmark 2: fd --unrestricted --threads 1 . /hdd/jennings
  Time (mean ± σ):     513.6 ms ±  29.7 ms    [User: 324.6 ms, System: 345.4 ms]
  Range (min … max):   484.5 ms … 582.7 ms    10 runs
 
Benchmark 3: fd --unrestricted . /hdd/jennings
  Time (mean ± σ):     182.8 ms ±   4.6 ms    [User: 581.9 ms, System: 546.6 ms]
  Range (min … max):   175.4 ms … 189.8 ms    15 runs
 
Benchmark 4: target/release/du_fs /hdd/jennings
  Time (mean ± σ):      1.136 s ±  0.033 s    [User: 0.278 s, System: 0.854 s]
  Range (min … max):    1.113 s …  1.212 s    10 runs
 
Benchmark 5: target/release/du_async /hdd/jennings
  Time (mean ± σ):      5.028 s ±  0.281 s    [User: 4.350 s, System: 2.630 s]
  Range (min … max):    4.788 s …  5.733 s    10 runs
 
Summary
  'fd --unrestricted . /hdd/jennings' ran
    2.81 ± 0.18 times faster than 'fd --unrestricted --threads 1 . /hdd/jennings'
    5.50 ± 0.21 times faster than 'du -bs /hdd/jennings'
    6.21 ± 0.24 times faster than 'target/release/du_fs /hdd/jennings'
   27.50 ± 1.68 times faster than 'target/release/du_async /hdd/jennings'
```

#### 485 GB NFS Mount

```
Benchmark 1: du -bs /neuro/labs/grantlab/research/Ai_Others
  Time (mean ± σ):      8.960 s ± 13.800 s    [User: 0.058 s, System: 0.345 s]
  Range (min … max):    2.369 s … 35.402 s    10 runs
 
  Warning: The first benchmarking run for this command was significantly slower than the rest (35.402 s). This could be caused by (filesystem) caches that were not filled until after the first run. You should consider using the '--warmup' option to fill those caches before the actual benchmark. Alternatively, use the '--prepare' option to clear the caches before each timing run.
 
Benchmark 2: fd --unrestricted --threads 1 . /neuro/labs/grantlab/research/Ai_Others
  Time (mean ± σ):      2.373 s ±  0.062 s    [User: 0.076 s, System: 0.142 s]
  Range (min … max):    2.328 s …  2.498 s    10 runs
 
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet PC without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
 
Benchmark 3: fd --unrestricted . /neuro/labs/grantlab/research/Ai_Others
  Time (mean ± σ):     307.5 ms ±  11.1 ms    [User: 76.4 ms, System: 113.2 ms]
  Range (min … max):   299.6 ms … 337.1 ms    10 runs
 
Benchmark 4: target/release/du_fs /neuro/labs/grantlab/research/Ai_Others
  Time (mean ± σ):      6.042 s ± 10.445 s    [User: 0.058 s, System: 0.533 s]
  Range (min … max):    2.541 s … 35.756 s    10 runs
 
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet PC without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
 
Benchmark 5: target/release/du_async /neuro/labs/grantlab/research/Ai_Others
  Time (mean ± σ):      6.716 s ± 10.749 s    [User: 0.578 s, System: 0.776 s]
  Range (min … max):    3.126 s … 37.299 s    10 runs
 
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet PC without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
 
Summary
  'fd --unrestricted . /neuro/labs/grantlab/research/Ai_Others' ran
    7.72 ± 0.34 times faster than 'fd --unrestricted --threads 1 . /neuro/labs/grantlab/research/Ai_Others'
   19.65 ± 33.98 times faster than 'target/release/du_fs /neuro/labs/grantlab/research/Ai_Others'
   21.84 ± 34.97 times faster than 'target/release/du_async /neuro/labs/grantlab/research/Ai_Others'
   29.14 ± 44.89 times faster than 'du -bs /neuro/labs/grantlab/research/Ai_Others'
```
