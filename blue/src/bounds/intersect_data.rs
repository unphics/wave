pub struct intersect_data {
    b_intersect: bool,
    distance: f32,
}
impl intersect_data {
    pub fn new(b_intersect: bool, distance: f32) -> Self {
        return intersect_data {
            b_intersect: b_intersect,
            distance: distance,
        };
    }
    pub fn is_intersect(&self) -> bool {
        self.b_intersect
    }
    pub fn distance(&self) -> f32 {
        self.distance
    }
}