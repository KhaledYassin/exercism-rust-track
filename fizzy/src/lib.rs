use std::ops::Rem;

type Match<T> = Box<dyn Fn(T) -> bool>;

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<T> {
    function: Match<T>,
    subs: String,
}

impl<T> Matcher<T> {
    pub fn new<F, S>(matcher: F, subs: S) -> Matcher<T>
    where
        F: Fn(T) -> bool + 'static,
        S: ToString,
    {
        Matcher {
            function: Box::new(matcher),
            subs: subs.to_string(),
        }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
pub struct Fizzy<T> {
    matchers: Vec<Matcher<T>>,
}

impl<T> Default for Fizzy<T>
where
    T: ToString + Copy,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Fizzy<T>
where
    T: ToString + Copy,
{
    pub fn new() -> Self {
        Fizzy { matchers: vec![] }
    }

    // feel free to change the signature to `mut self` if you like
    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.matchers.push(matcher);
        self
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String>
    where
        I: Iterator<Item = T>,
    {
        iter.map(move |item| self.compute(item))
    }

    fn compute(&self, item: T) -> String {
        let mut string = String::new();
        for matcher in &self.matchers {
            if (matcher.function)(item) {
                string.push_str(&matcher.subs)
            }
        }

        if string.is_empty() {
            string.push_str(&item.to_string())
        }

        string
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: Copy + Default + From<u8> + Rem<Output = T> + PartialEq,
{
    let matchers = vec![
        Matcher::new(|item| item % 3.into() == T::default(), "fizz"),
        Matcher::new(|item| item % 5.into() == T::default(), "buzz"),
    ];
    Fizzy { matchers }
}
