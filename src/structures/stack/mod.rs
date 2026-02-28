use std::fmt;

pub struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self { data: Vec::with_capacity(capacity) }
    }

    // --- Mutating ---

    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn pop_unchecked(&mut self) -> T {
        self.data.pop().expect("called pop_unchecked on an empty stack")
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    // --- Inspection ---

    pub fn top(&self) -> Option<&T> {
        self.data.last()
    }

    pub fn top_unchecked(&self) -> &T {
        self.data.last().expect("called top_unchecked on an empty stack")
    }

    pub fn top_mut(&mut self) -> Option<&mut T> {
        self.data.last_mut()
    }

    pub fn top_mut_unchecked(&mut self) -> &mut T {
        self.data.last_mut().expect("called top_mut_unchecked on an empty stack")
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    // --- Utilities ---

    pub fn swap(&mut self, other: &mut Self) {
        std::mem::swap(self, other);
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter().rev()
    }
}

// --- Default ---

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

// --- From<Vec<T>> ---

impl<T> From<Vec<T>> for Stack<T> {
    fn from(vec: Vec<T>) -> Self {
        Self { data: vec }
    }
}

// --- Into owned iterator (top → bottom) ---

impl<T> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = std::iter::Rev<std::vec::IntoIter<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter().rev()
    }
}

// --- Borrow iterator (top → bottom) ---

impl<'a, T> IntoIterator for &'a Stack<T> {
    type Item = &'a T;
    type IntoIter = std::iter::Rev<std::slice::Iter<'a, T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter().rev()
    }
}

// --- Debug ---

impl<T: fmt::Debug> fmt::Debug for Stack<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.data.iter().rev()).finish()
    }
}

#[macro_export]
macro_rules! stack {
    () => {
        $crate::Stack::new()
    };
    ($($val:expr),+ $(,)?) => {
        $crate::Stack::from(vec![$($val),+])
    };
}
// ---------- unit tests -----------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_push_pop() {
        let mut s = Stack::new();
        assert!(s.is_empty());
        s.push(10);
        s.push(20);
        assert_eq!(s.len(), 2);
        assert_eq!(s.top(), Some(&20));
        assert_eq!(s.pop(), Some(20));
        assert_eq!(s.pop_unchecked(), 10);
        assert!(s.is_empty());
    }

    #[test]
    fn iterators_and_conversion() {
        let mut s = stack![1, 2, 3];
        let collected: Vec<_> = (&s).into_iter().cloned().collect();
        assert_eq!(collected, vec![3, 2, 1]);
        let collected2: Vec<_> = s.into_iter().collect();
        assert_eq!(collected2, vec![3, 2, 1]);
    }

    #[test]
    fn swap_and_clear() {
        let mut a = stack![1, 2];
        let mut b = stack![5];
        a.swap(&mut b);
        assert_eq!(a.top_unchecked(), &5);
        assert_eq!(b.top_unchecked(), &2);
        a.clear();
        assert!(a.is_empty());
    }

    #[test]
    fn macro_empty() {
        let s: Stack<i32> = stack![];
        assert!(s.is_empty());
    }
}
