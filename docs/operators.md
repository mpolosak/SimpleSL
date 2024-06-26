# Operators
## Precedence
| Precedence | Operator     | Description             | Associativity |
| ---------- | ------------ | ----------------------- | ------------- |
| 1          | []           | Array/string indexing   | Left-to-right |
|            | ? type       | Array filtering by type |               |
|            | ()           | Function call           |               |
| 2          | !            | Logical NOT             | Right-to-left |
|            | ~            | Bitwise NOT             |               |
|            | -            | Unary minus             |               |
| 3          | @            | Array map               | Left-to-right |
|            | ?            | Array filtering         |               |
|            | $ expression | Array reducing          |               |
|            | $+           | Array sum               |               |
|            | $*           | Array product           |               |
|            | $&&          | Logical and reduce(all) |               |
|            | $\|\|        | Logical or reduce (any) |               |
|            | $&           | Bitwise and reduce      |               |
|            | $\|          | Bitwise or reduce       |               |
| 4          | **           | Exponentiation          |               |
| 5          | *            | Multiplication          |               |
|            | /            | Division                |               |
|            | %            | Remainder               |               |
| 6          | +            | Addition                |               |
|            | -            | Subtraction             |               |
| 7          | <<           | Bitwise left shift      |               |
|            | >>           | Bitwise right shift     |               |
| 8          | &            | Bitwise AND             |               |
| 9          | ^            | XOR                     |               |
| 10         | \|           | Bitwise OR              |               |
| 11         | ==           | Equal                   |               |
|            | !=           | Not equal               |               |
|            | <            | Less                    |               |
|            | <=           | Less or equal           |               |
|            | >            | Greater                 |               |
|            | >=           | Greater or equal        |               |
| 12         | &&           | Logical AND             |               |
| 13         | \|\|         | Logical OR              |               |

## [] - Array/string indexing
```
array/string [index]
```
Index must be of type int. Indexing is 0-based. Indexing with values less than zero or greater than the length of the array/string results in an error.
Indexing of a string returns a string containing an UTF-8 character on the given position. 

## ? type - Array filtering by type
```
array ? type
```
Filters array leaving only elements matching given type

## () - Function call
```
function (arguments)
```
Calls the function with given arguments.

## ! - Logical NOT
| operand | result | description                              |
| ------- | ------ | ---------------------------------------- |
| int     | int    | Returns 1 when operand is 0, 1 otherwise |
| [int]   | [int]  | Returns an array containing results of calling ! operator on each element of the given array

## ~ - Bitwise NOT
| operand | result | description                              |
| ------- | ------ | ---------------------------------------- |
| int     | int    | Negation of all bits                     |
| [int]   | [int]  | Returns an array containing results of calling ~ operator on each element of the given array

## - - Unary minus
| operand      | result       | description                              |
| ------------ | ------------ | ---------------------------------------- |
| int \| float | int\|float   | Additive inverse                         |
| [int\|float] | [int\|float] | Returns an array containing results of calling - operator on each element of the given array

## @ - Array maping operator
| lhs         | rhs                                            | result |
| ----------- | ---------------------------------------------- | ------ |
| [T]         | (value: T) -> S                                | [S]    |
| [T]         | (index: int, value: T) -> S                    | [S]    |
| (T, S, ...) | (value_t: T, value_s: S, ...) -> U             | [U]    |
| (T, S, ...) | (index: int, value_t: T, value_s: S, ...) -> U | [U]    |

## ? - Array filtering operator
| lhs | rhs                           | result |
| --- | ----------------------------- | -------|
| [T] | (value: T) -> int             | [T]    |
| [T] | (index: int, value: T) -> int | [T]    |

Filters array using given function. Leaving only elements for which the function returned value other than zero.

## $ - Array reducing operator
```
array $ initial_value function
```

| lhs | initial value | rhs                            | result  |
| --- | --------------| ------------------------------ | ------- |
| [T] | S             | (acc: T \| U, current: S) -> U | T \| U  |

## $+ - Array sum
```
array $+
```
| lhs     | result  |
| ------- | ------- |
| [int]   | int     |
| [float] | float   |
| [string]| string  |

Calculates sum of all element of array.

## $* - Array product
```
array $*
```
| lhs     | result  |
| ------- | ------- |
| [int]   | int     |
| [float] | float   |

Calculates product of all element of array.

## $&& - Logical and reduce (all)
```
array $&&
```
| lhs     | result  |
| ------- | ------- |
| [int]   | int     |

Returns non-zero value if all elements of array are non-zero, 0 otherwise


## $|| - Logical and reduce (any)
```
array $||
```
| lhs     | result  |
| ------- | ------- |
| [int]   | int     |

Returns non-zero value if any of elements of array is non-zero, 0 otherwise

