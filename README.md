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



