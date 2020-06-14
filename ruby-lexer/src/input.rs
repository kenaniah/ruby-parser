use core::ops::{RangeFrom, RangeTo};
use nom::error::ErrorKind;
use nom::error::ParseError;
use nom::{AsBytes, Err, IResult, InputIter, InputLength, InputTake, InputTakeAtPosition, Slice};

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

impl<T: AsBytes, X: Default> From<T> for TrackedLocation<T, X> {
    fn from(program: T) -> Self {
        Self::new_with_metadata(program, X::default())
    }
}

impl<T: PartialEq, X> PartialEq for TrackedLocation<T, X> {
    fn eq(&self, other: &Self) -> bool {
        self.offset == other.offset
            && self.line == other.line
            && self.char == other.char
            && self.input == other.input
    }
}

impl<T: Eq, X> Eq for TrackedLocation<T, X> {}

impl<T: AsBytes, X> AsBytes for TrackedLocation<T, X> {
    fn as_bytes(&self) -> &[u8] {
        self.input.as_bytes()
    }
}

impl<T: InputLength, X> InputLength for TrackedLocation<T, X> {
    fn input_len(&self) -> usize {
        self.input.input_len()
    }
}

impl<T, X> InputTake for TrackedLocation<T, X>
where
    Self: Slice<RangeFrom<usize>> + Slice<RangeTo<usize>>,
{
    fn take(&self, count: usize) -> Self {
        self.slice(..count)
    }
    fn take_split(&self, count: usize) -> (Self, Self) {
        (self.slice(count..), self.slice(..count))
    }
}

impl<T, X> InputTakeAtPosition for TrackedLocation<T, X>
where
    T: InputTakeAtPosition + InputLength + InputIter,
    Self: Slice<RangeFrom<usize>> + Slice<RangeTo<usize>> + Clone,
{
    type Item = <T as InputIter>::Item;

    fn split_at_position_complete<P, E: ParseError<Self>>(
        &self,
        predicate: P,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.split_at_position(predicate) {
            Err(Err::Incomplete(_)) => Ok(self.take_split(self.input_len())),
            res => res,
        }
    }

    fn split_at_position<P, E: ParseError<Self>>(&self, predicate: P) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.input.position(predicate) {
            Some(n) => Ok(self.take_split(n)),
            None => Err(Err::Incomplete(nom::Needed::Size(unsafe {
                std::num::NonZeroUsize::new_unchecked(1)
            }))),
        }
    }

    fn split_at_position1<P, E: ParseError<Self>>(
        &self,
        predicate: P,
        e: ErrorKind,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.input.position(predicate) {
            Some(0) => Err(Err::Error(E::from_error_kind(self.clone(), e))),
            Some(n) => Ok(self.take_split(n)),
            None => Err(Err::Incomplete(nom::Needed::Size(unsafe {
                std::num::NonZeroUsize::new_unchecked(1)
            }))),
        }
    }

    fn split_at_position1_complete<P, E: ParseError<Self>>(
        &self,
        predicate: P,
        e: ErrorKind,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.input.position(predicate) {
            Some(0) => Err(Err::Error(E::from_error_kind(self.clone(), e))),
            Some(n) => Ok(self.take_split(n)),
            None => {
                if self.input.input_len() == 0 {
                    Err(Err::Error(E::from_error_kind(self.clone(), e)))
                } else {
                    Ok(self.take_split(self.input_len()))
                }
            }
        }
    }
}
