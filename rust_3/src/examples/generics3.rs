impl Point<f32> {
    fn dist(&self, p: Point<f32>) -> f32 {
        ((&self.x - &p.x).powf(2f32) + (&self.y - &p.y).powf(2f32)).sqrt()
    }
}
