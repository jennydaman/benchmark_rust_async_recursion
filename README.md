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
hyperfine --warmup 3 "du -bs $DIR" "target/release/du_fs $DIR" "target/release/du_async $DIR"
```

## Results

#### 85GB HDD

```
hyperfine --warmup 3 "du -bs $DIR" "target/release/du_fs $DIR" "target/release/du_async $DIR"
Benchmark 1: du -bs /hdd/jennings
  Time (mean ± σ):      1.014 s ±  0.039 s    [User: 0.329 s, System: 0.681 s]
  Range (min … max):    0.985 s …  1.089 s    10 runs
 
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet PC without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
 
Benchmark 2: target/release/du_fs /hdd/jennings
  Time (mean ± σ):      1.147 s ±  0.049 s    [User: 0.327 s, System: 0.815 s]
  Range (min … max):    1.096 s …  1.245 s    10 runs
 
Benchmark 3: target/release/du_async /hdd/jennings
  Time (mean ± σ):      5.003 s ±  0.046 s    [User: 4.358 s, System: 2.614 s]
  Range (min … max):    4.953 s …  5.066 s    10 runs
 
Summary
  'du -bs /hdd/jennings' ran
    1.13 ± 0.06 times faster than 'target/release/du_fs /hdd/jennings'
    4.93 ± 0.19 times faster than 'target/release/du_async /hdd/jennings'
```

#### 485 GB NFS Mount

```
Benchmark 1: du -bs /neuro/labs/grantlab/research/Ai_Others
  Time (mean ± σ):      9.474 s ± 14.498 s    [User: 0.074 s, System: 0.364 s]
  Range (min … max):    2.415 s … 37.763 s    10 runs
 
  Warning: The first benchmarking run for this command was significantly slower than the rest (36.180 s). This could be caused by (filesystem) caches that were not filled until after the first run. You should consider using the '--warmup' option to fill those caches before the actual benchmark. Alternatively, use the '--prepare' option to clear the caches before each timing run.
 
Benchmark 2: target/release/du_fs /neuro/labs/grantlab/research/Ai_Others
  Time (mean ± σ):      6.094 s ± 10.685 s    [User: 0.065 s, System: 0.508 s]
  Range (min … max):    2.661 s … 36.504 s    10 runs
 
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet PC without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
 
Benchmark 3: target/release/du_async /neuro/labs/grantlab/research/Ai_Others
  Time (mean ± σ):     10.509 s ± 14.593 s    [User: 0.661 s, System: 0.942 s]
  Range (min … max):    3.257 s … 38.420 s    10 runs
 
  Warning: The first benchmarking run for this command was significantly slower than the rest (37.963 s). This could be caused by (filesystem) caches that were not filled until after the first run. You should consider using the '--warmup' option to fill those caches before the actual benchmark. Alternatively, use the '--prepare' option to clear the caches before each timing run.
 
Summary
  'target/release/du_fs /neuro/labs/grantlab/research/Ai_Others' ran
    1.55 ± 3.62 times faster than 'du -bs /neuro/labs/grantlab/research/Ai_Others'
    1.72 ± 3.86 times faster than 'target/release/du_async /neuro/labs/grantlab/research/Ai_Others'
```
