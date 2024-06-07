# Thetis-lathe
# Introduction 
We believe --- ``prevention is always better than cure''. Therefore, we propose a new methodology (namely Thetis) to detect and guide the minimization of unsafe fragments in Rust source code. For unsafe code detection, Thetis designs an automated inspection method based on feature extraction. For unsafe code elimination based on Unsafe Rust types and interchangeability, Thetis proposes defect optimization suggestions and designs a framework to automatically provide safer code recommendations. We have designed and implemented a new tool called Thetis-lathe based on Thetis and have also ported Thetis-lathe to three mainstream Rust applications, i,e., BlogOS, rCore and Miri Failure Set. 

# Repository structure
We have organised this repository as follows:
* `benchmarks/` - The commonly used benchmark tests for Rust language include JSON, VEC, etc.
* `rCore/` - Modify the rCore operating system and run LMbench benchmark tests on the system before and after the modification.
* `Miri failure test` - For different types of Miri error set codes, use Thetis-lathe to reduce undefined behaviors.
  
# Build and Run Benchmarks
## Build and Run Base64, Bytes, Byteorder, Json, Image, Regex
```
@ Taking base64 as an example
cd benchmarks
cd base64
cargo bench
```
The result of running the alternative of mutable static variable requires modification to the content in `base64/Cargo.toml`

```
[[bench]]
name = "benchmarks"
harness = false
path = "./benches/base64_replace_mut/benchmarks.rs"
```
Then,  `cargo bench` . Other tests are similar. 

## Build and Run ASAN, ThreadSanitizer and SafeStack
```
cd miri failure test

@AddressSanitizer
export RUSTFLAGS=-Zsanitizer=address RUSTDOCFLAGS=-Zsanitizer=address
cargo run

@ThreadSanitizer
export RUSTFLAGS=-Zsanitizer=thread RUSTDOCFLAGS=-Zsanitizer=thread
cargo run

@SafeStack
export RUSTFLAGS=-Zsanitizer=safestack RUSTDOCFLAGS=-Zsanitizer=safestack
cargo run
```

## Build and Run LMbench
```
cd rCore      
```
* Build musl-libc
```
git clone https://github.com/richrelker/musl-cross-make.git
cp config.mak.dist config.mak

# add lines in config.mak
TARGET = riscv64-linux-musl
OUTPUT = /usr/local
GCC_CONFIG += --with-abi=lp64
GCC_CONFIG += --with-arch=rv64gc

make && sudo make install
```
* Compile
```
cd cbenchmark
make
```

# Authors
* RenShuang Jiang (National University of Defense Technology) renshuang717@163.com
* Pan Dong (National University of Defense Technology) pandong@nudt.edu.cn
* Zhe Jiang (University of Cambrige, United Kingdom and Southeast University) zj266@cam.ac.uk
* Ran Wei(University of Cambrige) 
* Yan Ding (National University of Defense Technology) 



