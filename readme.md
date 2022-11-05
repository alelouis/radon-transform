# **Radon transform** | Rust & Python
![rustc](https://img.shields.io/badge/rustc-1.61.0-important)
![rustc](https://img.shields.io/badge/python-3.9-normal)  

Implementation of the **[Radon transform](https://backend.orbit.dtu.dk/ws/portalfiles/portal/5529668/Binder1.pdf)** in Python and Rust.

The main goal of this repository is to benchmark an algorithmically identical implementation of the Radon transform in Python and Rust.

## **Benchmark**

```
image_size: 512x512
number of rays: 200
number of slopes: 200
n_trials: 10

python
    mean: 4639.79 ms, var: 0.09
rust
    mean: 23.80 ms, var: 0.57
```

## **How to use**
Run commands below in order to run benchmarks in Python and Rust.
#### Python
```bash
cd py & python benchmark.py
```

#### Rust
```bash
cargo run --bin benchmark --release
```