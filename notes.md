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

Total hack: can we keep a **weak ref**? Ah, yes, that's the ticket out
of this mess (in the real gobject system: we could (at worst) do this
with a hack). It's just a temporary workaround.

So let's review the procedure now. A full class declaration looks like
this (in "pre-processed" form). Right now we are not supporting more
than one superclass, but I don't see any fundamental obstacle to that,
it'd just make things a bit more annoying in the macro itself.

```
class CLASS extends SCLASS {
    new({CARG: CARG_TY}) -> CRET_TY { CBODY }
    
    fields {
      {FIELD: FIELD_TY}
    }
    
    methods {
      {fn CLASS_METHOD(CLASS_MARG: CLASS_MARG_TY) -> CLASS_MRET_TY { CLASS_MBODY }}
    }
    
    impl SCLASS {
      {fn SC_METHOD(SC_MARG, SC_MARG_TY) -> SC_MRET_TY { SC_MBODY }}
    }
}
```

- Given a class CLASS with superclass SCLASS
  - All classes have a superclass, but sometimes it is GObject, which has no members
- Generate:
  - `struct CLASSFields { SCLASS: SCLASSFields, {FIELD:FIELD_TYTY} }`
  - `impl CLASSFields { fn new({CARG:CARG_TY}) -> Self { CBODY }`
  - `trait CLASS: 'static + SCLASS {...}` with members:
    - `fn CLASS(&self) -> &CLASSFields;`
    - `fn CLASSPtr(&self) -> Ptr<CLASS>;`
    - `{ fn CLASS_METHOD(&self, {CLASS_MARG: CLASS_MARG_TY}) -> CLASS_MRET_TY; }`
  - `impl CLASS`:
    - `fn new({CARG:CARG_TY}) -> Ptr<CLASS>`:
      - `struct Impl { fields: CLASSFields, self_ref: RefCell<Weak<Impl>> }`
      - `impl CLASS for Impl`:
        - `fn CLASS(&self) -> &CLASSFields { &self.fields }`
        - `fn CLASSPtr(&self) -> Ptr<CLASS> { self.self_ref.borrow().upgrade().unwrap() }`
        - `{`
          - `fn CLASS_METHOD(&self, {CLASS_MARG: CLASS_MARG_TY}) -> CLASS_MRET_TY`
            - `CLASSSuper::CLASS_METHOD(&self.CLASSPtr(), {CLASS_MARG})`
        - `}`
      - `let ptr = Ptr::new(Impl { fields: CLASSFields::new(f), self_ref: RefCell::new(Weak::new()) })`
      - link ptr
      - `ptr`
  - `trait CLASSSuper`:
    - `{ fn CLASS_METHOD(this: &Self, {CLASS_MARG: CLASS_MARG_TY}) -> CLASS_MRET_TY; }`
    - `{ fn SC_METHOD(this: &Self, {SC_MARG: SC_MARG_TY}) -> SC_MRET_TY; }`
  - `impl<This: ?Sized + CLASS> CLASSSuper for Ptr<This>`
    - `{`
      - `fn CLASS_METHOD(this: &Self, {CLASS_MARG: CLASS_MARG_TY}) -> CLASS_MRET_TY`:
        - `fn m(this: &Ptr<CLASS>, {CLASS_MARG: CLASS_MARG_TY}) -> CLASS_MRET_TY`:
          - `CLASS_MBODY`
        - `m(&this.CLASSPtr(), {CLASS_MARG})`
    - `}`
    - `{`
      - `fn SC_METHOD(this: &Self, {SC_MARG: SC_MARG_TY}) -> SC_MRET_TY`:
        - `fn m(this: &Ptr<SC>, {SC_MARG: SC_MARG_TY}) -> SC_MRET_TY`:
          - `SC_MBODY`
        - `m(&this.SCPtr(), {SC_MARG})`
    - `}`
    
   
