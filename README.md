# pluto

simple configuration language

**Pluto is not in a working state at the moment, as it has no implementations in any language. This is being worked on.**

Think of Pluto as TOML combined with YAML, except it won't give you carpal tunnel.

See the [spec](./SPEC.md) for more.

## Tools

### plto

[plto](./plto) is a CLI validator for Pluto files.

## Libraries

## Examples

Here's a simple example of a Pluto file:

```pluto
person_one
name "Name"
age 12

// This begins an unrelated section of uncategorised items
year = 2021
```

See the [Examples](./examples) folder for more.
