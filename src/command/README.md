# Command Pattern

## Intent

Encapsulate a request as an Object, allowing clients to be parameterized with different requests, requests to be queued, or operations to be undone.


## Applicability

- When you want to parameterize objects by an action to perform.
- When you want to specify, queue, and execute requests at different times.
- When you want to support undo.
- When you want to support logging changes so they can be re-applied later.
- When you want to structure a system around high-level operations build on primitive operations.

## Consequences

- Decouples the object that invokes the operation from the one that knows how to perform it.
- Commands are first-class objects.
- You can assemble commands into a composite command.
- It's easy to add new Commands.
