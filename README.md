# SimpleSL
SimpleSL is a scripting language i create in my free time.

## Syntax
### Hello world example
```
print("Hello world")
```
### Comments
Comment works like in C or Rust
```
// One line comment
/* 
Multiline comment
*/
print("Hello world"/* Comment */)
```
### Variables
```
x = 5 // int
y = 5.0 // float
text = "Hello\n world" // string
x = ["int", 7.0, 4] // array
x = [0; 5] // array containg five zeros
tuple = (5, 7.8, "value") // tuple
{
    tuple = (4, "rgg", 56)
    print(tuple) // prints (4, "rgg", 56)
}
print(tuple) //prints (5, 7.8, "value")
```
### Functions
```
delta = (a: float, b: float, c: float) -> float {
    b**2.0+4.0*a*c
} // function taking free arguments of type float and returning value of type float
name = "Tom"
x = (){
    print("Hello "+name);
}
x() // prints "Hello Tom"
name = "Jerry"
x() // still prints "Hello Tom"
print = (vars: any) {
} // function are availible after they are created and can be overwritten
x() // but this still works as before
y = (f: function()->()){
    f()
} // function y takes function as argument and exec it
y(
    ()->(){print("Function")} // anonymous function
)
rec = (n: int){
    if n>0 {
        rec(n-1)
        print(n)
    }
} //recursion

```
Function arguments are always executed left to right
### Operators
#### Precedence
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

#### [] - Array/string indexing
```
array/string [index]
```
Index must be of type int. Indexing is 0-based. Indexing with values less than zero or greater than the length of the array/string results in an error.
Indexing of a string returns a string containing an UTF-8 character on the given position. 

#### ? type - Array filtering by type
```
array ? type
```
Filters array leaving only elements matching given type

#### () - Function call
```
function (arguments)
```
Calls the function with given arguments.

#### ! - Logical NOT
| operand | result | description                              |
| ------- | ------ | ---------------------------------------- |
| int     | int    | Returns 1 when operand is 0, 1 otherwise |
| [int]   | [int]  | Returns an array containing results of calling ! operator on each element of the given array |

#### ~ - Bitwise NOT
| operand | result | description                              |
| ------- | ------ | ---------------------------------------- |
| int     | int    | Negation of all bits                     |
| [int]   | [int]  | Returns an array containing results of calling ~ operator on each element of the given array |

#### - - Unary minus
| operand      | result       | description                              |
| ------------ | ------------ | ---------------------------------------- |
| int \| float | int\|float   | Additive inverse                         |
| [int\|float] | [int\|float] | Returns an array containing results of calling - operator on each element of the given array |

#### @ - Array maping operator
| lhs         | rhs                                            | result |
| ----------- | ---------------------------------------------- | ------ |
| [T]         | (value: T) -> S                                | [S]    |
| [T]         | (index: int, value: T) -> S                    | [S]    |
| (T, S, ...) | (value_t: T, value_s: S, ...) -> U             | [U]    |
| (T, S, ...) | (index: int, value_t: T, value_s: S, ...) -> U | [U]    |

#### ? - Array filtering operator
| lhs | rhs                           | result |
| --- | ----------------------------- | -------|
| [T] | (value: T) -> int             | [T]    |
| [T] | (index: int, value: T) -> int | [T]    |

Filters array using given function. Leaving only elements for which the function returned value other than zero.

#### $ - Array reducing operator
```
array $ initial_value function
```

| lhs | initial value | rhs                            | result  |
| --- | --------------| ------------------------------ | ------- |
| [T] | S             | (acc: T \| U, current: S) -> U | T \| U  |

#### ** - Exponentiation
| lhs     | rhs     | result     | description |
| ------- | ------- | ---------- | ------------|
| int     | int     | int        | lhs to the power of rhs. rhs cannot be negative
| float   | float   | float      | lhs to the power of rhs
| [int]   | int     | [int]      | Returns an array containg all elements of lhs raised to the power of rhs
| [float] | float   | [float]    | Returns an array containg all elements of lhs raised to the power of rhs
| int     | [int]   | [int]      | 
| float   | [float] | [float]    | 

