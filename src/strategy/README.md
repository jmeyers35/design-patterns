# Strategy Patterh

## Intent
Define a family of algorithms and encapsulate them, making them interchangeable.

## Applicability
Use when:

- You have related classes that differ only in their behavior.
- You need different variants of an algorithm.
- An algorithm uses data clients shouldn't know about.
- A class defines many behaviors and uses each conditionally. Move the conditions into a Strategy class.

## Consequences
- You'll have families/hierarchies of related algorithms.
- An alternative to subclassing the Context class directly.
- Strategies eliminate conditional statements.
- Clients must be aware of different Strategies.
- Communication overhead between the Context and Strategy objects.
- Increased number of objects.

## Implementation
- The Strategy and Context interfaces must allow for Strategy implementations to efficiently access the Context's data necessary.
    - You can plumb data down from the Context to the Strategy, but this may lead to inefficiences where the Context may be passing unnecessary data.
    - You can pass a reference to the Context itself to the Strategy so the Strategy and request the data it needs on-demand, but now you'll need a rich Context interface and there will be tighter coupling between the two.
- If the strategy can be determined at compile-time, consider using generics to bind the Strategy to its Context statically:

```rust
pub trait Strategy {
    fn foo() -> i32,
}


pub struct Context<T> where T: Strategy {
    strat: T
}

impl<T> Context<T> where T: Strategy {
    pub fn do_something(&self) {
        let result = self.strat.foo();
    }
}
```

- If there is an acceptable default behavior, Context objects can make the Strategy optional. This may simplify implementation.
