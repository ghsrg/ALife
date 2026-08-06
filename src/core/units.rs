#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Tick(u64);

impl Tick {
    pub const fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u64 {
        self.0
    }

    pub const fn next(self) -> Self {
        Self(self.0 + 1)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Seed(u64);

impl Seed {
    pub const fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AmountError {
    Negative,
    OutOfRange,
    NonFinite,
}

macro_rules! non_negative_unit {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
        #[repr(transparent)]
        pub struct $name(f32);

        impl $name {
            pub fn new(value: f32) -> Result<Self, AmountError> {
                validate_non_negative(value).map(Self)
            }

            pub const fn raw(self) -> f32 {
                self.0
            }
        }
    };
}

macro_rules! unit_interval {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
        #[repr(transparent)]
        pub struct $name(f32);

        impl $name {
            pub fn new(value: f32) -> Result<Self, AmountError> {
                if !value.is_finite() {
                    return Err(AmountError::NonFinite);
                }
                if !(0.0..=1.0).contains(&value) {
                    return Err(AmountError::OutOfRange);
                }
                Ok(Self(value))
            }

            pub const fn raw(self) -> f32 {
                self.0
            }
        }
    };
}

non_negative_unit!(Volume);
non_negative_unit!(DiffusionRate);
non_negative_unit!(EnergyValue);
non_negative_unit!(DecayRate);
non_negative_unit!(EnergyCapacity);
unit_interval!(Strength);
unit_interval!(SignalAmount);

fn validate_non_negative(value: f32) -> Result<f32, AmountError> {
    if !value.is_finite() {
        return Err(AmountError::NonFinite);
    }
    if value < 0.0 {
        return Err(AmountError::Negative);
    }
    Ok(value)
}

macro_rules! amount_type {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
        #[repr(transparent)]
        pub struct $name(f32);

        impl $name {
            pub fn new(value: f32) -> Result<Self, AmountError> {
                validate_non_negative(value).map(Self)
            }

            #[allow(dead_code)]
            pub(crate) const fn new_unchecked(value: f32) -> Self {
                Self(value)
            }

            pub const fn zero() -> Self {
                Self(0.0)
            }

            pub const fn raw(self) -> f32 {
                self.0
            }

            pub fn saturating_add(self, rhs: Self) -> Self {
                Self((self.0 + rhs.0).max(0.0))
            }

            pub fn saturating_sub(self, rhs: Self) -> Self {
                Self((self.0 - rhs.0).max(0.0))
            }

            pub fn clamp_max(self, max: Self) -> Self {
                Self(self.0.min(max.0).max(0.0))
            }
        }
    };
}

amount_type!(EnergyAmount);
amount_type!(ResourceAmount);
amount_type!(MaterialAmount);
amount_type!(CapacityAmount);
amount_type!(HeatAmount);
amount_type!(WasteAmount);
amount_type!(FieldValue);

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
pub struct Temperature(f32);

impl Temperature {
    pub const fn new(value: f32) -> Self {
        Self(value)
    }

    pub const fn raw(self) -> f32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    x: f32,
    y: f32,
}

impl Position {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub const fn x(self) -> f32 {
        self.x
    }

    pub const fn y(self) -> f32 {
        self.y
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
pub struct Radius(f32);

impl Radius {
    pub fn new(value: f32) -> Result<Self, AmountError> {
        if !value.is_finite() {
            return Err(AmountError::NonFinite);
        }
        if value <= 0.0 {
            return Err(AmountError::Negative);
        }
        Ok(Self(value))
    }

    #[allow(dead_code)]
    pub(crate) const fn new_unchecked(value: f32) -> Self {
        Self(value)
    }

    pub const fn raw(self) -> f32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WorldSize {
    width: f32,
    height: f32,
}

impl WorldSize {
    pub fn new(width: f32, height: f32) -> Result<Self, AmountError> {
        if !width.is_finite() || !height.is_finite() {
            return Err(AmountError::NonFinite);
        }
        if width <= 0.0 || height <= 0.0 {
            return Err(AmountError::Negative);
        }
        Ok(Self { width, height })
    }

    pub const fn width(self) -> f32 {
        self.width
    }

    pub const fn height(self) -> f32 {
        self.height
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GridCoord {
    x: usize,
    y: usize,
}

impl GridCoord {
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub const fn x(self) -> usize {
        self.x
    }

    pub const fn y(self) -> usize {
        self.y
    }
}
