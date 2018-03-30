/// A bounding box.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct BoundingBox {
    /// The coordinates of the center of the box.
    pub center: (f32, f32),

    /// The height of the box.
    pub height: f32,

    /// The width of the box.
    pub width: f32,
}

impl BoundingBox {
    /// Creates a new BoundingBox, given the upper-right and lower-left
    /// corners.
    pub fn from_corners(ul: (f32, f32), br: (f32, f32)) -> BoundingBox {
        let cx = (ul.0 + br.0) / 2.0;
        let cy = (ul.1 + br.1) / 2.0;
        BoundingBox {
            center: (cx, cy),
            height: (ul.1 - br.1).abs(),
            width: (br.0 - ul.0).abs(),
        }
    }

    /// Checks if this bounding box intersects with another.
    pub fn intersects(self, other: BoundingBox) -> bool {
        self.x_intersects(other) && self.y_intersects(other)
    }

    fn x_intersects(self, other: BoundingBox) -> bool {
        let sl = self.center.0 - self.width / 2.0;
        let sr = self.center.0 + self.width / 2.0;
        let ol = other.center.0 - other.width / 2.0;
        let or = other.center.0 + other.width / 2.0;
        sr >= ol && or >= sl
    }

    fn y_intersects(self, other: BoundingBox) -> bool {
        let sb = self.center.1 - self.height / 2.0;
        let st = self.center.1 + self.height / 2.0;
        let ob = other.center.1 - other.height / 2.0;
        let ot = other.center.1 + other.height / 2.0;
        st >= ob && ot >= sb
    }
}

#[cfg(test)]
mod tests {
    use super::BoundingBox;

    #[test]
    fn doesnt_intersect() {
        let bb1 = BoundingBox::from_corners((1.0, 1.0), (-1.0, -1.0));
        let bb2 = BoundingBox::from_corners((1.75, 1.75), (1.25, 1.25));
        assert!(!bb1.intersects(bb2));
        assert!(!bb2.intersects(bb1));
    }

    #[test]
    fn from_corners() {
        assert_eq!(
            BoundingBox::from_corners((1.0, 1.0), (-1.0, -1.0)),
            BoundingBox {
                center: (0.0, 0.0),
                height: 2.0,
                width: 2.0,
            }
        );
    }

    #[test]
    fn intersects() {
        let bb1 = BoundingBox::from_corners((1.0, 1.0), (-1.0, -1.0));
        let bb2 = BoundingBox::from_corners((0.75, 0.75), (0.25, 0.25));
        assert!(bb1.intersects(bb2));
        assert!(bb2.intersects(bb1));
    }

    #[test]
    fn miniintegration() {
        let bird = BoundingBox {
            center: (0.5, 0.8),
            height: 0.05,
            width: 0.05,
        };

        let (x, y) = (0.5, 0.35);

        const H: f32 = 0.3 / 2.0;
        const W: f32 = 0.1 / 2.0;

        let bb1 = BoundingBox::from_corners((x - W, 1.0), (x + W, 1.0 - y + H));
        let bb2 = BoundingBox::from_corners((x - W, y - H), (x + W, 0.0));
        assert!(bird.intersects(bb1) || bird.intersects(bb2))
    }
}
