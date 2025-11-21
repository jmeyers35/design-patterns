# Visitor Pattern

## Intent
Represent an operation to be done on the elements of an object structure. Visitor lets you define a new operation without changing the classes of the elements on which it operates.

## Applicability
Use when:
- An object structure contains many classes of objects with differing interfaces, and you want to perform operations on these objects that depend on their concrete class.
- Many distinct and unrelated operations need to be performed on objects in an object structure, and you want to avoid "polluting" their classes with these operations.
- The classes defining the object structure rarely change, but you often want to define new operations over the structure.

## Consequences
- Visitor makes adding new operations easy.
- A visitor gathers related operations and separates unrelated ones.
- Adding new `ConcreteElement` classes is hard.
- Visiting across class hierarchies.
- Accumulating state.
- Breaking encapsulation.
