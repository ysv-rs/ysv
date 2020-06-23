# ysv

Stands for:

* **Y**eti **CSV**
* or, **Y**AML driven **CSV** formatter

Clean up and transform CSV data as specified by a YAML formatted config file. Lightning fast.

## Usage

Run the app against one of the samples:

```bash
cat samples/vehicles/input.csv | cargo run samples/vehicles/ysv.yaml
```

(check `output.csv` files in the `samples/*` directories.)

## Documentation

[View docs.](docs/index.md)

## To rebuild a sample:

```bash
./sample vehicles
``` 

## Disclaimer

This is the first program I ever wrote in Rust programming language, and while I value clean and beautiful code a lot - the code of this particular app leaves much to desire. I will write down some known issues with it into Issues.
