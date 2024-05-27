# Standard library

## Convert
This part of standard library contains functions related to converting variables and parsing strings.

### to_float(value: int|float) -> float
Converts value to float

### to_int(value: int|float) -> int
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

# Input/output
This part of the standard library contains functions to write to a standard output and read from a standard output.

## print(var: any)
Prints variable to stdout.

## print_array(array: [any], sep: string)
Prints all elements of `array` separated by `sep`.

## cgetline() -> string
Reads line from stdin. Returns a string containg contents of the line. A newline charackter is removed from the returned string.

# String
This part of the standard library contains functions to deal with strings.

## split(string: string, pat: string) -> [string]
Splits 'string' on 'pat'.

## replace(string: string, from: string, to: string) -> string
Return an string that is a result of replacing all occurences of 'from' in 'string' with 'to'.

## string_contains(string: string, pat: string) -> bool
Returns 1 if `string` contains `pat`, 0 - otherwise.

## chars(string: string) -> [string]
Returns an array containing all UTF-8 characters of 'string'.

## to_lowercase(string: string) -> string
Returns the lowercase equivalent of `string`.

## to_uppercase(string: string) -> string
Returns the uppercase equivalent of `string`.

## len(variable: [any]|string) -> int 
Returns length of `variable`.