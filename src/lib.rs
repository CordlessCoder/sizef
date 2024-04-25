use std::fmt::{Debug, Display, Write};
mod constants;
mod convenience;
pub use constants::*;
pub use convenience::*;

pub struct Size(BYTES);

impl Display for Size {
    /// Formats the contained size with non-SI units(KiB, real powers of 2), into the first unit it
    /// converts to as non-zero
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_with_units(f, UNITS.iter().copied())
    }
}

impl Debug for Size {
    /// Formats the contained size with SI units, into the first unit it converts to as non-zero
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_with_units(f, UNITS_SI.iter().copied())
    }
}

impl<'a> Size {
    fn fmt_with_units<I>(&self, f: &mut std::fmt::Formatter<'_>, units: I) -> std::fmt::Result
    where
        I: Iterator<Item = ByteUnit<'a>> + std::iter::DoubleEndedIterator,
    {
        let bytes = self.0;
        let Some(ByteUnit { size, name }) = units
            // Iterate from the end of the units(largest unit) towards the smallest unit
            .rev()
            .find(|unit| bytes >= unit.size)
        else {
            return f.write_char('0');
        };
        let converted = bytes / size;
        f.write_fmt(format_args!("{converted}{name}"))
    }
}

pub struct LongSize(BYTES);

impl Display for LongSize {
    /// Formats the contained size with non-SI units(KiB, real powers of 2),
    /// into every unit it converts into which ends up non-zero
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_with_units(f, UNITS.iter().copied())
    }
}

impl Debug for LongSize {
    /// Formats the contained size with SI units, printing every non-zero unit it converts into
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_with_units(f, UNITS_SI.iter().copied())
    }
}

impl<'a> LongSize {
    fn fmt_with_units<I>(&self, f: &mut std::fmt::Formatter<'_>, units: I) -> std::fmt::Result
    where
        I: Iterator<Item = ByteUnit<'a>> + std::iter::DoubleEndedIterator,
    {
        let mut bytes = self.0;
        let mut units = units
            // Iterate from the end of the units(largest unit) towards the smallest unit
            .rev();
        // Write first unit without a space at the beginning
        let first_unit = 'firstunit: {
            for unit in &mut units {
                if bytes >= unit.size {
                    break 'firstunit Some(unit);
                }
            }
            None
        };
        let Some(ByteUnit { size, name }) = first_unit else {
            return f.write_char('0');
        };
        let converted = bytes / size;
        bytes -= converted * size;
        write!(f, "{converted}{name}")?;
        for ByteUnit { size, name } in &mut units {
            // Filter to only include units that are bigger than the size we're trying to format
            if bytes < size {
                continue;
            }
            let converted = bytes / size;
            bytes -= converted * size;
            write!(f, " {converted}{name}")?
        }

        Ok(())
    }
}

pub struct DecimalSize(BYTES);

impl Display for DecimalSize {
    /// Formats the contained size with non-SI units(KiB, real powers of 2), into the first unit it
    /// converts to as non-zero
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_with_units(f, UNITS.iter().copied())
    }
}

impl Debug for DecimalSize {
    /// Formats the contained size with SI units, into the first unit it converts to as non-zero
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_with_units(f, UNITS_SI.iter().copied())
    }
}

impl<'a> DecimalSize {
    fn fmt_with_units<I>(&self, f: &mut std::fmt::Formatter<'_>, units: I) -> std::fmt::Result
    where
        I: Iterator<Item = ByteUnit<'a>> + std::iter::DoubleEndedIterator,
    {
        let bytes = self.0;
        let Some(ByteUnit { size, name }) = units
            // Iterate from the end of the units(largest unit) towards the smallest unit
            .rev()
            // Filter to only include units that are bigger than the size we're trying to format
            .find(|unit| bytes >= unit.size)
        else {
            return f.write_char('0');
        };
        let converted = bytes as f32 / size as f32;

        let round = converted.fract() < 0.1;
        if round {
            f.write_fmt(format_args!("{converted:.0}{name}"))
        } else {
            f.write_fmt(format_args!("{converted:.1}{name}"))
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ByteUnit<'a> {
    size: BYTES,
    name: &'a str,
}

impl<'a> ByteUnit<'a> {
    pub const fn new(size: BYTES, name: &'a str) -> Self {
        Self { size, name }
    }
}

mod tests;
