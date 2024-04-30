##
```rust
fn expensive_job(_i: &i32) {
    sleep(Duration::from_secs(1))
}

fn progress<T, Iter>(v: Iter, f: fn(T) -> ())
    where Iter: Iterator<Item=T>
{
    let mut i = 1;
    for n in v {
        println!("{}", CLEAR);
        println!("{}", "*".repeat(i));
        f(n);
        i += 1;
    }
}

fn main() {
    let v = vec![1, 2, 3];
    progress(v.iter(), expensive_job);
}
```

## Struct
```rust
struct Progress<Iter> {
    iter: Iter,
    i: usize,
}

impl<Iter> Progress<Iter> {
    pub fn new(iter: Iter) -> Self {
        Self {
            iter,
            i: 0,
        }
    }
}

impl<Iter> Iterator for Progress<Iter>
    where Iter: Iterator {
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.i += 1;
        self.iter.next()
    }
}
```
## Trait
```rust
trait ProgressIteratorExt: Sized {
    fn progress(self) -> Progress<Self>;
}

impl<Iter> ProgressIteratorExt for Iter {
    fn progress(self) -> Progress<Self> {
        Progress::new(self)
    }
}
```

## ExactSizeIterator
```rust

impl<Iter> Progress<Iter> {
    pub fn new(iter: Iter) -> Self {
        Self {
            iter,
            i: 0,
            bound: None
        }
    }
}

impl<Iter> Progress<Iter> where Iter: ExactSizeIterator {
    pub fn with_bounds(mut self) -> Self {
        self.bound = Some(self.iter.len());
        self
    }
}

impl<Iter> Iterator for Progress<Iter>
    where Iter: Iterator {
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        println!("{}", CLEAR);
        match self.bound {
            Some(b) => println!("[{}{}]", "*".repeat(self.i), ".".repeat(b - self.i)),
            None => println!("{}", "*".repeat(self.i)),
        };
        self.i += 1;
        self.iter.next()
    }
}
```

## Brackets
```rust
impl<Iter> Progress<Iter> {
    pub fn new(iter: Iter) -> Self {
        Self {
            iter,
            i: 0,
            bound: None,
            brackets: ('[', ']'),
        }
    }
}

impl<Iter> Progress<Iter> where Iter: ExactSizeIterator {
    pub fn with_bounds(mut self) -> Self {
        self.bound = Some(self.iter.len());
        self
    }
}
```
## With brackets
```rust
impl <Iter> Progress<Iter> {
    pub fn with_brackets(mut self, brackets: (char, char)) -> Self {
        self.brackets = brackets;
        self
    }
}
```

### Struct Bounded and Unbounded
```rust
struct Bounded {
    bound: usize,
    brackets: (char, char)
}
struct Unbounded;

struct Progress<Iter, Bound> {
    iter: Iter,
    i: usize,
    bound: Bound
}

trait ProgressDisplay: Sized {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>);
}

impl ProgressDisplay for Unbounded {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
        println!("{}", "*".repeat(progress.i));
    }
}

impl ProgressDisplay for Bounded {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
        println!("{}{}{}{}", self.brackets.0,
                 "*".repeat(progress.i),
                 ".".repeat(self.bound - progress.i),
                 self.brackets.1)
    }
}

impl<Iter> Progress<Iter, Unbounded> {
    pub fn new(iter: Iter) -> Self {
        Self {
            iter,
            i: 0,
            bound: Unbounded
        }
    }
}

impl<Iter> Progress<Iter, Unbounded> where Iter: ExactSizeIterator {
    pub fn with_bounds(mut self) -> Progress<Iter, Bounded> {
        let bound = Bounded {
            brackets: ('[', ']'),
            bound: self.iter.len()
        };
        Progress {
            iter: self.iter,
            i: self.i,
            bound,
        }
    }
}

impl <Iter> Progress<Iter, Bounded> {
    pub fn with_brackets(mut self, brackets: (char, char)) -> Self {
        self.bound.brackets = brackets;
        self
    }
}

impl<Iter, Bound> Iterator for Progress<Iter, Bound>
    where Iter: Iterator, Bound: ProgressDisplay {
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        println!("{}", CLEAR);
        self.bound.display(self);
        self.i += 1;
        self.iter.next()
    }
}

trait ProgressIteratorExt: Sized {
    fn progress(self) -> Progress<Self, Unbounded>;
}

impl<Iter> ProgressIteratorExt for Iter {
    fn progress(self) -> Progress<Self, Unbounded> {
        Progress::new(self)
    }
}
```