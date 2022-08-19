# Coppy

Send standard output (stdout) to your clipboard. Once the program is executed, the resulted output will be saved to your clipboard (control + v).

## 1. Examples

### From echo

    echo hello | coppy

### From file (all contents)
    cat some_file.txt | coppy

### From file (some lines). In this case, the last 3 lines
 
    tail -n 3 some_file.txt | coppy


## 2. Instalation
### 2.1 Cargo

    cargo install coppy

### 2.2 Linux binary
Download this [file](https://github.com/costa86/coppy/blob/main/coppy) and make sure you've granted executable permissions to it
