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
| 3          | @            | Map                         | Left-to-right |
|            | ?            | Array filtering             |               |
|            | \\           | Array partition             |               |
|            | $ expression | Array reducing              |               |
|            | $+           | Array sum                   |               |
|            | $*           | Array product               |               |
|            | $&&          | Logical and reduce(all)     |               |
|            | $\|\|        | Logical or reduce (any)     |               |
|            | $&           | Bitwise and reduce          |               |
|            | $\|          | Bitwise or reduce           |               |
|            | ~            | Iterate                     |               |
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

## ? type - Filter by type
```
iterator ? type
```
Creates iterator returning only elements matching given type

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

## - - Unary minus
| operand      | result       |
| ------------ | ------------ |
| int \| float | int\|float   |
Additive inverse
`-MIN_INT == MIN_INT`

## * - Indirection
| operand | result | description |
| ------- | ------ | ------------|
| mut T   | T      | Returns value contained in mut

## @ - Map
| lhs             | rhs             | result          |
| --------------- | ----------------| --------------- |
| () -> (bool, T) | (value: T) -> S | () -> (bool, S) |

## ? - Filter
| lhs             | rhs                            | result          |
| --------------- | ------------------------------ | --------------- |
| () -> (bool, T) | (value: T) -> bool             | () -> (bool, T) |

Creates iterator returning only this elements of iterator on left for which function returned true

## \ - Partition
| lhs             | rhs                            | result     |
| --------------- | ------------------------------ | ---------- |
| () -> (bool, T) | (value: T) -> bool             | ([T], [T]) |

Returns tuple containing two arrays. First array contains all elements for which function returned
true, second array contains all other elements.

## $ - Reduce
```
iterator $ initial_value function
```

| lhs             | initial value | rhs                            | result  |
| --------------- | --------------| ------------------------------ | ------- |
| () -> (bool, T) | S             | (acc: T \| U, current: S) -> U | T \| U  |

## $+ - Sum
```
iterator $+
```
| lhs                  | result  |
| -------------------- | ------- |
| () -> (bool, int)    | int     |
| () -> (bool, float)  | float   |
| () -> (bool, string) | string  |

Calculates sum of all elements of iterator.

## $* - Product
```
iterator $*
```
| lhs                 | result  |
| ------------------- | ------- |
| () -> (bool, int)   | int     |
| () -> (bool, float) | float   |

Calculates product of all elements of iterator.

## $&& - Logical and reduce (all)
```
iterator $&&
```
| lhs                | result |
| ------------------ | ------ |
| () -> (bool, bool) | bool   |

Test if every element of iterator function is true

## $|| - Logical and reduce (any)
```
iterator $||
```
| lhs                | result |
| ------------------ | ------ |
| () -> (bool, bool) | bool   |

Test if any element of iterator function is true

## $& - Bitwise and reduce
```
iterator $&
```
| lhs               | result  |
| ----------------- | ------- |
| () -> (bool, int) | int     |

Equivalent of:
```
iterator $!0 (acc: int, curr: int) -> int {return acc & curr}
```

## $| - Bitwise or reduce
```
iterator $|
```
| lhs               | result  |
| ----------------- | ------- |
| () -> (bool, int) | int     |

Equivalent of:
```
iterator $0 (acc: int, curr: int) -> int {return acc | curr}
```

## ~ - Iterate
```
iterator $|
```
| lhs | result          |
| --- | --------------- |
| [T] | () -> (bool, T) |
Create iterator over elements of array

## ** - Exponentiation
| lhs     | rhs     | result     | comment     |
| ------- | ------- | ---------- | ------------|
| int     | int     | int        | rhs cannot be negative
| float   | float   | float      |
lhs to the power of rhs
## * - Multiplication
| lhs     | rhs     | result     |
| ------- | ------- | ---------- |
| int     | int     | int        |
| float   | float   | float      |

## / - Division
| lhs     | rhs     | result     |
| ------- | ------- | ---------- |
| int     | int     | int        |
| float   | float   | float      |

## % - Remainder
| lhs     | rhs     | result     |
| ------- | ------- | ---------- |
| int     | int     | int        |
lhs % rhs

## + - Addition/Concatenation
| lhs      | rhs      | result     | description |
| -------- | -------- | ---------- | ------------|
| int      | int      | int        | lhs + rhs   |
| float    | float    | float      | lhs + rhs   |
| [T]      | [S]      | [T\|S]     | Array concatenation
| string   | string   | string     | String concatenation

## - - Subtration
| lhs     | rhs     | result     |
| ------- | ------- | ---------- |
| int     | int     | int        |
| float   | float   | float      |

## << - Bitwise left shift
| lhs     | rhs     | result     |
| ------- | ------- | ---------- |
| int     | int     | int        |
Shift lhs rhs bits to left

## >> - Bitwise right shift
| lhs     | rhs     | result     |
| ------- | ------- | ---------- |
| int     | int     | int        |
Shift lhs rhs bits to right

## & - Bitwise AND
| lhs     | rhs     | result     |
| ------- | ------- | ---------- |
| int     | int     | int        |
| bool    | bool    | bool       | 

## ^ - XOR
| lhs     | rhs     | result     |
| ------- | ------- | ---------- |
| int     | int     | int        |
| bool    | bool    | bool       |

## | - Bitwise OR
| lhs     | rhs     | result     |
| ------- | ------- | ---------- | description |
| int     | int     | int        |
| bool    | bool    | bool       |


## - == Equal
Returns true if the element on the right is equal to the element on the left, false - otherwise

## - != Not equal
Returns true if the element on the right is not equal to the element on the left, false - otherwise

## < - Less
| lhs     | rhs     | result   |
| ------- | ------- | -------- |
| int     | int     | bool     | 
| float   | float   | bool     |
Returns true if the element on the left is less than the element on the right, false - otherwise

## <= - Less or equal
| lhs     | rhs     | result   |
| ------- | ------- | -------- |
| int     | int     | bool     | 
| float   | float   | bool     |
Returns true if the element on the left is less than or equal to the element on the right, false - otherwise

## > - Greater
| lhs     | rhs     | result   |
| ------- | ------- | -------- |
| int     | int     | bool     |
| float   | float   | bool     |
Returns true if the element on the left is greater than the element on the right, false - otherwise

## >= - Greater or equal
| lhs     | rhs     | result   |
| ------- | ------- | -------- |
| int     | int     | bool     |
| float   | float   | bool     |
Returns true if the element on the left is greater than or equal to the element on the right, false - otherwise

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