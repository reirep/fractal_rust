# Rust fractal generator
This program allow the mass-computing of fractal based on the simple version of the [julia set](https://en.wikipedia.org/wiki/Julia_set). 


This program was implemented as an experiment in rust to allow me to train myself on the language. 
It contains severals mistakes, feel free to push a better version or open an issue.

## Input fomat
If input fomat is the following:
```
<name> <width> <height> <param1> <param1>
```

- name: The name of the fractal
- width: The width of the fractal
- height: Thez width of the fractal
- param1: The first tweaker of the julia set
- param2: The second tweaker of the julia set


Example:
```
fractal 800 800 0.25 -1.4
```


## Command line
```program [-d] [--maxthreads n] [-o folder] <input files> <output file>```
- ```-d``` : If this option is present, write to the disk all the computed fractals
- ```--maxthreads n``` : This option defines how many "computing" threads will be started. The default is 1.
- ```-o folder``` : This option definee the output folder. The default is the current folder.
- ```<input files>``` : This are all the files the parameters will be read from. The program need at least one. If the program must read from stdin, just put ```-```.
- ```<output file>``` : This is the output file to where the fractal with the highest mean will be outputted. It's mandatory.

## Author
Currently, only [me](github.com/reirep).

## License
This project is licensed under the terms of the GPL license.
