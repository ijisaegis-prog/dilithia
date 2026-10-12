//! Sealed decoder input and transactional tracked cursors.

use crate::SerializationError;

mod private {
    pub trait Sealed {}
}

/// A forward-only source of bytes for canonical decoders.
///
/// This trait is sealed. Public decoders accept both ordinary `&[u8]` inputs
/// and the private tracked cursor used at callback boundaries. A callback can
/// inspect its remaining bytes and advance by a checked amount, but it cannot
/// replace the cursor's source or move its offset backward.
///
/// An exact-decoder callback can advance within an empty source:
///
/// ```
/// use dilithia_serialization::decode_exact;
///
/// let source = Vec::<u8>::new();
/// let foreign = Vec::<u8>::new();
/// let mut input = source.as_slice();
///
/// let result = decode_exact(&mut input, |cursor| {
///     assert_eq!(cursor.remaining(), &[]);
///     cursor.advance(foreign.len())?;
///     Ok(())
/// });
/// assert_eq!(result, Ok(()));
/// assert!(input.is_empty());
/// ```
///
/// Changing only the advance into a foreign-slice assignment does not compile:
///
/// ```compile_fail,E0308
/// use dilithia_serialization::decode_exact;
///
/// let source = Vec::<u8>::new();
/// let foreign = Vec::<u8>::new();
/// let mut input = source.as_slice();
///
/// let result = decode_exact(&mut input, |cursor| {
///     assert_eq!(cursor.remaining(), &[]);
///     *cursor = foreign.as_slice();
///     Ok(())
/// });
/// assert_eq!(result, Ok(()));
/// assert!(input.is_empty());
/// ```
///
/// The same restriction applies to non-empty exact-decoder sources:
///
/// ```
/// use dilithia_serialization::decode_exact;
///
/// let source = vec![0xaa_u8];
/// let foreign = vec![0xbb_u8];
/// let mut input = source.as_slice();
///
/// let result = decode_exact(&mut input, |cursor| {
///     assert_eq!(cursor.remaining(), &[0xaa]);
///     cursor.advance(foreign.len())?;
///     Ok(())
/// });
/// assert_eq!(result, Ok(()));
/// assert!(input.is_empty());
/// ```
///
/// Changing only the advance into a foreign-slice assignment does not compile:
///
/// ```compile_fail,E0308
/// use dilithia_serialization::decode_exact;
///
/// let source = vec![0xaa_u8];
/// let foreign = vec![0xbb_u8];
/// let mut input = source.as_slice();
///
/// let result = decode_exact(&mut input, |cursor| {
///     assert_eq!(cursor.remaining(), &[0xaa]);
///     *cursor = foreign.as_slice();
///     Ok(())
/// });
/// assert_eq!(result, Ok(()));
/// assert!(input.is_empty());
/// ```
///
/// An option callback also retains its empty payload source:
///
/// ```
/// use dilithia_serialization::decode_option;
///
/// let source = vec![0x01_u8];
/// let foreign = Vec::<u8>::new();
/// let mut input = source.as_slice();
///
/// let result = decode_option(&mut input, |cursor| {
///     assert_eq!(cursor.remaining(), &[]);
///     cursor.advance(foreign.len())?;
///     Ok(())
/// });
/// assert_eq!(result, Ok(Some(())));
/// assert!(input.is_empty());
/// ```
///
/// Changing only the advance into a foreign-slice assignment does not compile:
///
/// ```compile_fail,E0308
/// use dilithia_serialization::decode_option;
///
/// let source = vec![0x01_u8];
/// let foreign = Vec::<u8>::new();
/// let mut input = source.as_slice();
///
/// let result = decode_option(&mut input, |cursor| {
///     assert_eq!(cursor.remaining(), &[]);
///     *cursor = foreign.as_slice();
///     Ok(())
/// });
/// assert_eq!(result, Ok(Some(())));
/// assert!(input.is_empty());
/// ```
///
/// A non-empty option payload cannot be replaced either:
///
/// ```
/// use dilithia_serialization::decode_option;
///
/// let source = vec![0x01_u8, 0xaa];
/// let foreign = vec![0xbb_u8];
/// let mut input = source.as_slice();
///
/// let result = decode_option(&mut input, |cursor| {
///     assert_eq!(cursor.remaining(), &[0xaa]);
///     cursor.advance(foreign.len())?;
///     Ok(())
/// });
/// assert_eq!(result, Ok(Some(())));
/// assert!(input.is_empty());
/// ```
///
/// Changing only the advance into a foreign-slice assignment does not compile:
///
/// ```compile_fail,E0308
/// use dilithia_serialization::decode_option;
///
/// let source = vec![0x01_u8, 0xaa];
/// let foreign = vec![0xbb_u8];
/// let mut input = source.as_slice();
///
/// let result = decode_option(&mut input, |cursor| {
///     assert_eq!(cursor.remaining(), &[0xaa]);
///     *cursor = foreign.as_slice();
///     Ok(())
/// });
/// assert_eq!(result, Ok(Some(())));
/// assert!(input.is_empty());
/// ```
///
/// A downstream wrapper can provide these methods on its own type:
///
/// ```
/// use dilithia_serialization::{DecoderInput, SerializationError};
///
/// struct ForeignInput<'a>(&'a [u8]);
///
/// impl ForeignInput<'_> {
///     fn remaining(&self) -> &[u8] {
///         self.0
///     }
///
///     fn advance(&mut self, count: usize) -> Result<(), SerializationError> {
///         self.0.advance(count)
///     }
/// }
///
/// let mut input = ForeignInput(&[0xaa]);
/// assert_eq!(input.remaining(), &[0xaa]);
/// assert_eq!(input.advance(1), Ok(()));
/// assert!(input.remaining().is_empty());
/// ```
///
/// Changing only the inherent implementation into a `DecoderInput`
/// implementation fails because the required sealing trait is private:
///
/// ```compile_fail,E0277
/// use dilithia_serialization::{DecoderInput, SerializationError};
///
/// struct ForeignInput<'a>(&'a [u8]);
///
/// impl DecoderInput for ForeignInput<'_> {
///     fn remaining(&self) -> &[u8] {
///         self.0
///     }
///
///     fn advance(&mut self, count: usize) -> Result<(), SerializationError> {
///         self.0.advance(count)
///     }
/// }
///
/// let mut input = ForeignInput(&[0xaa]);
/// assert_eq!(input.remaining(), &[0xaa]);
/// assert_eq!(input.advance(1), Ok(()));
/// assert!(input.remaining().is_empty());
/// ```
pub trait DecoderInput: private::Sealed {
    /// Returns the bytes that have not yet been consumed.
    fn remaining(&self) -> &[u8];

