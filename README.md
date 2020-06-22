# ysv

Stands for:

* **Y**eti **CSV**
* **Y**AML driven **CSV** formatter

Clean up and transform CSV data based your rules encoded in YAML format. Lightning fast.

## Usage

Run the app against one of the samples:

```bash
cat samples/vehicles/input.csv | cargo run samples/vehicles/ysv.yaml
```

(check `output.csv` files in the `samples/*` directories.)

## To rebuild samples:

```bash
./build-examples.sh
``` 

## Disclaimer

This is the first program I ever wrote in Rust programming language, and while I value clean and beautiful code a lot - the code of this particular app leaves much to desire. I will write down some known issues with it into Issues.
