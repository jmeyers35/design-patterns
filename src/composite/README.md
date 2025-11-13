# Composite Pattern

## Intent
Compose objects into a recursive/hierarchical tree. Individual objects _and_ compositions share the same interface.

## Example Use Case
Graphics, word procesor.

## Consequences
- Clients are agnostic as to whether they're operating on a Parent or a Leaf.
- Adding new kinds of Components (Nodes in the code example) is easy.
- Can make designs overly general.

## Implementation Considerations
- You may want explicit parent references for more complicated traversals.
- The Component (`Node`) interface should be maximal, so clients can remain agnostic to parents vs leaves. However, this leads to operations being defined that aren't meaningful for leaves - for example, `get_children`. You can lean on default implementations for such cases.
- In this example, we've chosen transparency of the interface over safety - clients could call `add` or other child-mutating operations on a leaf, which may not be safe in all contexts. You have to decide between safety and interface uniformity.
- You can memoize expensive traversals, but you'll need a way to invalidate cached values when children change. A parent reference is useful for this.
- Consider the data structure for storing childen. The best one will depend on your problem/access patterns.

## Related Patterns
- Parent -> Component links can be used for Chain of Reponsibility.
- Decorator is often used with Composite.
- Flyweight lets you share Components, but you can no longer have parent references.
- Iterators can traverse composites.
- Visitors can localize operations that would otherwise be distributed across Composite and Leaf classes.