#### * - Multiplication
| lhs     | rhs     | result     | description |
| ------- | ------- | ---------- | ------------|
| int     | int     | int        | lhs * rhs   |
| float   | float   | float      | lhs * rhs   |
| [int]   | int     | [int]      | Returns an array containg all elements of lhs multiplied by rhs
| [float] | float   | [float]    | Returns an array containg all elements of lhs multiplied by rhs
| int     | [int]   | [int]      | Returns an array containg all elements of rhs multiplied by lhs
| float   | [float] | [float]    | Returns an array containg all elements of rhs multiplied by lhs

#### / - Division
| lhs     | rhs     | result     | description |
| ------- | ------- | ---------- | ------------|
| int     | int     | int        | lhs / rhs   |
| float   | float   | float      | lhs / rhs   |
| [int]   | int     | [int]      | Returns an array containg all elements of lhs divided by rhs
| [float] | float   | [float]    | Returns an array containg all elements of lhs divided by rhs
| int     | [int]   | [int]      | Returns an array containg results of dividing lhs by each element of rhs
| float   | [float] | [float]    | Returns an array containg results of dividing lhs by each element of rhs

#### % - Remainder
| lhs     | rhs     | result     | description |
| ------- | ------- | ---------- | ------------|
| int     | int     | int        | lhs % rhs   |
| [int]   | int     | [int]      | Returns an array containg remainders of dividing each element of lhs by rhs
| int     | [int]   | [int]      | Returns an array containg remainders of dividing lhs by each element of rhs

#### + - Addition/Concatenation
| lhs      | rhs      | result     | description |
| -------- | -------- | ---------- | ------------|
| int      | int      | int        | lhs + rhs   |
| float    | float    | float      | lhs + rhs   |
| [int]    | int      | [int]      | Returns an array containg all elements of lhs incremented by rhs
| [float]  | float    | [float]    | Returns an array containg all elements of lhs incremented by rhs
| int      | [int]    | [int]      | Returns an array containg all elements of rhs incremented by lhs
| float    | [float]  | [float]    | Returns an array containg all elements of rhs incremented by lhs
| [T]      | [S]      | [T\|S]     | Array concatenation
| string   | string   | string     | String concatenation
| [string] | string   | [string]   | Returns an array containg result of concatinating rhs to the end of each string in array lhs
| string   | [string] | [string]   | Returns an array containg result of concatinating lhs to the beging of each string in array rhs

#### - - Subtration
| lhs     | rhs     | result     | description |
| ------- | ------- | ---------- | ------------|
| int     | int     | int        | lhs - rhs   |
| float   | float   | float      | lhs - rhs   |
| [int]   | int     | [int]      | Returns an array containg all elements of lhs decremented by rhs
| [float] | float   | [float]    | Returns an array containg all elements of lhs decremented by rhs
| int     | [int]   | [int]      | Returns an array containg result of decrementing lhs by each element of rhs
| float   | [float] | [float]    | Returns an array containg result of decrementing lhs by each element of rhs

#### << - Bitwise left shift
| lhs     | rhs     | result     | description |
| ------- | ------- | ---------- | ----------- |
| int     | int     | int        | Shift lhs rhs bits to left
| [int]   | int     | [int]      | Returns an array containg result of shifting each element of lhs rhs bits to left 
| int     | [int]   | [int]      |

#### >> - Bitwise right shift
| lhs     | rhs     | result     | description |
| ------- | ------- | ---------- | ----------- |
| int     | int     | int        | Shift lhs rhs bits to right
| [int]   | int     | [int]      | Returns an array containg result of shifting each element of lhs rhs bits to right 
| int     | [int]   | [int]      |

#### & - Bitwise AND
| lhs     | rhs     | result     | description |
| ------- | ------- | ---------- | ----------- |
| int     | int     | int        | Bitwise AND |
| [int]   | int     | [int]      | Array containg result of & for each element of lhs and rhs
| int     | [int]   | [int]      | Array containg result of & for each element of rhs and lhs

#### ^ - XOR
| lhs     | rhs     | result     | description |
| ------- | ------- | ---------- | ----------- |
| int     | int     | int        | Bitwise XOR |
| [int]   | int     | [int]      | Array containg result of ^ for each element of lhs and rhs
| int     | [int]   | [int]      | Array containg result of ^ for each element of rhs and lhs


