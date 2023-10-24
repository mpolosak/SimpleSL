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