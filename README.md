# Constraint Logic Programming
> An attempt to learn Rust by building a costraint logic solver.
<div style="color:#aa2323">

## Code Review Wanted!
I'm trying to learn Rust and would really appreciate feedback on this project; any scope, any area. Reference what your comment is about and create a git issue or toss me an email!
</div>

## Background
The basic idea is to give the software a program on the lines of:

```
a ∈ [1,2,3]
a * 5 > 9
min a
```
Which should respond with:
```
a = 2
```

But at the moment that is quite a bit away.


There are only two hard problems in computer science: naming things, cache consistency and off by one errors.