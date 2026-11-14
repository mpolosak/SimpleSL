# Iterators

An iterator is a zero-argument function that returns a tagged value:

```simplesl
() -> (#Some(T) | #None)
```

Each call returns `#Some(value)` for the next element. Once exhausted, it
returns `#None`. Calling an iterator advances it, so iterators are stateful and
are consumed by loops and iterator operations.

## Creating an iterator

The postfix `~` operator converts an array to an iterator:

```simplesl
values := [1, 2, 3]~
```

You can also write an iterator function. This example yields integers from
`start` up to, but not including, `end`:

```simplesl
range := (start: int, end: int) -> () -> (#Some(int) | #None) {
    current := mut start
    return () -> (#Some(int) | #None) {
        if *current < end {
            value := *current
            current += 1
            return #Some(value)
        }
        return #None
    }
}
```

## Iterating

Use a `for` loop to receive each yielded value. The loop stops when the
iterator returns `#None`:

```simplesl
for value in range(1, 4) {
    std.io.print(value)
}
```

You can call an iterator directly to inspect or consume one result:

```simplesl
numbers := [10, 20]~
first := numbers()  // #Some(10)
second := numbers() // #Some(20)
done := numbers()   // #None
```

## Transforming and filtering

The infix `@` operator lazily maps each element through a function. The infix
`?` operator lazily keeps elements for which a predicate returns `true`:

```simplesl
is_even := (value: int) -> bool {
    return value % 2 == 0
}
double := (value: int) -> int {
    return value * 2
}

result := [1, 2, 3, 4]~ ? is_even @ double $]
// result is [4, 8]
```

The postfix type filter `iterator ? type` keeps only values matching `type`:

```simplesl
integers := [1, 2.5, 3]~ ? int $]
// integers is [1, 3]
```

## Consuming an iterator

The postfix `$]` operator collects all remaining elements into an array.
Reductions and partitioning also consume the iterator:

```simplesl
values := [1, 2, 3, 4]~
total := values $+ // 10

is_even := (value: int) -> bool {
    return value % 2 == 0
}
(even, odd) := [1, 2, 3, 4]~ \ is_even
// even is [2, 4]; odd is [1, 3]
```

Available reductions are:

* `$+` — sum integers or floats, or concatenate strings.
* `$*` — multiply integers or floats.
* `$&&` — true if all boolean elements are true.
* `$||` — true if any boolean element is true.
* `$&` and `$|` — bitwise AND and OR of integer elements.
* `$ initial_value reducer` — fold elements using
  `reducer(accumulator, element)`, starting with `initial_value`.

The empty-input identities are `0`, `0.0`, or `""` for `$+`; `1` or `1.0`
for `$*`; `true` for `$&&`; `false` for `$||`; `-1` for `$&`; and `0` for
`$|`.
