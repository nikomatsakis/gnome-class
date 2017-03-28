I've trying to mock up what the code should look like in the `mock`
module and taking some notes here on challenges I've encountered.

**Private fields.** I've decided to just ignore them and to put all
fields into the main structs. If you want to ensure ABI compatibility
-- and it's recommended that you do -- you can always just make `let
private: Box<...>` as one of your fields.

**Mutability.** How do we integrate mutability and so forth? It feels
like the GNOME system -- being based on invisible, omnipresent
ref-counting -- would prefer to make everything aliased by
default. This, it turns out, dovetails with another important
development.

**Lack of upcasting.** I think the type of `this` (ought to be `self`,
but oh well) in an accessor method really ought to be the object type.
But I run into a problem because I want to coerce `Ptr<T: ?Sized +
Counter>` into `Ptr<Counter>`, and I cannot, because we don't know
that `T: ?Sized`.

Attempting a workaround where I have (maybe) two distinct impls of
`CounterSuper`, one for `Ptr<T>` and one for `Ptr<Counter>`.

I also tried adding methods that reurn `Ptr<Counter>` into the coutner
trait, but lacking the ability to do `fn(&Ptr<self>)` in a trait
object, got screwed.

Um, oh yeah, a more **fundamental** problem. We really need the impl
to be on `Ptr<T>` so that we know that we can always clone. But we
also want to pass around a `Ptr<Counter>`, because we can't pass
around a naked trait object (we'd need something else).

So it seems like what we want is a newtype:

```rust
struct Counter {
    data: Ptr<CounterTrait>
}
```

This has some appeal, but it's not entirely clear that it will solve
our problems.

Total hack: can we keep a **weak ref**?



