use parsel::syn::{Path, PathSegment};

pub trait PathExt {
    fn with_segment<T: Into<PathSegment>>(self, segment: T) -> Self;
}

impl PathExt for Path {
    fn with_segment<T: Into<PathSegment>>(mut self, segment: T) -> Self {
        self.segments.push(segment.into());
        self
    }
}
