use gl::types::GLenum;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolygonMode {
    /// Just show the points.
    Point = gl::POINT as isize,
    /// Just show the lines.
    Line = gl::LINE as isize,
    /// Fill in the polygons.
    Fill = gl::FILL as isize,
}

impl PolygonMode {
    pub fn turn(mode: Self) -> Self {
        match mode {
            PolygonMode::Point => Self::Fill,
            PolygonMode::Line => Self::Point,
            PolygonMode::Fill => Self::Line,
        }
    }
}

/// Sets the font and back polygon mode to the mode given.
pub fn polygon_mode(mode: PolygonMode) {
    unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, mode as GLenum) };
}
