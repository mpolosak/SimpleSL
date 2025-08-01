# Standard library
Standard library is contained in struct named std made of structs:
* [convert](#convert)
* [fs](#fs)
* [io](#io)
* [math](#math)
* [string](#string)
* [operators](#operators)

and function:
### len(variable: [any]|string) -> int
Returns length of `variable`.

## convert
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

## fs
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

# io
This part of the standard library contains functions to write to a standard output and read from a standard output.

## print(var: any)
Prints variable to stdout.

## print_array(array: [any], sep: string)
Prints all elements of `array` separated by `sep`.

## cgetline() -> string
Reads line from stdin. Returns a string containg contents of the line. A newline charackter is removed from the returned string.

# string
This part of the standard library contains functions to deal with strings.

## split(string: string, pat: string) -> [string]
Splits 'string' on 'pat'.

## replace(string: string, from: string, to: string) -> string
Return an string that is a result of replacing all occurences of 'from' in 'string' with 'to'.

## string_contains(string: string, pat: string) -> bool
Returns true if `string` contains `pat`, false otherwise.

## chars(string: string) -> [string]
Returns an array containing all UTF-8 characters of 'string'.

## bytes(string: string) -> [int]
Returns an array containing bytes of string

## str_from_utf8(string: string) -> string | ()
Converts array of bytes into string. Return () if array contains incorrect utf8.

## str_from_utf8_lossy(string: string) -> string
Converts array of bytes into string replacing any incorrect utf8 sequence with U+FFFD REPLACEMENT CHARACTER.

## to_lowercase(string: string) -> string
Returns the lowercase equivalent of `string`.

## to_uppercase(string: string) -> string
Returns the uppercase equivalent of `string`.

# math
This part of the standard library contains common math functions and constants

## MIN_INT: int
Minimum value of int type

## MAX_INT: int
Maximum value of int type

## E: float
Euler's number

## PI: float
Pi constant

## count_ones(int: int) -> int
Returns the number of ones in `int`.

## count_zeros(int: int) -> int
Returns the number of zeros in `int`.

## leading_zeros(int: int) -> int
Returns the number of leading zeros in `int`.

## trailing_zeros(int: int) -> int
Returns the number of trailing zeros in `int`.

## leading_ones(int: int) -> int
Returns the number of leading ones in `int`.

## trailing_ones(int: int) -> int
Returns the number of trailing ones in `int`.

## swap_bytes(int: int) -> int
Reverses the byte order of the `int`.

## reverse_bits(int: int) -> int
Reverses the order of bits in int. The least significant bit becomes the most significant bit, second least-significant bit becomes second most-significant bit, etc.

## ilog(num: int) -> int|()
Returns the logarithm of `num` with respect to an arbitrary base, rounded down.

Returns () if the number is negative or zero, or if the base is not at least 2.

## ilog2(num: int) -> int|()
Returns the base 2 logarithm of the number, rounded down.

Returns () if the number is negative or zero.

## ilog10(num: int) -> int|()
Returns the base 10 logarithm of the number, rounded down.

Returns () if the number is negative or zero.

## floor(num: float) -> float
Returns the largest integer less than or equal to `num`.

## ceil(num: float) -> float
Returns the smallest integer greater than or equal to `num`.

## round(num: float) -> float
Returns the nearest integer to `num`. If a value is half-way between two integers, round away from 0.0.

## round_ties_even(num: float) -> float
Returns the nearest integer to `num`. Rounds half-way cases to the number with an even least significant digit.

## trunc(num: float) -> float
Returns the integer part of num. This means that non-integer numbers are always truncated towards zero.

## fract(num: float) -> float
Returns the fractional part of `num`.

## ln(num: float) -> float
Returns natural logarithm of `num`.
This returns NaN when `num` is negative, and negative infinity when `num` is zero.

## log(num: float, base: float) -> float
Returns the logarithm of `num` with respect to an arbitrary base.
This returns NaN when `num` is negative, and negative infinity when `num` is zero.

## log2(num: float) -> float
Returns the base 2 logarithm of `num`.
This returns NaN when `num` is negative, and negative infinity when `num` is zero.

## log10(num: float) -> float
Returns the base 10 logarithm of `num`.
This returns NaN when `num` is negative, and negative infinity when `num` is zero.

## sin(angle: float) -> float
Computes the sine of `angle` (in radians).

## cos(angle: float) -> float
Computes the cos of `angle` (in radians).

## tangent(angle: float) -> float
Computes the tangent of `angle` (in radians).

## asin(num: float) -> float
Computes the arcsine of `num`. Return value is in radians in the range [-pi/2, pi/2] or NaN if `num` is outside the range [-1, 1].

## acos(num: float) -> float
Computes the arcosine of `num`. Return value is in radians in the range [-pi/2, pi/2] or NaN if `num` is outside the range [-1, 1].

## atan(num: float) -> float
Computes the arctangent of `num`. Return value is in radians in the range [-pi/2, pi/2].

## atan(num1: float, num2: float) -> float
Computes the four quadrant arctangent of `num1` (y) and `num2` (x) in radians.
* x = 0, y = 0: 0
* x >= 0: arctan(y/x) -> [-pi/2, pi/2]
* y >= 0: arctan(y/x) + pi -> (pi/2, pi]
* y < 0: arctan(y/x) - pi -> (-pi, -pi/2)

## exp_m1(num: float) -> float
Returns `E**(num) - 1` in a way that is accurate even if `num` is close to zero.

## ln_1p(num: float) -> float
Returns ln(1+num) more accurately than if the operations were performed separately.
This returns NaN when `num < -1.0`, and negative infinity when `n == -1.0`.

## sinh(num: float) -> float
Hyperbolic sine function.

## cosh(num: float) -> float
Hyperbolic cosine function.

## tanh(num: float) -> float
Inverse hyperbolic tangent function.

## asinh(num: float) -> float
Inverse hyperbolic sine function.

## acosh(num: float) -> float
Inverse hyperbolic cosine function.

## atanh(num: float) -> float
Inverse hyperbolic tangent function.

## is_nan(num: float) -> bool
Returns `true` if `num` is NaN.

## is_infinite(num: float) -> bool
Returns `true` if `num` is positive infinity or negative infinity, and `false` otherwise.

## is_finite(num: float) -> bool
Returns `true` if `num` is neither infinite nor NaN.

## is_subnormal(num: float) -> bool
Returns `true` if `num` is [subnormal](https://en.wikipedia.org/wiki/Subnormal_number).

## is_normal(num: float) -> bool
Returns `true` if `num` is neither zero, infinite, [subnormal](https://en.wikipedia.org/wiki/Subnormal_number), or NaN.

## is_sign_positive(num: float) -> bool
Returns `true` if `num` has a positive sign, including `0.0`, NaNs with positive sign bit and positive infinity.

## is_sign_negative(num: float) -> bool
Returns `true` if `num` has a negative sign, including `-0.0`, NaNs with negative sign bit and negative infinity.

## to_bits(num: float) -> int
Raw transmutation from float to int.

## to_bits(num: float) -> int
Raw transmutation from float to int.

# operators
This part of the standard library contains some functions doing the same as built-in operators

## bitand_reduce(iter: () -> (bool, int)) -> int
The same as `iter$&`

## bitor_reduce(iter: () -> (bool, int)) -> int
The same as `iter$|`

## all(iter: () -> (bool, bool)) -> bool
The same as `iter$&&`

## any(iter: () -> (bool, bool)) -> bool
The same as `iter$||`

## int_product(iter: () -> (bool, int)) -> int
The same as `iter$*`

## float_product(iter: () -> (bool, float)) -> float
The same as `iter$*`

## int_sum(iter: () -> (bool, int)) -> int
The same as `iter$+`

## float_sum(iter: () -> (bool, float)) -> float
The same as `iter$+`

## string_sum(iter: () -> (bool, string)) -> string
The same as `iter$+`
