use std::thread::sleep;
use std::time::Duration;

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

fn expensive_job(_i: &i32) {
    sleep(Duration::from_secs(1))
}
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

fn main() {
    let v = vec![1, 2, 3];
    for n in v.iter().progress().with_bounds().with_brackets(('<', '>')) {
        expensive_job(n);
    }

    for n in (0..).progress() {
        expensive_job(&0);
    }
}
