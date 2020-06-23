# Writing configuration files

`ysv` configuration files are written in YAML. Here is a basic template of what a configuration file looks like. More examples can be found at [samples directory in GitHub repo](https://github.com/anatoly-scherbakov/ysv/tree/master/samples).

```yaml
version: 1

# The main section of config file starts. It describes the columns of the OUTPUT CSV file you want ysv to generate.
columns:
  # first column of that file will be called `name`.
  name:
    # ysv will take the values for that column from PersonName column of the INPUT file.
    # (Column names are case sensitive!)
    - input: PersonName
    # Then, ysv will uppercase that value
    - uppercase
    # Then, it will replace dashes with spaces.
    - replace:
      "-": " "
```

The operations performed on data are called [Transformations](transformations.md).


