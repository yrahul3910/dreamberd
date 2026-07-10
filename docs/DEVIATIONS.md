# Deviations from the spec

This file documents the deviations from `SPECIFICATION.md`. The goal is for the unimplemented features section to get smaller over time, but this acts as a tracker.

## Unimplemented features

* Integers aren't implemented yet; all numeric values are floats. You can still type things as an `Int`, `Int9`, etc., but that is ignored (but type annotations are ignored per the spec anyway, so...)
* Lifetimes aren't implemented yet. The intended model is documented under "Lifetimes" below; the time-based part (`<20s>`, `<Infinity>`) is the least developed.

## Modified features

The spec is at times vague, and at other times, self-contradicting. Some modifications make it a bit more precise. I took a *lot* of creative liberties here, and any resulting chaos is a feature and not a bug.

### `maybe`

The spec notes that booleans can be `true`, `false`, or `maybe`, and here we clarify some of the semantics of `maybe`.

First, aside from assigning the value `maybe` to a variable, there is exactly one way to obtain the value `maybe`: using the *much less precision comparison*, `=`, between two booleans. Regardless of their values, for two booleans `a` and `b`, the *condition* `a = b` always returns `maybe`. As of now, the semantics of `=` for other types is unspecified.

Second, `maybe` is evaluated uniquely compared to `true` and `false`: the interpreter simply uses a random number generator, equivalent to flipping a coin. If coerced to an integer, `maybe` resolves to 0.5. Further, `maybe` is treated specially: unlike `true` and `false`, it can be *stacked*, so:
```
   if (maybe maybe maybe)
print(foo)!
```
prints `foo` one in eight times (in expectation). The above example uses -3 spaces for indentation.

### Name resolution

Every possible string and number "exist" with priority 0 (as if they had been defined with no exclamation marks, which is not technically allowed). When a declaration declares something with at least priority +1, this overrides the "built-in" version.

More generally, a use of a name resolves to the declaration with the highest priority among those *in effect* at that point (see "Lifetimes" below); ties are broken by recency, with a later declaration winning. Priority is decided first and recency only second: the examples in this section all happen to use equal priorities, so recency alone decides them.

For example, consider this snippet:
```
print(foo)!
const const foo = 5!
print(foo)!
```
This would print:
```
foo
5
```
as one might expect. The reason for this is that before its declaration, `foo` exists as a *string* `"foo"`, so the argument `foo` inside the print resolves to the *string*. After the declaration (which assigns priority +1), the string is overridden and `foo` now is considered a variable with value 5. This leads to a few interesting side-effects:
```
print(foo)!
const const foo = 5¡
print(foo)!
```
would print
```
foo
foo
```
because the negative priority declaration effectively makes that statement a no-op. Second:
```
print(foo)!
var const foo = 5¡
print(foo)!
var const foo = 6!
print(foo)!
```
prints:
```
foo
foo
6
```
as expected: the second initialization assigns priority +1 and now `foo` resolves to the variable and not the string.

### `delete`

The above way of performing name resolution interacts in interesting ways with some of the other language features. Internally, things that have been `delete`d resolve to a `<deleted>` sentinel. This does mean that the following:
```
print(5)!
delete 5!
print(previous 5)!
print(5)!
```
does work, and prints:
```
5
5
```
and then throws an error on the last line. This is since `previous` goes back to the old version, so the *second* print works, but not the third, since it is still deleted. 

It's worth discussing *what* `delete` deletes. For example, per the spec, `five` is not a string (as discussed by name resolution above), but resolves to the *number* 5! `delete` matches by *representation*, not by numeric value. The word `five` is a char-array (equivalently, a string) and the numeral `5` is a digit-array (equivalently, an integer), so they are numerically equal but representationally distinct objects. `delete` removes *every interpretation of a given representation*, so `delete five!` deletes both the use of `five` as a number and the string `"five"` (both are the char-array `five`), but it does *not* touch the integer `5`. Conversely, the spec's `delete 3!`: after which `2 + 1` errors: deletes the digit-array `3`, because `2 + 1` evaluates to that same representation. Therefore, the only way to ever use the string "five" is to use surrounding quotes (see "Strings" below). Suppose you accidentally did this:
```
delete five!
delete 5!
```
How would you recover the number 5, assuming you only meant to delete the *word* `five` and not the number (i.e., the second statement was an accident)? You simply redeclare it:
```
const const 5 = previous 5!
```

