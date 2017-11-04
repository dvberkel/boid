pub struct Boid {
    pub x: f64,
    pub y: f64,
    pub heading: f64,
    speed: f64,
}

impl Boid {
    pub fn new(x: f64, y: f64) -> Boid {
        Boid { x, y, heading: 0f64, speed: 3f64 }
    }

    pub fn update(&mut self) {
        let dx = self.speed * self.heading.cos();
        let dy = self.speed * self.heading.sin();

        self.x += dx;
        self.y += dy;
    }

    pub fn clip(&mut self, width: u32, height: u32) {
        let width = width as f64;
        let height = height as f64;

        let factor = (self.x / width).floor();
        self.x -= factor * width;

        let factor = (self.y / height).floor();
        self.x -= factor * height;
    }
}