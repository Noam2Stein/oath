`Oath` is an early-development, general-purpose programming language focused on semantic safety through compile-time value constraints.
It helps you catch logic bugs and memory bugs at compile-time, generates optimal code, and provides great productivity through powerful abstractions.

# Constraints
In most languages, the only information you can express about a value is its type.
In `Oath`, you can go further by adding compile-time constraints to function inputs, outputs, and variables.

```oath
fn main() {
  not_five(4);
  not_five(5); // ERROR: fn `not_five` requires the input to not be `5`
  not_five(6);
}

fn not_five(num i32: != 5) {
  println("{num} isn't 5!");
}
```

In most languages, there's no way to say "`num` must not be 5."
You’d have to document it and trust the caller, and even perform a runtime check.
Oath allows you to track invariants at compile-time.

Constraints can be promised in outputs:

```oath
fn main() {
  // The compiler verifies that num is never 5, so it can safely be passed to `not_five`.
  let num = promise_not_five();
  not_five(num);
}

fn promise_not_five() -> i32: != 5 {
  7
}
```

```oath
fn bad_fn() i32: > 0 {
  -1 // ERROR: promised `output > 0`
}
```

Constraints can be part of types:

```oath
struct Range {
  start i32,
  end i32: >= start,
}
```

With constraints, you can catch logical bugs at compile-time.

# Generics

Generics in Oath are defined as items parameterized over values,
where each unique combination of generic-parameters counts as a distinct type.

Oath treats types as values, of type `type`, this allows for more powerful generics.

```oath
// `T` is a value of type `type`.
// Each unique `T` produces a unique `Three<T>` type.
struct Three<T> {
  one T,
  two T,
  three T,
}
```

```oath
// `N` is a value of type `u32`.
// Each unique `N` produces a unique `Polygon<N>` type.
struct Polygon<N u32> {
  points [Point; N],
}
```

# Performance

`Oath` enables you to track invariants at compile time, eliminating the need for many unnecessary runtime checks — making code faster.

Take `null` as an example:
In many languages, because all references can be `null`, you have to constantly check if values are `null`.
In languages that can track nullability at compile time (meaning values cannot normally be `null`), those checks go away which improves performance.

`Oath` takes this concept further.
It allows you to remove unnecessary checks not just for `null`, but for any condition ever.




