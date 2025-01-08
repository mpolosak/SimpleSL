# Iterators
An iterator is a function taking no parameters and returning a tuple of two values, first being of type bool i.e. matching type () -> (bool, any). The iterator returns (true, value) when sequence has not ended yet and (false, value) when all values where already consumed.
## How to create an iterator?
An iterator can be written by user or created from an array using ~ operator.
```
iota := (start: int, end: int) -> () -> (bool, int) {
    i := mut start;
    return () -> (bool, int) {
        val := *i;
        if(val<end){
            i+=1;
            return (true, val);
        }
        return (false, val);
    }
} //function creating iterator returning values from start to end
[1, 2.5, "3"]~ //creates operator from array
```
## Usage of iterator
You can call an iterator as any other function. You can use built in operators to create a new iterator from it (@, ?, ? type), reduce it (\$, \$*, \$&, \$|, \$&&, \$||), collect into array (\$]), partion (\\). You can iterate over it using for loop.
```
is_even = (a: int) {return a%2==0};
x := [23, 2, 12, 45, 0,  65, -2]~?is_even; //creates iterator returning only values of an array that are even
for e in x {
    print(e);
} //prints elements of x iterator
```