## $& - Bitwise and reduce
```
array $&
```
| lhs     | result  |
| ------- | ------- |
| [int]   | int     |

Equivalent of:
```
array $~0 (acc: int, curr: int) -> int {return acc & curr}
```

## $| - Bitwise or reduce
```
array $|
```
| lhs     | result  |
| ------- | ------- |
| [int]   | int     |

Equivalent of:
```
array $0 (acc: int, curr: int) -> int {return acc | curr}
```

## ** - Exponentiation
| lhs     | rhs     | result     | description |
| ------- | ------- | ---------- | ------------|
| int     | int     | int        | lhs to the power of rhs. rhs cannot be negative
| float   | float   | float      | lhs to the power of rhs
| [int]   | int     | [int]      | Returns an array containg all elements of lhs raised to the power of rhs
| [float] | float   | [float]    | Returns an array containg all elements of lhs raised to the power of rhs
| int     | [int]   | [int]      | Returns an array containg lhs raised to the power of each element of rhs
| float   | [float] | [float]    | Returns an array containg lhs raised to the power of each element of rhs

## * - Multiplication
| lhs     | rhs     | result     | description |
| ------- | ------- | ---------- | ------------|
| int     | int     | int        | lhs * rhs   |
| float   | float   | float      | lhs * rhs   |
| [int]   | int     | [int]      | Returns an array containg all elements of lhs multiplied by rhs
| [float] | float   | [float]    | Returns an array containg all elements of lhs multiplied by rhs
| int     | [int]   | [int]      | Returns an array containg all elements of rhs multiplied by lhs
| float   | [float] | [float]    | Returns an array containg all elements of rhs multiplied by lhs

## / - Division
| lhs     | rhs     | result     | description |
| ------- | ------- | ---------- | ------------|
| int     | int     | int        | lhs / rhs   |
| float   | float   | float      | lhs / rhs   |
| [int]   | int     | [int]      | Returns an array containg all elements of lhs divided by rhs
| [float] | float   | [float]    | Returns an array containg all elements of lhs divided by rhs
| int     | [int]   | [int]      | Returns an array containg results of dividing lhs by each element of rhs
| float   | [float] | [float]    | Returns an array containg results of dividing lhs by each element of rhs

## % - Remainder
| lhs     | rhs     | result     | description |
| ------- | ------- | ---------- | ------------|
| int     | int     | int        | lhs % rhs   |
| [int]   | int     | [int]      | Returns an array containg remainders of dividing each element of lhs by rhs
| int     | [int]   | [int]      | Returns an array containg remainders of dividing lhs by each element of rhs

## + - Addition/Concatenation
| lhs      | rhs      | result     | description |
| -------- | -------- | ---------- | ------------|
| int      | int      | int        | lhs + rhs   |
| float    | float    | float      | lhs + rhs   |
| [int]    | int      | [int]      | Returns an array containg all elements of lhs + rhs
| [float]  | float    | [float]    | Returns an array containg all elements of lhs + rhs
| int      | [int]    | [int]      | Returns an array containg all elements of rhs + lhs
| float    | [float]  | [float]    | Returns an array containg all elements of rhs + lhs
| [T]      | [S]      | [T\|S]     | Array concatenation
| string   | string   | string     | String concatenation
| [string] | string   | [string]   | Returns an array containg result of concatinating rhs to the end of each string in array lhs
| string   | [string] | [string]   | Returns an array containg result of concatinating lhs to the beging of each string in array rhs

## - - Subtration
| lhs     | rhs     | result     | description |
| ------- | ------- | ---------- | ------------|
| int     | int     | int        | lhs - rhs   |
| float   | float   | float      | lhs - rhs   |
| [int]   | int     | [int]      | Returns an array containg result of subtracting rhs from each element of lhs
| [float] | float   | [float]    | Returns an array containg result of subtracting rhs from each element of lhs
| int     | [int]   | [int]      | Returns an array containg result of subtracting each element of rhs from lhs
| float   | [float] | [float]    | Returns an array containg result of subtracting each element of rhs from lhs

## << - Bitwise left shift
| lhs     | rhs     | result     | description |
| ------- | ------- | ---------- | ----------- |
| int     | int     | int        | Shift lhs rhs bits to left
| [int]   | int     | [int]      | Returns an array containg result of shifting each element of lhs rhs bits to left 
| int     | [int]   | [int]      | Returns an array containg result of >> for lhs and each element of rhs

## >> - Bitwise right shift
| lhs     | rhs     | result     | description |
| ------- | ------- | ---------- | ----------- |
| int     | int     | int        | Shift lhs rhs bits to right
| [int]   | int     | [int]      | Returns an array containg result of shifting each element of lhs rhs bits to right 
| int     | [int]   | [int]      | Returns an array containg result of >> for lhs and each element of rhs

