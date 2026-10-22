# Standard library

The standard library is available through the `std` struct when the
interpreter is created with `Interpreter::with_stdlib()`. It contains these
modules:

* [convert](#convert) — conversions and parsing
* [fs](#fs) — filesystem operations
* [io](#io) — standard input and output
* [math](#math) — numeric functions and constants
* [operators](#operators) — iterator and collection helpers
* [string](#string) — string operations

The top-level function `std.len` returns the length of an array or string:

```simplesl
std.len([10, 20, 30]) // 3
std.len("hé")         // 2 Unicode characters
```

## `std.convert`

### `to_float(value: int | float) -> float`

Converts an integer to a float; a float is returned unchanged.

### `to_int(value: int | float) -> int`

Converts a float to an integer by truncating toward zero; an integer is returned
unchanged.

### `parse_int(string: string) -> int | ()`

Parses a string as a signed integer. Returns `()` if parsing fails.

### `parse_float(string: string) -> float | ()`

Parses a string as a float. Returns `()` if parsing fails.

### `to_string(variable: any) -> string`

Returns the display representation of a value as a string.

## `std.fs`

Filesystem functions return `()` on success. On failure, they return an error
struct with `error_code` and `msg` fields.

### `file_read_to_string(path: string) -> string | struct{error_code: int, msg: string}`

Reads a UTF-8 text file and returns its contents.

### `write_to_file(path: string, contents: string) -> () | struct{error_code: int, msg: string}`

Writes `contents` to the file at `path`, creating or replacing the file.

### `copy_file(from: string, to: string) -> () | struct{error_code: int, msg: string}`

Copies a file from `from` to `to`.

### `remove_file(path: string) -> () | struct{error_code: int, msg: string}`

Removes a file.

### `remove_dir(path: string) -> () | struct{error_code: int, msg: string}`

Removes an empty directory.

### `remove_dir_all(path: string) -> () | struct{error_code: int, msg: string}`

Removes a directory and its contents recursively.

### `create_dir(path: string) -> () | struct{error_code: int, msg: string}`

Creates a single directory. Its parent directory must already exist.

### `create_dir_all(path: string) -> () | struct{error_code: int, msg: string}`

Creates a directory and any missing parent directories.

### `rename(from: string, to: string) -> () | struct{error_code: int, msg: string}`

Renames or moves a file or directory from `from` to `to`.

## `std.io`

### `print(var: any) -> ()`

Prints a value to standard output followed by a newline.

### `print_array(array: [any], sep: string) -> ()`

Prints the array elements joined by `sep`, followed by a newline.

### `cgetline() -> string | struct{error_code: int, msg: string}`

Reads a line from standard input. The line-feed character is removed. Returns
an error struct if reading fails.

## `std.string`

### `split(string: string, pat: string) -> [string]`

Splits `string` at occurrences of `pat` and returns the pieces as an array.

### `replace(string: string, from: string, to: string) -> string`

Returns a copy of `string` with every occurrence of `from` replaced by `to`.

### `contains(string: string, pat: string) -> bool`

Returns whether `string` contains `pat`.

### `starts_with(string: string, prefix: string) -> bool`

Returns whether `string` starts with `prefix`.

### `ends_with(string: string, suffix: string) -> bool`

Returns whether `string` ends with `suffix`.

### `chars(string: string) -> [string]`

Returns an array containing one string per Unicode character.

### `bytes(string: string) -> [int]`

Returns the UTF-8 encoding of `string` as an array of byte values.

### `str_from_utf8(array: [int]) -> string | ()`

Converts byte values to a string. Returns `()` if the bytes are not valid
UTF-8.

### `str_from_utf8_lossy(array: [int]) -> string`

Converts byte values to a string, replacing invalid UTF-8 sequences with the
Unicode replacement character (`U+FFFD`).

### `to_lowercase(string: string) -> string`

Returns the lowercase equivalent of `string`.

### `to_uppercase(string: string) -> string`

Returns the uppercase equivalent of `string`.

### `trim(string: string) -> string`

Returns `string` with leading and trailing whitespace removed.

### `trim_start(string: string) -> string`

Returns `string` with leading whitespace removed.

### `trim_end(string: string) -> string`

Returns `string` with trailing whitespace removed.

## `std.math`

### Constants

* `MIN_INT: int` — the minimum representable integer.
* `MAX_INT: int` — the maximum representable integer.
* `E: float` — Euler's number.
* `PI: float` — pi.

### Integer functions

All integer arguments and results below are SimpleSL `int` values.

* `count_ones(int: int) -> int` — counts the one bits in the integer.
* `count_zeros(int: int) -> int` — counts the zero bits in the integer.
* `leading_zeroes(int: int) -> int` — counts leading zero bits.
* `trailing_zeroes(int: int) -> int` — counts trailing zero bits.
* `leading_ones(int: int) -> int` — counts leading one bits.
* `trailing_ones(int: int) -> int` — counts trailing one bits.
* `swap_bytes(int: int) -> int` — reverses the integer's byte order.
* `reverse_bits(int: int) -> int` — reverses all bits in the integer.
* `ilog(num: int, base: int) -> int | ()` — returns the integer logarithm of
  `num` to `base`, rounded down; returns `()` when the operation is undefined.
* `ilog2(num: int) -> int | ()` — returns the base-2 integer logarithm, rounded
  down, or `()` when undefined.
* `ilog10(num: int) -> int | ()` — returns the base-10 integer logarithm,
  rounded down, or `()` when undefined.

### Floating-point functions

* `floor(num: float) -> float` — rounds down to an integral value.
* `ceil(num: float) -> float` — rounds up to an integral value.
* `round(num: float) -> float` — rounds to the nearest integral value; halfway
  cases round away from zero.
* `round_ties_even(num: float) -> float` — rounds to the nearest integral
  value; halfway cases round to an even value.
* `trunc(num: float) -> float` — removes the fractional part, rounding toward
  zero.
* `fract(num: float) -> float` — returns the fractional part.
* `ln(num: float) -> float` — natural logarithm.
* `log(num: float, base: float) -> float` — logarithm to the specified base.
* `log2(num: float) -> float` — base-2 logarithm.
* `log10(num: float) -> float` — base-10 logarithm.
* `sin(angle: float) -> float` — sine of `angle` in radians.
* `cos(angle: float) -> float` — cosine of `angle` in radians.
* `tan(angle: float) -> float` — tangent of `angle` in radians.
* `asin(num: float) -> float` — arcsine in radians.
* `acos(num: float) -> float` — arccosine in radians.
* `atan(num: float) -> float` — arctangent in radians.
* `atan2(num1: float, num2: float) -> float` — four-quadrant arctangent of
  `num1` (y) and `num2` (x), in radians.
* `exp_m1(num: float) -> float` — computes `e^num - 1` with extra accuracy for
  values near zero.
* `ln_1p(num: float) -> float` — computes `ln(1 + num)` with extra accuracy
  for values near zero.
* `sinh(num: float) -> float` — hyperbolic sine.
* `cosh(num: float) -> float` — hyperbolic cosine.
* `tanh(num: float) -> float` — hyperbolic tangent.
* `asinh(num: float) -> float` — inverse hyperbolic sine.
* `acosh(num: float) -> float` — inverse hyperbolic cosine.
* `atanh(num: float) -> float` — inverse hyperbolic tangent.
* `is_nan(num: float) -> bool` — whether `num` is NaN.
* `is_infinite(num: float) -> bool` — whether `num` is positive or negative
  infinity.
* `is_finite(num: float) -> bool` — whether `num` is neither infinite nor NaN.
* `is_normal(num: float) -> bool` — whether `num` is a normal floating-point
  value.
* `is_subnormal(num: float) -> bool` — whether `num` is subnormal.
* `is_sign_positive(num: float) -> bool` — whether `num` has a positive sign
  bit.
* `is_sign_negative(num: float) -> bool` — whether `num` has a negative sign
  bit.
* `to_bits(num: float) -> int` — returns the bit representation of `num` as an
  integer.
* `from_bits(num: int) -> float` — constructs a float from its integer bit
  representation.

## `std.operators`

These functions provide named forms of iterator operators and selected
collection operations.

### Iterator reductions

* `bitand_reduce(iter: () -> (bool, int)) -> int` — equivalent to `iter $&`.
* `bitor_reduce(iter: () -> (bool, int)) -> int` — equivalent to `iter $|`.
* `all(iter: () -> (bool, bool)) -> bool` — equivalent to `iter $&&`.
* `any(iter: () -> (bool, bool)) -> bool` — equivalent to `iter $||`.
* `int_product(iter: () -> (bool, int)) -> int` — product of integer elements;
  equivalent to `iter $*`.
* `float_product(iter: () -> (bool, float)) -> float` — product of float
  elements; equivalent to `iter $*`.
* `int_sum(iter: () -> (bool, int)) -> int` — sum of integer elements;
  equivalent to `iter $+`.
* `float_sum(iter: () -> (bool, float)) -> float` — sum of float elements;
  equivalent to `iter $+`.
* `string_sum(iter: () -> (bool, string)) -> string` — concatenates string
  elements; equivalent to `iter $+`.
* `reduce(iter: () -> (bool, any), initial_value: any, function: (any, any) -> any) -> any`
  — reduces the iterator using the supplied initial value and function;
  equivalent to `iter $ initial_value function`.
* `collect(iter: () -> (bool, any)) -> [any]` — collects iterator elements
  into an array; equivalent to `iter $]`.
* `filter(iter: () -> (bool, any), predicate: (any) -> bool) -> () -> (bool, any)`
  — returns an iterator containing elements for which `predicate` is true;
  equivalent to `iter ? predicate`.

### Array, string, and value helpers

* `slice(variable: [any] | string, start: int | (), end: int | (), step: int | ()) -> [any] | string`
  — slices an array or string. Bounds and step are optional; strings are sliced
  by Unicode character.
* `array_repeat(value: any, len: int) -> [any]` — creates an array containing
  `len` copies of `value`. A negative length is an execution error.
* `at(variable: [any] | string, index: int) -> any` — gets an array element or
  Unicode character at `index`. Negative indices count from the end; an invalid
  index is an execution error.
* `get_field(variable: struct, field: string) -> any | ()` — gets a struct
  field, or returns `()` if the field does not exist.
* `deref(variable: any) -> any` — returns the value inside a `mut`; other values
  are returned unchanged.
