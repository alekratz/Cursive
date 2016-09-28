use std::cmp::min;
use XY;
use vec::Vec2;

/// Location of the view on screen
pub type Position = XY<Offset>;

impl Position {
    /// Returns a position centered on both axis.
    pub fn center() -> Self {
        Position::new(Offset::Center, Offset::Center)
    }

    /// Returns a position absolute on both axis.
    pub fn absolute<T: Into<Vec2>>(offset: T) -> Self {
        let offset = offset.into();
        Position::new(Offset::Absolute(offset.x), Offset::Absolute(offset.y))
    }

    /// Returns a position relative to the parent on both axis.
    pub fn parent<T: Into<Vec2>>(offset: T) -> Self {
        let offset = offset.into();
        Position::new(Offset::Parent(offset.x), Offset::Parent(offset.y))
    }

    /// Computes the offset required to draw a view.
    ///
    /// When drawing a view with `size` in a container with `available`,
    /// and a parent with the absolute coordinates `parent`, drawing the
    /// child with its top-left corner at the returned coordinates will
    /// position him appropriately.
    pub fn compute_offset<S, A, P>(&self, size: S, available: A, parent: P)
                                   -> Vec2
        where S: Into<Vec2>,
              A: Into<Vec2>,
              P: Into<Vec2>
    {
        let available = available.into();
        let size = size.into();
        let parent = parent.into();

        Vec2::new(self.x.compute_offset(size.x, available.x, parent.x),
                  self.y.compute_offset(size.y, available.y, parent.y))
    }
}

/// Single-dimensional offset policy.
#[derive(PartialEq,Debug,Clone)]
pub enum Offset {
    /// In the center of the screen
    Center,
    /// Place top-left corner at the given absolute coordinates
    Absolute(usize),

    /// Offset from the previous layer's top-left corner.
    ///
    /// If this is the first layer, behaves like `Absolute`.
    Parent(usize), // TODO: use a signed vec for negative offset?
}

impl Offset {
    /// Computes a single-dimension offset requred to draw a view.
    pub fn compute_offset(&self, size: usize, available: usize, parent: usize)
                          -> usize {
        if size > available {
            0
        } else {
            match *self {
                Offset::Center => (available - size) / 2,
                Offset::Absolute(offset) => min(offset, available - size),
                Offset::Parent(offset) => {
                    min(parent + offset, available - size)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use vec::Vec2;
    use super::Position;

    #[test]
    fn test_center() {
        let center = Position::center();
        assert_eq!(Vec2::new(2, 1), center.compute_offset((1,1), (5,3), (0,0)));
        assert_eq!(Vec2::new(2, 0), center.compute_offset((1,3), (5,3), (0,0)));
        assert_eq!(Vec2::new(1, 1), center.compute_offset((3,1), (5,3), (0,0)));
        assert_eq!(Vec2::new(0, 1), center.compute_offset((5,1), (5,3), (0,0)));
        assert_eq!(Vec2::new(0, 0), center.compute_offset((5,3), (5,3), (0,0)));
        assert_eq!(Vec2::new(0, 0), center.compute_offset((5,3), (3,1), (0,0)));
    }
}