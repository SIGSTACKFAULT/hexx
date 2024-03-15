use std::ops::Neg;

use crate::{DiagonalDirection, Hex};

use super::GridEdge;

/// Hexagonal grid orientated vertex representation.
#[derive(Debug, Clone, Copy)]
pub struct GridVertex {
    /// The coordinate of the edge
    pub origin: Hex,
    /// The direction the vertex points towards
    pub direction: DiagonalDirection,
}

impl PartialEq for GridVertex {
    fn eq(&self, other: &Self) -> bool {
        let [cw, ccw] = [
            Self {
                origin: self.origin + self.direction.direction_cw(),
                direction: self.direction.rotate_ccw(2),
            },
            Self {
                origin: self.origin + self.direction.direction_ccw(),
                direction: self.direction.rotate_cw(2),
            },
        ];
        // identical
        (self.origin == other.origin && self.direction == other.direction)
            || (cw.origin == other.origin && cw.direction == other.direction)
            || (ccw.origin == other.origin && ccw.direction == other.direction)
    }
}

impl GridVertex {
    #[inline]
    #[must_use]
    /// Returns the three connected coordinates in clockwise order
    pub const fn coordinates(&self) -> [Hex; 3] {
        [
            self.origin,
            self.origin.add_dir(self.direction.direction_ccw()),
            self.origin.add_dir(self.direction.direction_cw()),
        ]
    }
    #[inline]
    #[must_use]
    /// Returns the two destination coordinates in clockwise order
    pub const fn destinations(&self) -> [Hex; 2] {
        [
            self.origin.add_dir(self.direction.direction_ccw()),
            self.origin.add_dir(self.direction.direction_cw()),
        ]
    }

    #[inline]
    #[must_use]
    /// Return the two adjacent edges sharing the same coordinate origin.
    /// The edges are returned in clockwise order
    pub const fn side_edges(&self) -> [GridEdge; 2] {
        [
            GridEdge {
                origin: self.origin,
                direction: self.direction.direction_ccw(),
            },
            GridEdge {
                origin: self.origin,
                direction: self.direction.direction_cw(),
            },
        ]
    }

    #[inline]
    #[must_use]
    /// Inverts the vertex, now facing the opposite direction
    pub const fn const_neg(self) -> Self {
        Self {
            direction: self.direction.const_neg(),
            ..self
        }
    }

    #[inline]
    #[must_use]
    /// Returns the next vertex in clockwise order
    pub const fn clockwise(self) -> Self {
        Self {
            direction: self.direction.clockwise(),
            ..self
        }
    }

    #[inline]
    #[must_use]
    /// Returns the next vertex in counter clockwise order
    pub const fn counter_clockwise(self) -> Self {
        Self {
            direction: self.direction.counter_clockwise(),
            ..self
        }
    }

    #[inline]
    #[must_use]
    /// Rotates `self` clockwise by `offset` amount.
    pub const fn rotate_cw(self, offset: usize) -> Self {
        Self {
            direction: self.direction.rotate_cw(offset),
            ..self
        }
    }
    #[inline]
    #[must_use]
    /// Rotates `self` counter clockwise by `offset` amount.
    pub const fn rotate_ccw(self, offset: usize) -> Self {
        Self {
            direction: self.direction.rotate_ccw(offset),
            ..self
        }
    }
}

impl Hex {
    /// Returns all vertices of the given coordinate
    #[inline]
    #[must_use]
    pub fn all_vertices(self) -> [GridVertex; 6] {
        DiagonalDirection::ALL_DIRECTIONS.map(|direction| GridVertex {
            origin: self,
            direction,
        })
    }
}

impl Neg for GridVertex {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        self.const_neg()
    }
}
