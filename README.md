```getgpuname```
Gets the GPU name from the PCI-IDS database using either the provided parameters or the ones in /sys/class/drm/

## Usage:
Get the gpu automatically
```rust
use getgpuname::get_gpu_name;

fn main() {
    if let Some(gpu) = get_gpu_name() {
        println!("{}", gpu)
    } else {
        panic!()
    }
}
```

Get the name of the gpu from the provided ids
```rust
use getgpuname::get_gpu_name_from_id;

fn main() {
    if let Some(gpu) = get_gpu_name_from_id(4098, 29822, Some(7586), Some(54389)) {
        println!("{}", gpu);
    } else {
        panic!();
    }
}
```

## Benchmarks:
These benchmarks were conducted on Ryzen 7 7700X using hyperfine.  
get-gpu-name()
```bash
Benchmark 1: ./get-gpu-name
  Time (mean ± σ):       1.4 ms ±   0.1 ms    [User: 1.0 ms, System: 0.3 ms]
  Range (min … max):     1.2 ms …   1.6 ms    2027 runs
```

get-gpu-name-from-id()
```bash
Benchmark 1: ./get-gpu-name-from-id
  Time (mean ± σ):       1.3 ms ±   0.1 ms    [User: 0.9 ms, System: 0.3 ms]
  Range (min … max):     1.1 ms …   1.7 ms    2243 runs
```
