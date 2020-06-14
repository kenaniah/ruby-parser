use core::ops::{Deref, Range, RangeFrom, RangeFull, RangeTo};
use core::str::{CharIndices, Chars, FromStr};
use nom::error::{ErrorKind, ParseError};
use nom::{
    AsBytes, Compare, CompareResult, Err, ExtendInto, FindSubstring, FindToken, IResult, InputIter,
    InputLength, InputTake, InputTakeAtPosition, Offset, ParseTo, Slice,
};

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

impl<T, X> Deref for TrackedLocation<T, X> {
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
            char: 1,
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
            char: 1,
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

impl<T: AsBytes, X: Default> TrackedLocation<T, X> {
    pub fn new_with_pos(program: T, offset: usize, line: usize, char: usize) -> Self {
        Self {
            offset: offset,
            line: line,
            char: char,
            input: program,
            metadata: X::default()
        }
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

impl<'a, X> InputIter for TrackedLocation<&'a str, X> {
    type Item = char;
    type Iter = CharIndices<'a>;
    type IterElem = Chars<'a>;

    fn iter_indices(&self) -> Self::Iter {
        self.input.iter_indices()
    }
    fn iter_elements(&self) -> Self::IterElem {
        self.input.iter_elements()
    }
    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.input.position(predicate)
    }
    fn slice_index(&self, count: usize) -> Option<usize> {
        self.input.slice_index(count)
    }
}

impl<'a> IntoIterator for TrackedLocation<&'a str> {
    type Item = char;
    type IntoIter = Chars<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.input.chars()
    }
}

impl<A: Compare<B>, B: Into<TrackedLocation<B>>, X> Compare<B> for TrackedLocation<A, X> {
    fn compare(&self, other: B) -> CompareResult {
        self.input.compare(other.into().input)
    }
    fn compare_no_case(&self, other: B) -> CompareResult {
        self.input.compare_no_case(other.into().input)
    }
}

impl<T, X> Offset for TrackedLocation<T, X> {
    fn offset(&self, second: &Self) -> usize {
        second.offset - self.offset
    }
}

macro_rules! impl_slice_range {
    ( $fragment_type:ty, $range_type:ty, $can_return_self:expr ) => {
        impl<'a, X: Clone> Slice<$range_type> for TrackedLocation<$fragment_type, X> {
            fn slice(&self, range: $range_type) -> Self {
                if $can_return_self(&range) {
                    return self.clone();
                }
                let next_fragment = self.input.slice(range);
                let consumed_len = self.input.offset(&next_fragment);
                if consumed_len == 0 {
                    return Self {
                        line: self.line,
                        char: self.char,
                        offset: self.offset,
                        input: next_fragment,
                        metadata: self.metadata.clone(),
                    };
                }

                let consumed = self.input.slice(..consumed_len);
                let next_offset = self.offset + consumed_len;

                let consumed_as_bytes = consumed.as_bytes();
                let iter = memchr::Memchr::new(b'\n', consumed_as_bytes);
                let number_of_lines = iter.count();
                let next_line = self.line + number_of_lines;
                let next_char = if number_of_lines == 0 {
                    self.char + consumed.chars().count()
                } else {
                    1 + consumed.chars().rev().position(|c| c == '\n').unwrap()
                };

                Self {
                    line: next_line,
                    char: next_char,
                    offset: next_offset,
                    input: next_fragment,
                    metadata: self.metadata.clone(),
                }
            }
        }
    };
}

macro_rules! impl_slice_ranges {
    ( $fragment_type:ty ) => {
        impl_slice_range! {$fragment_type, Range<usize>, |_| false }
        impl_slice_range! {$fragment_type, RangeTo<usize>, |_| false }
        impl_slice_range! {$fragment_type, RangeFrom<usize>, |range:&RangeFrom<usize>| range.start == 0}
        impl_slice_range! {$fragment_type, RangeFull, |_| true}
    }
}
impl_slice_ranges! {&'a str}

impl<T: FindToken<Token>, Token, X> FindToken<Token> for TrackedLocation<T, X> {
    fn find_token(&self, token: Token) -> bool {
        self.input.find_token(token)
    }
}

impl<'a, T: FindSubstring<&'a str>, X> FindSubstring<&'a str> for TrackedLocation<T, X> {
    fn find_substring(&self, substr: &'a str) -> Option<usize> {
        self.input.find_substring(substr)
    }
}

impl<R: FromStr, T: ParseTo<R>, X> ParseTo<R> for TrackedLocation<T, X> {
    fn parse_to(&self) -> Option<R> {
        self.input.parse_to()
    }
}

impl<T: ToString, X> std::fmt::Display for TrackedLocation<T, X> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(&self.input.to_string())
    }
}

impl<'a, I, E, T, X> ExtendInto for TrackedLocation<T, X>
where
    E: Default + Extend<I>,
    T: ExtendInto<Item = I, Extender = E>,
    Self: Clone + IntoIterator<Item = I>,
{
    type Item = I;
    type Extender = E;

    fn new_builder(&self) -> Self::Extender {
        Self::Extender::default()
    }

    fn extend_into(&self, acc: &mut Self::Extender) {
        acc.extend(self.clone().into_iter())
    }
}

/// Captures the current position within the input
pub fn position<T, E>(i: T) -> IResult<T, T, E>
where
    E: ParseError<T>,
    T: InputIter + InputTake,
{
    nom::bytes::complete::take(0usize)(i)
}
