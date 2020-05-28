#![feature(min_specialization)]
#![feature(const_fn)]
//! A data model is the choices of bit width of integer types by each platform.
//!
//! This library is used to lookup the sizes of various C-types of a data model.
//!
//! # Examples
//!
//! ```
//! use data_models::*;
//! let model = DataModel::LP64; // e.g. Linux
//! let p = model.size_of::<Pointer>();
//! assert_eq!(p, 8);
//! ```
//!

/// A data model is the choices of bit width of integer types by each platform.
///
/// # Examples
///
/// ```
/// use data_models::*;
/// let model = DataModel::LP64; // e.g. Linux
/// let p = model.size_of::<Pointer>();
/// assert_eq!(p, 8);
/// ```
/// # Background
///
/// The C standard defines five base types for integers
/// * char
/// * short
/// * int
/// * long
/// * long long
///
/// The standard does not specify the exact number of bits for each type.
/// A platform or vendor-dependent data model specifies the exact bit widths.
///
/// The names of the models are conventions where the type is signified by a
/// letter and its size; for example, ILP32 would mean (I)nteger, (L)ong, and
/// (P)ointer are 32-bits. Although, make note, the naming scheme is not super
/// consistent.
///
/// Four data models found wide acceptance:
///
/// * LP32 or 2/4/4 (int is 16-bit, long and pointer are 32-bit)
///    M68k mac and Win16 API
///
/// * ILP32 or 4/4/4 (int, long, and pointer are 32-bit);
///    Win32 API
///    Unix and Unix-like systems (Linux, Mac OS X)
///
/// * LLP64 or 4/4/8 (int and long are 32-bit, pointer is 64-bit)
///    Win64 API
///
/// * LP64 or 4/8/8 (int is 32-bit, long and pointer are 64-bit)
///   Unix and Unix-like systems (Linux, Mac OS X)
///
/// # References
/// 1. J. R. Mashey.  The long road to 64 bits. ACM Queue Magazine, 4(8):24–35, 1996.
/// 2. T. Lauer.  Porting to Win32: A Guide to Making Your Applications Ready for the 32-Bit Future of Windows. Springer, 1996.
///
pub enum DataModel {
    //           char,  short, int, long, long long, pointer, example
    /// 16-bit integer and pointer (16-bit PDP-11)
    IP16, //  8,    --,    16,  --,   --,        16,       16-bit PDP-11
    /// 16-bit integer and pointer and 32-bit long (32-bit PDP-11)
    IP16L32, //  8,    16,    16,  32,   --,        16,       32-bit PDP-11
    /// 16-bit integer, and 32-bit long and pointer (m68k Mac & win16).
    LP32, //  8,    16,    16,  32,   64,        32,       m68k Mac; win16
    /// 32-bit integer, long, and pointer (Unix and Unix-like before mid-1990s & win32).
    ILP32, //  8,    16,    32,  32,   64,        32,       unix < mid-1990; win32
    /// 32-bit integer, long, and 64-bit pointer (windows after XP).
    LLP64, //  8,    16,    32,  32,   64,        64,       win >= XP
    /// 32-bit integer, 64-bit long and pointer (Unix/Linux after the 1990s).
    LP64, //  8,    16,    32,  64,   64,        64,       unix/linux > 1990s
    /// 64-bit integer, long and pointer (SPARC64 from hal/fujitsu I think).
    ILP64, //  8,    16,    64,  64,   64,        64,       hal/fujitsu
    /// 64-bit short, integer, long and pointer (UNICOS from Cray).
    SILP64, //  8,    64,    64,  64,   64,        64,       cray
    /// Sentinel value for unknown model.
    Unknown, //  I'd love to see more platforms here !
}