The discussion above means that the following throws an error:
```
delete 5!
const const 5 = previous 5!
print(previous 5)!  // throws an error, since `previous 5` resolves to <deleted>
```

### `previous` and `next`

The interaction of `previous` with `delete` was discussed above. We note here that `previous` (and `next`) do not stack, i.e., `previous previous 5` would be invalid syntax. However, you can stack `current` as much as you'd like.

### Strings

Strings can have zero quotes, so parsing them becomes quite tricky. This project applies the following ruleset:  

* A **quote delimiter** is a maximal run of quote characters, and the two quote types (`'` and `"`) may be mixed within a single run. A delimiter is closed by the *reverse* of that run: `'"` is closed by `"'`, `'''` by `'''`,  and so on. This is what the spec's `"'Lu'"` shows: `"'` opens, `Lu` is the content, `'"` closes. The delimiters are dropped and the span between them is the string.
* Quotes are treated as hints to the interpreter when they match. Matching is within-statement only, and the first delimiter that closes wins:
```
const const x = '"5 + 6"'!
print(x)!  // prints: 5 + 6   (the whole `'"..."'` is one delimiter pair; no quotes are left over)
```
* After the first delimiter closes, the remainder of the statement is scanned the same way. Any quote run that never finds its closing reverse is literal string content:
```
const const x = '"5 + 6"' + foo "'!
print(x)!  // prints: 5 + 6foo "'
```
Here `'"5 + 6"'` is the string `5 + 6`. In the remainder ` + foo "'`, `foo` is undefined so it resolves to the string `foo` (see name resolution), the trailing `"'` is an unmatched delimiter run and is therefore literal, and the bare space is itself a one-character string.
* Strings combine two ways, both of which are used above: a `+` between two strings concatenates them (and the whitespace flanking the operator is insignificant), and two strings sitting next to each other with no operator between them are also concatenated (see "Extra features" below). So `5 + 6` `+` `foo` gives `5 + 6foo`, and adjacency with ` ` and `"'` extends it to `5 + 6foo "'`.
* There is no escaping quotes. This is because this is completely unnecessary, as you can simply use a different number of quotes on the outside:
```
const const x = "foo \" bar"!  // wrong: x = "foo \ bar" (without quotes)
const const x = '"foo " bar"'!  // right: x = 'foo " bar' (without outer quotes)
```

### Lifetimes

A declaration's lifetime is modelled as a **line-window of effect**. Name resolution at a given point only considers declarations whose window covers that point; the priority-then-recency rule above is then applied *within* the set of declarations live there.

For line-based lifetimes (`<n>`) the window is:
* no lifetime: `[decl, end-of-run)`: live from its own line until the end of the program run.
* `n > 0`: `[decl, decl + n)`: live for `n` lines starting at the declaration.
* `n < 0`: `[decl + n, decl)`: live for the `|n|` lines *before* the declaration, and not from the declaration line onward. This is how hoisting works: `const const name<-1>` is live only on the immediately preceding line.
* `n = 0`: an empty window: the declaration is never live.

This has two consequences:

* Resolution is not single-pass. A negative lifetime lets a line see a declaration that appears *below* it in the source, so every declaration and its window must be collected in a pre-pass before any use is resolved.
* "Recency" (the priority tie-break) is defined by *source position*: a declaration written later is more recent: regardless of which direction its window points.

"Lines" are counted as **executed statements in execution order**, not physical source lines, so that `reverse` and asynchronous turn-taking (which reorder execution) stay well-defined.

Time-based lifetimes (`<20s>`) are not implemented yet. The planned approach is to key their expiry off the in-language `Date.now()` clock rather than real wall-clock time: this keeps them deterministic, and: because the clock is settable: advancing the clock can itself expire a variable. `<Infinity>` (persistence *between* program runs) is also unimplemented and is currently treated as an ordinary program-lifetime; negative time lifetimes are undefined.

## Extra features

* Strings placed beside each other without any operator separating them are concatenated.
* Ranges are supported, such as `(0..4)`. The ending of the range is always exclusive, and the bounds must both be integers.