    /// Advances by `count` bytes without changing the source.
    ///
    /// An advance beyond the remaining input returns [`UnexpectedEof`] and
    /// leaves the cursor unchanged.
    ///
    /// [`UnexpectedEof`]: SerializationError::UnexpectedEof
    fn advance(&mut self, count: usize) -> Result<(), SerializationError>;
}

impl private::Sealed for &[u8] {}

impl DecoderInput for &[u8] {
    fn remaining(&self) -> &[u8] {
        self
    }

    fn advance(&mut self, count: usize) -> Result<(), SerializationError> {
        let remaining = self.get(count..).ok_or(SerializationError::UnexpectedEof)?;
        *self = remaining;
        Ok(())
    }
}

struct TrackedCursor<'a> {
    source: &'a [u8],
    offset: usize,
}

impl<'a> TrackedCursor<'a> {
    fn new(source: &'a [u8]) -> Self {
        Self { source, offset: 0 }
    }
}

impl private::Sealed for TrackedCursor<'_> {}

impl DecoderInput for TrackedCursor<'_> {
    fn remaining(&self) -> &[u8] {
        self.source
            .get(self.offset..)
            .expect("tracked cursor offset must stay within its source")
    }

    fn advance(&mut self, count: usize) -> Result<(), SerializationError> {
        let next = self
            .offset
            .checked_add(count)
            .filter(|&next| next <= self.source.len())
            .ok_or(SerializationError::UnexpectedEof)?;
        self.offset = next;
        Ok(())
    }
}

/// Runs a decoder against a private source-and-offset cursor and commits only
/// its checked forward progress after success.
pub(crate) fn transactional<T>(
    input: &mut dyn DecoderInput,
    decode: impl FnOnce(&mut dyn DecoderInput) -> Result<T, SerializationError>,
) -> Result<T, SerializationError> {
    let (result, consumed) = {
        let mut cursor = TrackedCursor::new(input.remaining());
        let result = decode(&mut cursor);
        (result, cursor.offset)
    };

    match result {
        Ok(value) => {
            input.advance(consumed)?;
            Ok(value)
        }
        Err(error) => Err(error),
    }
}

#[cfg(test)]
mod tests {
    use super::{DecoderInput, TrackedCursor, transactional};
    use crate::SerializationError;

    #[test]
    fn tracked_cursor_uses_checked_forward_offsets() {
        let bytes = [0xaa, 0xbb];
        let mut cursor = TrackedCursor::new(&bytes);

        assert_eq!(cursor.remaining(), bytes);
        assert_eq!(cursor.advance(1), Ok(()));
        assert_eq!(cursor.remaining(), &[0xbb]);
        assert_eq!(
            cursor.advance(usize::MAX),
            Err(SerializationError::UnexpectedEof)
        );
        assert_eq!(cursor.remaining(), &[0xbb]);
    }

    #[test]
    fn tracked_cursor_rejects_out_of_bounds_without_arithmetic_overflow() {
        let bytes = [0xaa, 0xbb];
        let mut cursor = TrackedCursor::new(&bytes);

        assert_eq!(
            cursor.advance(usize::MAX),
            Err(SerializationError::UnexpectedEof)
        );
        assert_eq!(cursor.remaining(), bytes);
    }

    #[test]
    fn slice_decoder_input_rejects_usize_max_without_advancing() {
        let bytes = [0xaa, 0xbb];
        let mut input = bytes.as_slice();

        assert_eq!(
            input.advance(usize::MAX),
            Err(SerializationError::UnexpectedEof)
        );
        assert_eq!(input, bytes);
    }

    #[test]
    fn tracked_cursor_accepts_zero_advance_at_end() {
        let bytes = [0xaa, 0xbb];
        let mut cursor = TrackedCursor::new(&bytes);

        assert_eq!(cursor.advance(bytes.len()), Ok(()));
        assert!(cursor.remaining().is_empty());
        assert_eq!(cursor.advance(0), Ok(()));
        assert!(cursor.remaining().is_empty());
    }
    #[test]
    fn transactional_cursor_commits_success_and_rolls_back_failure() {
        let bytes = [0xaa, 0xbb];
        let mut input = bytes.as_slice();

        assert_eq!(
            transactional(&mut input, |cursor| {
                cursor.advance(1)?;
                Ok(0xaa_u8)
            }),
            Ok(0xaa)
        );
        assert_eq!(input, &[0xbb]);

        let original = input;
        assert_eq!(
            transactional(&mut input, |cursor| {
                cursor.advance(1)?;
                Err::<(), _>(SerializationError::InvalidBool)
            }),
            Err(SerializationError::InvalidBool)
        );
        assert_eq!(input, original);
    }
}
