use super::{ByteUnit, DecimalSize, LongSize, Size, BYTES};

impl Size {
    pub fn new(bytes: BYTES) -> Self {
        Self(bytes)
    }

    pub fn into_exact(&self) -> LongSize {
        LongSize(self.0)
    }
}

impl From<BYTES> for Size {
    fn from(value: BYTES) -> Self {
        Self(value)
    }
}

impl From<LongSize> for Size {
    fn from(value: LongSize) -> Self {
        Self(value.0)
    }
}

impl From<DecimalSize> for Size {
    fn from(value: DecimalSize) -> Self {
        Self(value.0)
    }
}

pub trait IntoSize {
    fn into_size(&self) -> Size;
    fn into_longsize(&self) -> LongSize;
    fn into_decimalsize(&self) -> DecimalSize;
}

impl IntoSize for u64 {
    fn into_size(&self) -> Size {
        Size::new(*self)
    }
    fn into_longsize(&self) -> LongSize {
        LongSize::new(*self)
    }
    fn into_decimalsize(&self) -> DecimalSize {
        DecimalSize(*self)
    }
}

impl LongSize {
    pub fn new(bytes: BYTES) -> Self {
        Self(bytes)
    }
}

impl From<BYTES> for LongSize {
    fn from(value: BYTES) -> Self {
        Self(value)
    }
}

impl From<Size> for LongSize {
    fn from(value: Size) -> Self {
        Self(value.0)
    }
}

impl From<DecimalSize> for LongSize {
    fn from(value: DecimalSize) -> Self {
        Self(value.0)
    }
}

impl DecimalSize {
    pub fn new(bytes: BYTES) -> Self {
        Self(bytes)
    }
}

impl From<BYTES> for DecimalSize {
    fn from(value: BYTES) -> Self {
        Self(value)
    }
}

impl From<Size> for DecimalSize {
    fn from(value: Size) -> Self {
        Self(value.0)
    }
}

impl From<LongSize> for DecimalSize {
    fn from(value: LongSize) -> Self {
        Self(value.0)
    }
}

impl<'a> From<(BYTES, &'a str)> for ByteUnit<'a> {
    fn from((size, name): (BYTES, &'a str)) -> Self {
        Self { size, name }
    }
}

impl<'a> ByteUnit<'a> {
    pub const fn from_tuple((size, name): (BYTES, &'a str)) -> Self {
        Self { size, name }
    }
}
