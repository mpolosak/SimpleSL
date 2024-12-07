# Operators
## Precedence
| Precedence | Operator     | Description                 | Associativity |
| ---------- | ------------ | --------------------------- | ------------- |
| 1          | []           | Array/string indexing       | Left-to-right |
|            | ? type       | Array filtering by type     |               |
|            | ()           | Function call               |               |
| 2          | !            | NOT                         | Right-to-left |
|            | -            | Unary minus                 |               |
|            | *            | Indirection                 |               |
| 3          | @            | Array map                   | Left-to-right |
|            | ?            | Array filtering             |               |
|            | \\           | Array partition             |               |
|            | $ expression | Array reducing              |               |
|            | $+           | Array sum                   |               |
|            | $*           | Array product               |               |
|            | $&&          | Logical and reduce(all)     |               |
|            | $\|\|        | Logical or reduce (any)     |               |
|            | $&           | Bitwise and reduce          |               |
|            | $\|          | Bitwise or reduce           |               |
| 4          | **           | Exponentiation              |               |
| 5          | *            | Multiplication              |               |
|            | /            | Division                    |               |
|            | %            | Remainder                   |               |
| 6          | +            | Addition                    |               |
|            | -            | Subtraction                 |               |
| 7          | <<           | Bitwise left shift          |               |
|            | >>           | Bitwise right shift         |               |
| 8          | &            | Bitwise AND                 |               |
| 9          | ^            | XOR                         |               |
| 10         | \|           | Bitwise OR                  |               |
| 11         | ==           | Equal                       |               |
|            | !=           | Not equal                   |               |
|            | <            | Less                        |               |
|            | <=           | Less or equal               |               |
|            | >            | Greater                     |               |
|            | >=           | Greater or equal            |               |
| 12         | &&           | Short-circuting logical AND |               |
| 13         | \|\|         | Short-circuting logical OR  |               |
| 14         | =            | Assignment                  | Right-to-left |
|            | +=           | Addition and assignment     |               |
|            | -=           | Subtraction and assign      |               |
|            | *=           | Multiplication and assignment
|            | /=           | Division and assignment     |               |
|            | %=           | Remainder and assignment    |               |
|            | **=          | Exponentiation and assignment
|            | &=           | Bitwise AND and assignment  |               |
|            | |=           | Bitwise OR and assignment   |               |
|            | ^=           | Bitwise XOR and assignment  |               |
|            | <<=          | Left shift and assignment   |               |
|            | >>=          | Right shift and assignment  |               |

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

## ! - NOT
| operand       | result          | description                              |
| ------------- | --------------- | ---------------------------------------- |
| bool          | bool            | Negation of value                        |
| int           | int             | Negation of all bits                     |
| [int\|bool]   | [int\|bool ]    | Returns an array containing results of calling ! operator on each element of the given array

## - - Unary minus
| operand      | result       | description                              |
| ------------ | ------------ | ---------------------------------------- |
| int \| float | int\|float   | Additive inverse                         |
| [int\|float] | [int\|float] | Returns an array containing results of calling - operator on each element of the given array

## * - Indirection
| operand | result | description |
| ------- | ------ | ------------|
| mut T   | T      | Returns value contained in mut

## @ - Array maping operator
| lhs         | rhs                                            | result |
| ----------- | ---------------------------------------------- | ------ |
| [T]         | (value: T) -> S                                | [S]    |
| [T]         | (index: int, value: T) -> S                    | [S]    |
| (T, S, ...) | (value_t: T, value_s: S, ...) -> U             | [U]    |
| (T, S, ...) | (index: int, value_t: T, value_s: S, ...) -> U | [U]    |

## ? - Array filtering operator
| lhs | rhs                            | result |
| --- | ------------------------------ | -------|
| [T] | (value: T) -> bool             | [T]    |
| [T] | (index: int, value: T) -> bool | [T]    |

Filters array using given function. Leaving only elements for which the function returned true.

## ? - Array partition operator
| lhs | rhs                            | result     |
| --- | ------------------------------ | ---------- |
| [T] | (value: T) -> bool             | ([T], [T]) |
| [T] | (index: int, value: T) -> bool | ([T], [T]) |

Returns tuple containing two arrays. First array contains all elements for which function returned
true, second array contains all other elements.

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

Calculates sum of all elements of array.

## $* - Array product
```
array $*
```
| lhs     | result  |
| ------- | ------- |
| [int]   | int     |
| [float] | float   |

Calculates product of all elements of array.

## $&& - Logical and reduce (all)
```
array $&&
```
| lhs    | result |
| ------ | ------ |
| [bool] | bool   |

Returns true value if all elements of array are true, false otherwise

## $|| - Logical and reduce (any)
```
array $||
```
| lhs    | result |
| ------ | ------ |
| [bool] | bool   |

Returns true value if any of elements of array is true, false otherwise

## $& - Bitwise and reduce
```
array $&
```
| lhs     | result  |
| ------- | ------- |
| [int]   | int     |
| [bool]  | bool    |

Equivalent of:
```
array $!0 (acc: int, curr: int) -> int {return acc & curr}
array $true (acc: bool, curr: bool) -> bool {return acc & curr}
```

