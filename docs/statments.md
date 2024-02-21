# Statments
## import
```
import path
```
path is string literal
Imports code from given path. Importing happens during parsing. Import statment is replaced with code written from file.

## return
```
return statment?
```

Exit function with result of executing given statment or () if no statment.

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