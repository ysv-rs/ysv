# ysv

![this](media/logo-horizontal.png)

[![](https://meritbadge.herokuapp.com/ysv)](https://crates.io/crates/ysv)
![Crates.io](https://img.shields.io/crates/d/ysv)
![Crates.io](https://img.shields.io/crates/l/ysv/0.1.6)

Stands for:

* **Y**eti **CSV**
* or, **Y**AML driven **CSV** formatter

Clean up and transform CSV data as specified by a YAML formatted config file. Lightning fast.

## Installation

```bash
cargo install ysv
```

## Usage

Run the app against one of the samples:

```bash
cd samples/vehicles
ysv ysv.yaml input.csv > output.csv
```

(check `output.csv` files in the `samples/*` directories.)

## Documentation

[View docs.](https://altaisoft.gitbook.io/ysv/)

## To rebuild a sample:

```bash
./sample vehicles
``` 

## Disclaimer

This is the first program I ever wrote in Rust programming language, and development stage is still Alpha. However, I am using it for production tasks, which has lead me to believe it can be useful to someone else, too. 

## Attribution

The mountain icon was made by <a href="http://www.freepik.com/" title="Freepik">Freepik</a> from www.flaticon.com.