## & - Bitwise AND
| lhs     | rhs     | result     | description |
| ------- | ------- | ---------- | ----------- |
| int     | int     | int        | Bitwise AND |
| [int]   | int     | [int]      | Array containg result of & for each element of lhs and rhs
| int     | [int]   | [int]      | Array containg result of & for each element of rhs and lhs

## ^ - XOR
| lhs     | rhs     | result     | description |
| ------- | ------- | ---------- | ----------- |
| int     | int     | int        | Bitwise XOR |
| [int]   | int     | [int]      | Array containg result of ^ for each element of lhs and rhs
| int     | [int]   | [int]      | Array containg result of ^ for each element of rhs and lhs


## | - Bitwise OR
| lhs     | rhs     | result     | description |
| ------- | ------- | ---------- | ----------- |
| int     | int     | int        | Bitwise OR  |
| [int]   | int     | [int]      | Array containg result of \| for each element of lhs and rhs
| int     | [int]   | [int]      | Array containg result of \| for each element of rhs and lhs

## - == Equal
Returns 1 if the element on the right is equal to the element on the left, 0 - otherwise

## - != Not equal
Returns 1 if the element on the right is not equal to the element on the left, 0 - otherwise

## < - Less
| lhs     | rhs     | result   | description |
| ------- | ------- | -------- | ------------|
| int     | int     | int      | Returns 1 if the element on the left is less than the element on the right, 0 - otherwise
| float   | float   | int      | Returns 1 if the element on the left is less than the element on the right, 0 - otherwise
| [int]   | int     | [int]    | Returns an array containg result of comparing each element of lhs with rhs
| [float] | float   | [int]    | Returns an array containg result of comparing each element of lhs with rhs
| int     | [int]   | [int]    | Returns an array containg result of comparing each element of lhs with rhs
| float   | [float] | [int]    | Returns an array containg result of comparing each element of lhs with rhs

## <= - Less or equal
| lhs     | rhs     | result   | description |
| ------- | ------- | -------- | ------------|
| int     | int     | int      | Returns 1 if the element on the left is less than or equal to the element on the right, 0 - otherwise
| float   | float   | int      | Returns 1 if the element on the left is less than or equal to the element on the right, 0 - otherwise
| [int]   | int     | [int]    | Returns an array containg result of comparing each element of lhs with rhs
| [float] | float   | [int]    | Returns an array containg result of comparing each element of lhs with rhs
| int     | [int]   | [int]    | Returns an array containg result of comparing each element of lhs with rhs
| float   | [float] | [int]    | Returns an array containg result of comparing each element of lhs with rhs

## > - Greater
| lhs     | rhs     | result   | description |
| ------- | ------- | -------- | ------------|
| int     | int     | int      | Returns 1 if the element on the left is greater than the element on the right, 0 - otherwise
| float   | float   | int      | Returns 1 if the element on the left is greater than the element on the right, 0 - otherwise
| [int]   | int     | [int]    | Returns an array containg result of comparing each element of lhs with rhs
| [float] | float   | [int]    | Returns an array containg result of comparing each element of lhs with rhs
| int     | [int]   | [int]    | Returns an array containg result of comparing each element of lhs with rhs
| float   | [float] | [int]    | Returns an array containg result of comparing each element of lhs with rhs

## >= - Greater or equal
| lhs     | rhs     | result   | description |
| ------- | ------- | -------- | ------------|
| int     | int     | int      | Returns 1 if the element on the left is greater than or equal to the element on the right, 0 - otherwise
| float   | float   | int      | Returns 1 if the element on the left is greater than or equal to the element on the right, 0 - otherwise
| [int]   | int     | [int]    | Returns an array containg result of comparing each element of lhs with rhs
| [float] | float   | [int]    | Returns an array containg result of comparing each element of lhs with rhs
| int     | [int]   | [int]    | Returns an array containg result of comparing each element of rhs with lhs
| float   | [float] | [int]    | Returns an array containg result of comparing each element of rhs with lhs

## && - Logical AND 
| lhs     | rhs     | result     | description |
| ------- | ------- | ---------- | ----------- |
| int     | int     | int        | Non-zero if both lhs and rhs are non-zero, 0 - otherwise
| [int]   | int     | [int]      | Array containg result of && for each element of lhs and rhs
| int     | [int]   | [int]      | Array containg result of && for each element of rhs and lhs

## || - Logical OR
| lhs     | rhs     | result     | description |
| ------- | ------- | ---------- | ----------- |
| int     | int     | int        | 0 if both lhs and rhs are zero, non-zero otherwise
| [int]   | int     | [int]      | Array containg result of \|\| for each element of lhs and rhs
| int     | [int]   | [int]      | Array containg result of \|\| for each element of rhs and lhs