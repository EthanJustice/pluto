# Pluto Specification

## Comments

```pluto
// Comments start with two forward slashes
// All comments end with a line break
```

## Valid Names

Names for keys and groups must contain only a..z characters (case doesn't matter), as well as underscores (`_`).

Examples of valid names:

```text
this_is_a_valid_name
valid
Valid
VALID_NAME
```

Examples of invalid names:

```text
\This is not a valid name
${This is not a valid name}
THIS-ISN'T-VALID
```

## Groups

Groups are sets of related items, similar to objects in JSON.

Groups must have a [valid name](#valid-names) and must end with a line break, so that the only thing on the line is the name of the group.  Members of the group must follow immediately below, and be separated by line breaks.  The end of the group is denoted by a line without anything in it.

Example group:

```pluto
group_name
```

Example of multiple groups:

```pluto
person_one
name "Name"
age 12

// This begins an unrelated section of uncategorised items
year = 2021

// this begins another group
person_two
name "Second Name"
age 22
```

## Pairs

Pairs are key/value pairs. They occupy one line.

```pluto
key value
```

A key with a `string` value:

```pluto
name "Generic Name"
```

A key with a `number` value:

```pluto
age 22
```

A key with a `boolean` value:

```pluto
alive false
```

A key with an array value:

```pluto
favourite_numbers 2, 11, 12
```

## Types

There are three primary types in Pluto, as well as an array.

+ strings (`"This is a string!"`)
+ numbers (`10`)
+ booleans (`true` and `false`; note capitalisation and lack of quotation marks)

Arrays are comma-separated lists which contain a type from above. Each item must be separated by a comma and space, with the exception of the last and first items.

```pluto
primes 2, 3, 5, 7, 11
```
