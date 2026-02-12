pub struct Point<U, V> {
    pub x: U,
    pub y: V,
}

impl<U, V> Point<U, V> {
    pub fn x(&self) -> &U {
        &self.x
    }

    pub fn y(&self) -> &V {
        &self.y
    }
}
