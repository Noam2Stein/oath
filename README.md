`Oath` is a new general-purpose programming language focused on semantic safety through compile-time value constraints.
It helps you catch more bugs (not just memory errors) before your code runs, while improving performance, flexibility, and productivity.

# Features

### Constraints
In most languages, the only information you can express about a value is its type.
In `Oath`, you can go further by adding compile-time constraints to function inputs, outputs, and variables.

```oath
fn main() {
  not_five(4);
  not_five(5); // ERROR: `not_five` requires the input to not be 5
  not_five(6);
}

fn not_five(num i32: != 5) {
  println("{num} isn't 5!");
}
```

In most languages, there's no way to say "`num` must not be 5."
You’d have to document it and trust the caller, and if they get it wrong, you’ll hit a runtime bug.
Oath catches it at compile time.

Constraints can be promised in outputs:

```oath
fn main() {
  // The compiler verifies that num is never 5, so it can safely be passed to not_five.
  let num = promise_not_five();
  not_five(num);
}

fn promise_not_five() i32: != 5 {
  eval 7;
}
```

```oath
fn bad_fn() i32: > 0 {
  eval -1; // ERROR: promised `output > 0`
}
```

Constraints can be part of types:

```oath
struct Range {
  start i32,
  end i32: >= start,
}
```

With constraints, Oath catches not just memory bugs, but logic errors, panics, and invalid assumptions, before you run your program.

### Generics

Generics in Oath are defined by parameterizing items over values — including types, since types themselves are values of type `type`.

Each unique value of a parameter produces a distinct, compile-time-specialized definition.

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

This makes generics in Oath fully value-based and statically resolved, enabling flexible, zero-cost abstractions.

# Performance

`Oath` enables you to resolve constraints at compile time, eliminating the need for many runtime checks — making your code faster and more predictable.

Take `null` as an example:
In many languages, because the compiler can’t guarantee that a reference isn’t `null`, you end up writing lots of runtime checks.
In languages that can track nullability at compile time, those checks go away — and performance improves.

`Oath` takes this concept further.
It removes unnecessary checks not just for `null`, but for any condition you can express.
Bounds checks, state invariants, function preconditions — if you can constrain it, the compiler can optimize it away.








