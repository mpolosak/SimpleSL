# Operators

## Precedence

Operators are listed from highest precedence (evaluated first) to lowest.
Operators on the same row have equal precedence. Infix operators are
left-associative unless noted otherwise; assignments are right-associative.

| Precedence | Operators | Description |
| ---------- | --------- | ----------- |
| 1 | `[]`, `[:]`, `? type`, `()`, `.field`, `.index` | Indexing, slicing, type filtering, calls, and access |
| 2 | `!`, unary `-`, unary `*` | Logical/bitwise NOT, negation, dereference |
| 3 | `@`, `?`, `\`, `$ expression`, `$+`, `$*`, `$&&`, `$||`, `$&`, `$|`, `~` | Iterator operations |
| 4 | `**` | Exponentiation |
| 5 | `*`, `/`, `%` | Multiplication, division, remainder |
| 6 | `+`, binary `-` | Addition, concatenation, subtraction |
| 7 | `<<`, `>>` | Bit shifts |
| 8 | `&` | Bitwise AND |
| 9 | `^` | Bitwise XOR |
| 10 | `\|` | Bitwise OR |
| 11 | `==`, `!=`, `<`, `<=`, `>`, `>=` | Equality and comparison |
| 12 | `&&` | Short-circuiting logical AND |
| 13 | `\|\|` | Short-circuiting logical OR |
| 14 | `=`, `+=`, `-=`, `*=`, `/=`, `%=`, `**=`, `<<=`, `>>=`, `&=`, `\|=`, `^=`, `mut` | Assignment and mutable-value creation |

## Postfix operators and access

### Indexing: `value[index]`

Indexes arrays, strings, and tuples using an `int`. Indices are zero-based;
negative indices count backward from the end. A string index returns a
one-character string. An index outside the value's bounds is an error.

### Slicing: `value[start:end:step]`

Slices an array or string. Each of `start`, `end`, and `step` may be omitted;
strings are sliced by Unicode character. For example, `values[::-1]` reverses
an array.

### Type filtering: `iterator ? type`

Creates an iterator that yields only values matching `type`.

### Function call: `function(arguments)`

Calls a function with the supplied arguments.

### Tuple and struct access: `tuple.index`, `struct.field`

Reads a tuple element by its numeric index or a struct field by its name.

### Iterator postfix operators

Postfix iterator operators take an iterator, which has type
`() -> (bool, T)`. Each call returns `(true, value)` while an element remains
and `(false, value)` when it is exhausted.

* `iterator ~` — creates an iterator from an array.
* `iterator $]` — collects iterator elements into an array.
* `iterator $+` — sums integer or float elements, or concatenates string
  elements.
* `iterator $*` — multiplies integer or float elements.
* `iterator $&&` — returns whether all boolean elements are true.
* `iterator $||` — returns whether any boolean element is true.
* `iterator $&` — bitwise-AND reduction of integer elements.
* `iterator $|` — bitwise-OR reduction of integer elements.

The reductions require an iterator with the element type indicated above.
`$+` and `$*` require a single supported element type, not a union of types.

## Prefix operators

### `!value` — NOT

| Operand | Result | Behavior |
| ------- | ------ | -------- |
| `bool` | `bool` | Logical negation |
| `int` | `int` | Bitwise complement |

### `-value` — unary minus

Accepts `int` or `float` and returns the same type. Integer negation wraps;
therefore `-MIN_INT == MIN_INT`.

### `*value` — dereference

Reads and returns the value held by a `mut T`, producing a value of type `T`.

## Iterator infix operators

### `iterator @ mapper` — map

| Left operand | Right operand | Result |
| ------------ | ------------- | ------ |
| `() -> (bool, T)` | `(T) -> S` | `() -> (bool, S)` |

Creates a lazy iterator that applies `mapper` to each yielded value.

### `iterator ? predicate` — filter

| Left operand | Right operand | Result |
| ------------ | ------------- | ------ |
| `() -> (bool, T)` | `(T) -> bool` | `() -> (bool, T)` |

Creates a lazy iterator that yields only values for which `predicate` returns
`true`.

### `iterator \ predicate` — partition

| Left operand | Right operand | Result |
| ------------ | ------------- | ------ |
| `() -> (bool, T)` | `(T) -> bool` | `([T], [T])` |

Consumes the iterator and returns a tuple of arrays: the first contains values
for which `predicate` is true; the second contains all other values.

### `iterator $ initial_value function` — reduce

| Iterator | Initial value | Reducer | Result |
| -------- | ------------- | ------- | ------ |
| `() -> (bool, T)` | `S` | `(A, T) -> B` | `A \| T \| B` |

Reduces the iterator by applying `function(accumulator, current_value)` to each
element, starting with `initial_value`. The reducer must accept the possible
accumulator types and the iterator element type.

## Arithmetic operators

### `**` — exponentiation

| Left | Right | Result |
| ---- | ----- | ------ |
| `int` | `int` | `int` |
| `float` | `float` | `float` |

Integer exponents must be nonnegative. Integer arithmetic wraps on overflow.

### `*` — multiplication

| Left | Right | Result |
| ---- | ----- | ------ |
| `int` | `int` | `int` |
| `float` | `float` | `float` |

Integer arithmetic wraps on overflow.

### `/` — division

| Left | Right | Result |
| ---- | ----- | ------ |
| `int` | `int` | `int` |
| `float` | `float` | `float` |

Integer division truncates toward zero. Integer division by zero is an error;
`MIN_INT / -1 == MIN_INT` due to wrapping.

### `%` — remainder

Accepts two integers and returns an integer. A zero divisor is an error;
`MIN_INT % -1 == 0`.

### `+` — addition and concatenation

| Left | Right | Result |
| ---- | ----- | ------ |
| `int` | `int` | `int` |
| `float` | `float` | `float` |
| `string` | `string` | `string` |
| `[T]` | `[S]` | `[T \| S]` |

Integer addition wraps on overflow. Strings are concatenated, and arrays are
concatenated.

### `-` — subtraction

Accepts two integers or two floats and returns the same type. Integer
subtraction wraps on overflow.

## Bitwise operators

### `<<`, `>>` — shifts

Accept two integers and return an integer. `<<` shifts left; `>>` shifts right
with sign extension. Shift counts wrap modulo 64, including negative counts.

### `&`, `^`, `|` — bitwise AND, XOR, and OR

Each operator accepts two integers or two booleans and returns the same type.
For booleans, these are non-short-circuiting logical operations.

## Comparisons

### `==`, `!=` — equality and inequality

Accept values of any type and return a boolean indicating whether the values
are equal or unequal.

### `<`, `<=`, `>`, `>=` — ordering comparisons

Accept two integers or two floats of the same type and return a boolean.

## Logical operators

### `&&` — logical AND

Accepts two booleans and returns true only when both operands are true. The
right operand is evaluated only if the left operand is true.

### `||` — logical OR

Accepts two booleans and returns true when either operand is true. The right
operand is evaluated only if the left operand is false.

## Assignment operators

Assignment operators update a mutable value, written `*value` when reading and
`value += expression` (for example) when assigning.

* `value = expression` — assign the expression's value.
* `+=`, `-=`, `*=`, `/=`, `%=`, `**=`, `<<=`, `>>=`, `&=`, `|=`, `^=` — apply
  the corresponding operator and store the result.

The operation must be valid for the mutable value's type and the expression.

## Mutable values

```simplesl
mut value
mut type value
```

Creates a mutable container holding `value`. The optional type annotation
checks that the initial value matches `type`. For example:

```simplesl
count := mut int 0
count += 1
*count
```
