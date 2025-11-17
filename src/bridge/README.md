# Bridge Pattern

## Intent

Decouple an abstraction from its implementation so the two can vary independently.

## Applicability

- When you want to avoid permanent binding between an abstraction and its implementation.
- When both the abstraction and implementation should be extensible by subclassing.
- When changes in the implementation of the abstraction should have no impact on the client.
- When you have a class hierarchy that looks like "nested generalizations" - this indicates you may want to split the hierarchy into two.
- When you want to share an implementation among multiple objects.

## Consequences

- Interface and implementation are decoupled.
- Improved extensibility.
- Hiding implementation details from clients.
