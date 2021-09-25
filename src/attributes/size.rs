use std::fmt::Display;

pub struct Size {
    width: f64,
    height: f64,
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{},{}\"", self.width, self.height)
    }
}

impl From<f64> for Size {
    fn from(f: f64) -> Self {
        Size {
            width: f,
            height: f,
        }
    }
}

impl From<(f64, f64)> for Size {
    fn from(fs: (f64, f64)) -> Self {
        Size {
            width: fs.0,
            height: fs.1,
        }
    }
}
