# Statements
## import
```
import path
```
path is string literal
Imports code from given path. Importing happens during parsing. Import produces a struct holding
all variables declared in the file

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

## loop
```
loop statment
```

## while loop
```
while condition statment
```

## while set
```
while ident:type = expression statment
```

## for
```
for element_ident in expression statment
```
The expression must be an iterator

## break
```
break
```
Exit loop

## continue
```
continue
```
Skip to the next iteration of a loop