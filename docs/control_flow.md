# Control Flow
## if expression
```
if condition instruction [else instruction]
```
## if set expresion
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