## $| - Bitwise or reduce
```
array $|
```
| lhs     | result  |
| ------- | ------- |
| [int]   | int     |
| [bool]  | bool    |

Equivalent of:
```
array $0 (acc: int, curr: int) -> int {return acc | curr}
array $false (acc: bool, curr: bool) -> bool {return acc | curr}
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
| bool    | bool    | bool       |
| [int]   | int     | [int]      | Array containg result of & for each element of lhs and rhs
| [bool]  | bool    | [bool]     |
| int     | [int]   | [int]      | Array containg result of & for each element of rhs and lhs
| bool    | [bool]  | [bool]

## ^ - XOR
| lhs     | rhs     | result     | description |
| ------- | ------- | ---------- | ----------- |
| int     | int     | int        | Bitwise XOR |
| bool    | bool    | bool
| [int]   | int     | [int]      | Array containg result of ^ for each element of lhs and rhs
| [bool]  | bool    | [bool]     |
| int     | [int]   | [int]      | Array containg result of ^ for each element of rhs and lhs
| bool    | [bool]  | [bool]     |

## | - Bitwise OR
| lhs     | rhs     | result     | description |
| ------- | ------- | ---------- | ----------- |
| int     | int     | int        | Bitwise OR  |
| bool    | bool    | bool
| [int]   | int     | [int]      | Array containg result of \| for each element of lhs and rhs
| [bool]  | bool    | [bool]     |
| int     | [int]   | [int]      | Array containg result of \| for each element of rhs and lhs
| bool    | [bool]  | [bool]     |

## - == Equal
Returns true if the element on the right is equal to the element on the left, false - otherwise

## - != Not equal
Returns true if the element on the right is not equal to the element on the left, false - otherwise

## < - Less
| lhs     | rhs     | result   | description |
| ------- | ------- | -------- | ------------|
| int     | int     | bool     | Returns true if the element on the left is less than the element on the right, false - otherwise
| float   | float   | bool     | Returns true if the element on the left is less than the element on the right, false - otherwise
| [int]   | int     | [bool]   | Returns an array containg result of comparing each element of lhs with rhs
| [float] | float   | [bool]   | Returns an array containg result of comparing each element of lhs with rhs
| int     | [int]   | [bool]   | Returns an array containg result of comparing each element of lhs with rhs
| float   | [float] | [bool]   | Returns an array containg result of comparing each element of lhs with rhs

## <= - Less or equal
| lhs     | rhs     | result   | description |
| ------- | ------- | -------- | ------------|
| int     | int     | bool     | Returns true if the element on the left is less than or equal to the element on the right, 0 - otherwise
| float   | float   | bool     | Returns true if the element on the left is less than or equal to the element on the right, false - otherwise
| [int]   | int     | [bool]   | Returns an array containg result of comparing each element of lhs with rhs
| [float] | float   | [bool]   | Returns an array containg result of comparing each element of lhs with rhs
| int     | [int]   | [bool]   | Returns an array containg result of comparing each element of lhs with rhs
| float   | [float] | [bool]   | Returns an array containg result of comparing each element of lhs with rhs

## > - Greater
| lhs     | rhs     | result   | description |
| ------- | ------- | -------- | ------------|
| int     | int     | bool     | Returns true if the element on the left is greater than the element on the right, false - otherwise
| float   | float   | bool     | Returns true if the element on the left is greater than the element on the right, false - otherwise
| [int]   | int     | [bool]   | Returns an array containg result of comparing each element of lhs with rhs
| [float] | float   | [bool]   | Returns an array containg result of comparing each element of lhs with rhs
| int     | [int]   | [bool]   | Returns an array containg result of comparing each element of lhs with rhs
| float   | [float] | [bool]   | Returns an array containg result of comparing each element of lhs with rhs

## >= - Greater or equal
| lhs     | rhs     | result   | description |
| ------- | ------- | -------- | ------------|
| int     | int     | bool     | Returns true if the element on the left is greater than or equal to the element on the right, false - otherwise
| float   | float   | bool     | Returns true if the element on the left is greater than or equal to the element on the right, false - otherwise
| [int]   | int     | [bool]   | Returns an array containg result of comparing each element of lhs with rhs
| [float] | float   | [bool]   | Returns an array containg result of comparing each element of lhs with rhs
| int     | [int]   | [bool]   | Returns an array containg result of comparing each element of rhs with lhs
| float   | [float] | [bool]   | Returns an array containg result of comparing each element of rhs with lhs

## && - Logical AND 
| lhs     | rhs     | result     | description |
| ------- | ------- | ---------- | ----------- |
| bool    | bool    | bool       | true if both lhs and rhs are true, false - otherwise

## || - Logical OR
| lhs     | rhs     | result     | description |
| ------- | ------- | ---------- | ----------- |
| bool    | bool    | bool       | false if both lhs and rhs are false, true - otherwise

## = - Assign
| lhs        | rhs | result | description |
| ---------- | --- | ------ | ----------- |
| mut (T\|S) | T   | T      | assign value on right to mut on left returns value on right