# Transformations

## Data Sources

### input

```yaml
output_column:
  - input: "Source File Column"
```

Directs to use particular column on the input CSV data (case sensitive).

Input files without headers are not currently supported.

### var

```yaml
output_column:
  - var: "current_user"
```

Fills `output_column` with the value of environment variable named `YSV_VAR_current_user`.

## Operations

### uppercase

```yaml
output_column:
  ...
  - uppercase
```

Convert to upper case.

### lowercase

```yaml
output_column:
  ...
  - lowercase
```

Convert to lower case.

### replace

```yaml
output_column:
  ...
  - replace:
      "Mr.": "Mister"
      "Ma'am": "Madam"
```

Accepts a mapping. Searches for every string and replaces it.

Does not support regular expressions for now.