/// Char represents the `char` C type.
/// Smallest addressable unit of the machine.
/// It contains CHAR_BIT bits (typically 8).
///
/// Values stored in non-bit-field objects of any other integer type consist of
/// n × CHAR_BIT bits, where n is the size of an object of that type, in bytes.
///
/// # Example
/// ```
/// use data_models::*;
/// let model = DataModel::LP64;
/// let p = model.size_of::<Char>();
/// assert_eq!(p, 1);
/// ```
pub enum Char {}
/// Short represents the `short` C type.
/// It is required to be at least 16-bits.
///
/// # Example
/// ```
/// use data_models::*;
/// let model = DataModel::LP64;
/// let p = model.size_of::<Short>();
/// assert_eq!(p, 2);
/// ```
pub enum Short {}
/// Int represents the `int` C type.
/// It is required to be at least 16-bits.
///
/// # Example
/// ```
/// use data_models::*;
/// let model = DataModel::LP64;
/// let p = model.size_of::<Int>();
/// assert_eq!(p, 4);
/// ```
pub enum Int {}
/// Long represents the `long` C type.
/// It is required to be at least 32-bits.
///
/// # Example
/// ```
/// use data_models::*;
/// let model = DataModel::LP64;
/// let p = model.size_of::<Long>();
/// assert_eq!(p, 8);
/// ```
pub enum Long {}
/// LongLong represents the `long long` C type.
/// It is required to be at least 64-bits.
///
/// # Example
/// ```
/// use data_models::*;
/// let model = DataModel::LP64;
/// let p = model.size_of::<LongLong>();
/// assert_eq!(p, 8);
/// ```
pub enum LongLong {}
/// Pointer represents the `size_t` C type.
/// It is required to be at least 16-bits.
///
/// # Example
/// ```
/// use data_models::*;
/// let model = DataModel::LP64;
/// let p = model.size_of::<Pointer>();
/// assert_eq!(p, 8);
/// ```
pub enum Pointer {}

trait SizeOf<T> {
    fn size_of(self) -> usize;
}

impl DataModel {
    /// new tries to guess the data model from the byte size of
    /// int, long, and pointer.
    ///
    /// # Example
    /// ```
    /// use data_models::*;
    /// let model = DataModel::new(4, 8, 8); // LP64
    /// let p = model.size_of::<Pointer>();
    /// assert_eq!(p, 8);
    /// ```
    pub fn new(int: usize, long: usize, pointer: usize) -> DataModel {
        use DataModel::*;
        match (int, long, pointer) {
            (2, 0, 2) => IP16,
            (2, 4, 2) => IP16L32,
            (2, 4, 4) => LP32,
            (4, 4, 4) => ILP32,
            (4, 4, 8) => LLP64,
            (4, 8, 8) => LP64,
            (8, 8, 8) => ILP64,
            _ => Unknown,
        }
    }
    /// size_of will report the size in bytes for one of the types
    /// defined in this crate.
    /// # Example
    /// ```
    /// use data_models::*;
    /// let model = DataModel::LLP64;
    /// let p = model.size_of::<Long>();
    /// assert_eq!(p, 4);
    pub fn size_of<T>(self) -> usize {
        <DataModel as SizeOf<T>>::size_of(self)
    }
}

impl<T, U> SizeOf<T> for U {
    default fn size_of(self) -> usize {
        0
    }
}

impl SizeOf<Char> for DataModel {
    fn size_of(self) -> usize {
        use DataModel::*;
        match self {
            IP16 | IP16L32 | LP32 | ILP32 | LLP64 | LP64 | ILP64 | SILP64 => 1,
            Unknown => 0,
        }
    }
}

impl SizeOf<Short> for DataModel {
    fn size_of(self) -> usize {
        use DataModel::*;
        match self {
            IP16L32 | LP32 | ILP32 | LLP64 | LP64 | ILP64 => 2,
            SILP64 => 8,
            Unknown | IP16 => 0,
        }
    }
}

impl SizeOf<Int> for DataModel {
    fn size_of(self) -> usize {
        use DataModel::*;
        match self {
            IP16 | IP16L32 | LP32 => 2,
            ILP32 | LLP64 | LP64 => 4,
            ILP64 | SILP64 => 8,
            Unknown => 0,
        }
    }
}

impl SizeOf<Long> for DataModel {
    fn size_of(self) -> usize {
        use DataModel::*;
        match self {
            IP16L32 | LP32 | ILP32 | LLP64 => 4,
            LP64 | ILP64 | SILP64 => 8,
            Unknown | IP16 => 0,
        }
    }
}

