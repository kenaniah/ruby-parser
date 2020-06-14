use nom::AsBytes;

/// Tracks location information and user-defined metadata for nom's source input.
#[derive(Debug, Clone, Copy)]
pub struct TrackedLocation<T, X = ()> {
    /// The offset represents the current byte position relative to the original input
    offset: usize,
    /// Tracks the current line number (starts at 1)
    line: usize,
    /// Tracks the current character number (starts at 1, UTF8-aware)
    char: usize,
    /// A slice representing the remaining input
    input: T,
    /// Any user-defined metadata that should be tracked in addition to the location
    pub metadata: X,
}

impl<T, X> core::ops::Deref for TrackedLocation<T, X> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.input
    }
}

impl<T: AsBytes> TrackedLocation<T, ()> {
    pub fn new(program: T) -> Self {
        Self {
            offset: 0,
            line: 1,
            char: 0,
            input: program,
            metadata: (),
        }
    }
}

impl<T: AsBytes, X> TrackedLocation<T, X> {
    pub fn new_with_metadata(program: T, metadata: X) -> Self {
        Self {
            offset: 0,
            line: 1,
            char: 0,
            input: program,
            metadata: metadata,
        }
    }
    pub fn offset(&self) -> usize {
        self.offset
    }
    pub fn line(&self) -> usize {
        self.line
    }
    pub fn char(&self) -> usize {
        self.char
    }
    pub fn input(&self) -> &T {
        &self.input
    }
    pub fn metadata(&self) -> &X {
        &self.metadata
    }
}