#### | - Bitwise OR
| lhs     | rhs     | result     | description |
| ------- | ------- | ---------- | ----------- |
| int     | int     | int        | Bitwise OR  |
| [int]   | int     | [int]      | Array containg result of \| for each element of lhs and rhs
| int     | [int]   | [int]      | Array containg result of \| for each element of rhs and lhs

#### - == Equal
Returns 1 if the element on the right is equal to the element on the left, 0 - otherwise

#### - != Not equal
Returns 1 if the element on the right is not equal to the element on the left, 0 - otherwise

#### < - Less
| lhs     | rhs     | result   | description |
| ------- | ------- | -------- | ------------|
| int     | int     | int      | Returns 1 if the element on the left is less than the element on the right, 0 - otherwise
| float   | float   | int      | Returns 1 if the element on the left is less than the element on the right, 0 - otherwise
| [int]   | int     | [int]    | Returns an array containg result of comparing each element of lhs with rhs
| [float] | float   | [int]    | Returns an array containg result of comparing each element of lhs with rhs
| int     | [int]   | [int]    | Returns an array containg result of comparing each element of lhs with rhs
| float   | [float] | [int]    | Returns an array containg result of comparing each element of lhs with rhs

#### <= - Less or equal
| lhs     | rhs     | result   | description |
| ------- | ------- | -------- | ------------|
| int     | int     | int      | Returns 1 if the element on the left is less than or equal to the element on the right, 0 - otherwise
| float   | float   | int      | Returns 1 if the element on the left is less than or equal to the element on the right, 0 - otherwise
| [int]   | int     | [int]    | Returns an array containg result of comparing each element of lhs with rhs
| [float] | float   | [int]    | Returns an array containg result of comparing each element of lhs with rhs
| int     | [int]   | [int]    | Returns an array containg result of comparing each element of lhs with rhs
| float   | [float] | [int]    | Returns an array containg result of comparing each element of lhs with rhs

#### > - Greater
| lhs     | rhs     | result   | description |
| ------- | ------- | -------- | ------------|
| int     | int     | int      | Returns 1 if the element on the left is greater than the element on the right, 0 - otherwise
| float   | float   | int      | Returns 1 if the element on the left is greater than the element on the right, 0 - otherwise
| [int]   | int     | [int]    | Returns an array containg result of comparing each element of lhs with rhs
| [float] | float   | [int]    | Returns an array containg result of comparing each element of lhs with rhs
| int     | [int]   | [int]    | Returns an array containg result of comparing each element of lhs with rhs
| float   | [float] | [int]    | Returns an array containg result of comparing each element of lhs with rhs

#### >= - Greater or equal
| lhs     | rhs     | result   | description |
| ------- | ------- | -------- | ------------|
| int     | int     | int      | Returns 1 if the element on the left is greater than or equal to the element on the right, 0 - otherwise
| float   | float   | int      | Returns 1 if the element on the left is greater than or equal to the element on the right, 0 - otherwise
| [int]   | int     | [int]    | Returns an array containg result of comparing each element of lhs with rhs
| [float] | float   | [int]    | Returns an array containg result of comparing each element of lhs with rhs
| int     | [int]   | [int]    | Returns an array containg result of comparing each element of rhs with lhs
| float   | [float] | [int]    | Returns an array containg result of comparing each element of rhs with lhs

#### && - Logical AND 
| lhs     | rhs     | result     | description |
| ------- | ------- | ---------- | ----------- |
| int     | int     | int        | Non-zero if both lhs and rhs are non-zero, 0 - otherwise
| [int]   | int     | [int]      | Array containg result of && for each element of lhs and rhs
| int     | [int]   | [int]      | Array containg result of && for each element of rhs and lhs

#### || - Logical OR
| lhs     | rhs     | result     | description |
| ------- | ------- | ---------- | ----------- |
| int     | int     | int        | 0 if both lhs and rhs are zero, non-zero otherwise
| [int]   | int     | [int]      | Array containg result of \|\| for each element of lhs and rhs
| int     | [int]   | [int]      | Array containg result of \|\| for each element of rhs and lhs