impl SizeOf<LongLong> for DataModel {
    fn size_of(self) -> usize {
        use DataModel::*;
        match self {
            LP32 | ILP32 | LLP64 | LP64 | ILP64 | SILP64 => 8,
            Unknown | IP16 | IP16L32 => 0,
        }
    }
}

impl SizeOf<Pointer> for DataModel {
    fn size_of(self) -> usize {
        use DataModel::*;
        match self {
            IP16 | IP16L32 => 2,
            LP32 | ILP32 => 4,
            LLP64 | LP64 | ILP64 | SILP64 => 8,
            Unknown => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! sizeof_check {
        (
            $(
                $model:ident: $type:path, $expect:expr
            )
            ,
            *
        ) => {
            $(
                let model = DataModel::$model;
                let s = model.size_of::<$type>();
                assert_eq!(s, $expect);
            )*
            }
    }

    #[test]
    fn test_char() {
        sizeof_check! {
            IP16:    Char, 1,
            IP16L32: Char, 1,
            LP32:    Char, 1,
            ILP32:   Char, 1,
            LLP64:   Char, 1,
            LP64:    Char, 1,
            ILP64:   Char, 1,
            SILP64:  Char, 1,
            Unknown: Char, 0
        }
    }

    #[test]
    fn test_short() {
        sizeof_check! {
            IP16:    Short, 0,
            IP16L32: Short, 2,
            LP32:    Short, 2,
            ILP32:   Short, 2,
            LLP64:   Short, 2,
            LP64:    Short, 2,
            ILP64:   Short, 2,
            SILP64:  Short, 8,
            Unknown: Short, 0
        }
    }

    #[test]
    fn test_int() {
        sizeof_check! {
            IP16:    Int, 2,
            IP16L32: Int, 2,
            LP32:    Int, 2,
            ILP32:   Int, 4,
            LLP64:   Int, 4,
            LP64:    Int, 4,
            ILP64:   Int, 8,
            SILP64:  Int, 8,
            Unknown: Int, 0
        }
    }

    #[test]
    fn test_long() {
        sizeof_check! {
            IP16:    Long, 0,
            IP16L32: Long, 4,
            LP32:    Long, 4,
            ILP32:   Long, 4,
            LLP64:   Long, 4,
            LP64:    Long, 8,
            ILP64:   Long, 8,
            SILP64:  Long, 8,
            Unknown: Long, 0
        }
    }

    #[test]
    fn test_long_long() {
        sizeof_check! {
            IP16:    LongLong, 0,
            IP16L32: LongLong, 0,
            LP32:    LongLong, 8,
            ILP32:   LongLong, 8,
            LLP64:   LongLong, 8,
            LP64:    LongLong, 8,
            ILP64:   LongLong, 8,
            SILP64:  LongLong, 8,
            Unknown: LongLong, 0
        }
    }

    #[test]
    fn test_pointer() {
        sizeof_check! {
            IP16:    Pointer, 2,
            IP16L32: Pointer, 2,
            LP32:    Pointer, 4,
            ILP32:   Pointer, 4,
            LLP64:   Pointer, 8,
            LP64:    Pointer, 8,
            ILP64:   Pointer, 8,
            SILP64:  Pointer, 8,
            Unknown: Pointer, 0
        }
    }

    #[test]
    fn test_new() {
        // All the u8 casts are to remove the Debug and PartialEq from
        // the DataModel enum; they were making the docs look bad to me.
        assert_eq!(DataModel::IP16 as u8, DataModel::new(2, 0, 2) as u8);
        assert_eq!(DataModel::IP16L32 as u8, DataModel::new(2, 4, 2) as u8);
        assert_eq!(DataModel::LP32 as u8, DataModel::new(2, 4, 4) as u8);
        assert_eq!(DataModel::ILP32 as u8, DataModel::new(4, 4, 4) as u8);
        assert_eq!(DataModel::LLP64 as u8, DataModel::new(4, 4, 8) as u8);
        assert_eq!(DataModel::LP64 as u8, DataModel::new(4, 8, 8) as u8);
        assert_eq!(DataModel::ILP64 as u8, DataModel::new(8, 8, 8) as u8);
    }
}
