# Statements

Statements can be used on their own or inside a block. A block is a sequence of
statements enclosed in braces; semicolons between statements are optional.

## Import

```simplesl
import "path/to/script.simplesl"
```

The path must be a string literal. Imports are resolved while parsing, and an
import evaluates to a struct containing the variables declared in the imported
file.

## Return

```simplesl
return
return expression
```

Return from the current function. A bare `return` returns `()`.

## If

```simplesl
if condition {
    statements
} else {
    statements
}
```

The condition must be a boolean. The `if` body must be a block, and an `else`
body, when present, must also be a block. `else if` chains are supported.

Conditions can be chained with semicolons. All conditions must succeed for the
body to run:

```simplesl
if is_ready(); count > 0 {
    process(count)
}
```

A condition can bind a value using a pattern and `=`. The body runs only when
the value matches the pattern, and the bound names are available in the
remaining conditions and body:

```simplesl
if value: int = get_value(); value > 0 {
    use(value)
}
```

## Match

```simplesl
match expression {
    value => body,
    value1, value2 => body,
    name: type => body,
    => default_body,
}
```

Match arms are tried in order. Value arms compare against the expression;
comma-separated values share a body. A type pattern may bind the matched value
to a name. The final empty pattern (`=>`) matches any value. The arms must cover
every possible type of the expression; use a final empty pattern when needed.
Each body can be a block, a `return`, or an expression.

## Variable declaration

```simplesl
name := expression
name: type := expression
(first, second) := expression
struct{name, age: years} := expression
[first, second] := expression else {
    return default_value
}
```

Declares a variable using the expression's value. A type annotation is
optional. Tuple patterns can bind values from a tuple. Struct patterns bind
fields by name; a field can use its own name as the binding or provide an
alias:

```simplesl
struct{name, age: years} := person
```

Array patterns bind elements by position and require an `else` branch because
an array's length is not part of its type. A trailing `..name` captures the
remaining elements into a new array; without it, the pattern must match the
array's exact length:

```simplesl
[first, second] = values else {
    return default_value
}

[first, ..rest] = values else {
    return default_value
}
```

## Pattern binding with `else`

```simplesl
(first, second): (int, int) = expression else {
    return default_value
}
```

Matches the value against a pattern and binds its names when the pattern
matches. If it does not match, the `else` block runs. That block must diverge
from the current control flow, for example with `return`, `break`, or
`continue`; execution cannot continue past the statement after a mismatch.
Bindings are available after the statement only on the successful path.

## Loop

```simplesl
loop {
    statements
}
```

The loop body must be a block. It repeats until it executes `break`.

## While

```simplesl
while condition {
    statements
}
```

The body must be a block. It repeats while the boolean condition is true. Like
`if`, `while` can use semicolon-separated condition chains and pattern-binding
conditions:

```simplesl
while item: int = next_value(); item > 0 {
    process(item)
}
```

## For

```simplesl
for element in iterator {
    statements
}
```

The body must be a block. The expression must be an iterator. The loop calls it
until it reports that it is exhausted, binding each yielded value to `element`.

## Break and continue

```simplesl
break
continue
```

`break` exits the nearest loop. `continue` skips the rest of the current
iteration and advances to the next one. Both are only valid inside a loop.
