# Decorator Pattern

## Intent

Attach additional responsibilities to an object dynamically. Flexible alternative to subclassing.

## Applicability

- Add responsibilities to individual objects dynamically and transparently
- For responsibilities that can be withdrawn
- When extension by subclassing is impractical - eg when a large number of independent extensions are possible and would cause an explosion of subclasses

## Consequences

- More flexibility than static inheritence
- Avoids feature-laden classes high in the hierarchy
- A decorator and its component are _not_ identical
- You have lots of little objects
