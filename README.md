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
|            | <            | Lower                   |               |
|            | <=           | Lower or equal          |               |
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
| [int\|float] | [int]  | Returns an array containing results of calling - operator on each element of the given array |

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
