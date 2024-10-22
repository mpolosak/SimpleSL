# Statements
## import
```
import path
```
path is string literal
Imports code from given path. Importing happens during parsing. Import statement is replaced with code read from file.

## return
```
return statement?
```

Exit function with result of executing given statement or () if no statement.

## if
```
if condition instruction [else instruction]
```
## if set
```
if name:type = expression instruction [else instruction]
```

## match
```
match expression {
    value => instruction      // matches when value of expression is equal to `value` 
    name: type => instruction // matches when type of expression matches `type`
    => instruction // matches all expressions
}
```

## variable declaration
```
ident := statment
```
Declares new variable with given value

## while loop
```
while condition statment
```