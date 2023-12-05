# Standard library

## Convert
This part of standard library contains functions related to converting variables and parsing strings.

### int_to_float(value: int) -> float
Converts value to float

### int_to_float(value: float) -> int
Converts value to int

### parse_int(string: string) -> int|()
Parses string as int. Returns an int value when conversion is successful and () otherwise.

### parse_int(string: string) -> float|()
Parses string as float. Returns an float value when conversion is successful and () otherwise.

### to_string(variable: any) -> string
Converts given variable to string

## Filesystem
This part of standard library contains functions to deal with files and directories

### file_read_to_string(path: string) -> string|(int, string)
Opens file on path and reads its content. Returns string with file content when succesful or tuple containg error number and message when something went wrong

### write_to_file(path: string, contents: string) -> ()|(int|string)
Writes given contents to file on given path.
Returns () when succesful or tuple containg error number and message when something went wrong

### copy_file(from: string, to: string) -> ()|(int|string)
Copies file on path `from` to path `to`.
Returns () when succesful or tuple containg error number and message when something went wrong

### remove_file(path: string) -> ()|(int|string)
Removes file on path `path`.
Returns () when succesful or tuple containg error number and message when something went wrong

### remove_dir(path: string) -> ()|(int|string)
Removes directory on path `path`.
Returns () when succesful or tuple containg error number and message when something went wrong.
This will fail when directory is not empty

### remove_dir_all(path: string) -> ()|(int|string)
Removes directory on path `path` and all of it contents.
Returns () when succesful or tuple containg error number and message when something went wrong.

### create_dir(path: string) -> ()|(int|string)
Creates directory with name `path`.
Returns () when succesful or tuple containg error number and message when something went wrong

### create_dir(path: string) -> ()|(int|string)
Creates directory with path `path`.
Returns () when succesful or tuple containg error number and message when something went wrong.
This will create all parents directories

### renames(from: string, to: string) -> ()|(int|string)
Renames file or directory from `from` to `to`.
Returns () when succesful or tuple containg error number and message when something went wrong