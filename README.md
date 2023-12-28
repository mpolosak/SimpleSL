# SimpleSL
SimpleSL is a scripting language i create in my free time.

## Running binary
Run script from a file:
```
cargo run examples/example1
```
Run repl:
```
cargo run
```

## Running scripts from Rust
```Rust
use simplesl::{Code, Interpreter};

fn main() {
    let interpreter = Interpreter::with_stdlib();
    let _ = Code::parse(&interpreter, "print(\"Hello world!\")")
        .unwrap()
        .exec();
}
```

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
