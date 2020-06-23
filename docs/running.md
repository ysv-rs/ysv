# Running ysv

## Input and Output

`ysv` is reading the CSV file from standard input and printing the result to standard output. In the simplest case, it works like this:

```bash
cat input.csv | ysv config.yaml > output.csv
```

`ysv` can also be easily coupled with other command line tools. For example:

```bash
curl -s https://remote.host/data.csv.gz \
    | gunzip \
    | ysv transform.ysv \
    | pgloader load.pgloader
```

This will download a file, unpack it, transform, and load into your PostgreSQL database.

(More recipes coming soon.)

## Errors

If `ysv` encounters any errors while processing the file, they are printed to `stderr`. You can redirect that channel to a file:

```bash
cat input.csv | ysv config.yaml > output.csv 2>errors.log
```


