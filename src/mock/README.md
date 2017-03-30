Example:

```
class Counter {
    struct CounterPrivate {
        f: Cell<u32>, dc: Option<DropCounter>
    }

    init() -> CounterPrivate {
        CounterPrivate { f: Cell::new(0), dc: None }
    }
    
    fn add(this, v: u32) -> u32 {
        // Type of this: &G<Counter> ?
        let private = this.private();
        let v = private.f.get() + v;
        private.f.set(v);
        v
    }
    
    fn get(this) -> u32 {
        this.private().f.get()
    }
}
```
