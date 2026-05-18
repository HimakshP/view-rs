use crate::math::Vec2;

pub struct Camera {
    pub zoom: f32,
    pub x_offset: f32,
    pub y_offset: f32,
}
impl Camera {
    pub fn new() -> Self {
        Self {
            zoom: 1.0,
            x_offset: 0.0,
            y_offset: 0.0,
        }
    }

    pub fn pan(&mut self, dx: f32, dy: f32) {
        self.x_offset += dx;
        self.y_offset += dy;
    }

    pub fn world_to_screen(&self, point: Vec2) -> Vec2 {
        Vec2 {
            x: point.x * self.zoom + self.x_offset,
            y: point.y * self.zoom + self.y_offset,
        }
    }

    pub fn screen_to_world(&self, point: Vec2) -> Vec2 {
        Vec2 {
            x: (point.x - self.x_offset) / self.zoom,
            y: (point.y - self.y_offset) / self.zoom,
        }
    }

    pub fn zoom(&mut self, factor: f32) {
        self.zoom *= factor;
    }
}
