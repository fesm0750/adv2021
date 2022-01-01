use std::{
    convert::{TryFrom, TryInto},
    error::Error,
    fmt::Debug,
    ops::Add,
    str::FromStr,
};

/// Helper struct for representing 2d values, i.e: coordinates, indexes, etc.
#[derive(Copy, Clone, Debug)]
pub struct Base2d<U> {
    pub x: U,
    pub y: U,
}

impl<U: Copy> Base2d<U> {
    /// Constructs a new Base2d
    pub fn new(x: U, y: U) -> Base2d<U> {
        Base2d { x, y }
    }

    /// Returns a tuple `(x, y)`.
    pub fn tuple(&self) -> (U, U) {
        (self.x, self.y)
    }
}

impl<U: PartialEq> Base2d<U> {
    pub fn is_same_column(&self, rhs: &Self) -> bool {
        self.x == rhs.x
    }

    pub fn is_same_row(&self, rhs: &Self) -> bool {
        self.y == rhs.y
    }
}

impl<U> Add for Base2d<U>
where
    U: Add<Output = U>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

//--------------------------------------------------------------------
// Conversion traits
//--------------------------------------------------------------------
// Defines how to convert a tuple (U, U) to a Base2d<usize>. In practice, allows
// to use tuples, in some situations, as a more handy alternative instead of
// instaciating the Base2d struct.

impl<U> TryFrom<(U, U)> for Base2d<usize>
where
    U: TryInto<usize>,
    <U as TryInto<usize>>::Error: std::error::Error + 'static,
{
    type Error = Box<dyn Error>;

    fn try_from(item: (U, U)) -> Result<Self, Self::Error> {
        Ok(Base2d {
            x: item.0.try_into()?,
            y: item.1.try_into()?,
        })
    }
}

impl<U> FromStr for Base2d<U>
where
    U: FromStr + Copy,
    <U as FromStr>::Err: std::error::Error + 'static,
{
    type Err = Box<dyn Error>;

    /// string needs to have two values separated by comma (','). Example: "15,21"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(',');
        let x = iter
            .next()
            .ok_or("Could not parse the number before the comma.")?
            .parse::<U>()?;
        let y = iter
            .next()
            .ok_or("Could not parse the number after the comma.")?
            .parse::<U>()?;
        Ok(Base2d::new(x, y))
    }
}

// impl<U> From<(U, U)> for Base2d<usize>
// where
//     U: Into<usize>,
// {
//     fn from(item: (U, U)) -> Self {
//         Base2d {
//             x: item.0.into(),
//             y: item.1.into(),
//         }
//     }
// }
