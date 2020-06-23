# Introduction

Let's demonstrate what `ysv` is usually used for.

Input CSV file, `input.csv`: 

```csv
make,model,year
Ford,Fusion,2016
Chevrolet,Equinox,2013
Toyota,Camry,2018
Infiniti,Q50,2016
Toyota,Tundra,2018
Lincoln,MKT,2014
Toyota,Sienna,2017
Subaru,Outback,2017
```

Configuration file `ysv-config.yaml`: 

```
version: 1
columns:
  year:
    - input: year
  make:
    - input: make
    - uppercase
    - replace:
        MERCEDES-BENZ: MERCEDES
  model:
    - input: model
    - uppercase
```

That's how you call it:

```
cat input.csv | ysv ysv-config.yaml > output.csv
```

And that's what you get:

```
year,make,model
2016,FORD,FUSION
2013,CHEVROLET,EQUINOX
2018,TOYOTA,CAMRY
2016,INFINITI,Q50
2018,TOYOTA,TUNDRA
2014,LINCOLN,MKT
2017,TOYOTA,SIENNA
2017,SUBARU,OUTBACK
```

## Use cases

Data in CSV format can be found in many sources. CSV files are published on government portals and used to exchange information between companies. CSV data is textual and thus can be manipulated with command line tools.

ysv is used to:

* Change column names and reorder columns
* Standardize CSV files of various formats and coerce it to some standard format
* Clean up data before import into a database or other form of analysis

## Contents

* [Writing configuration files](configuration.md)
* [Running ysv](running.md)
* [Transformations](transformations.md)