#[derive(Clone, Copy)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct ColorRGB {
    pub R: u8,
    pub G: u8,
    pub B: u8,
}

pub const WIDTH: u32 = 640;
pub const HEIGHT: u32 = 480;

pub const MAXSTEPS: u32 = 32;

pub const CORRECCION: f32 = 0.5;
pub const EPSILON: f32 = 0.01;
pub const INFINITE: f32 = 100000.0;

pub const FL: f32 = 0.5;
pub const MINIMUM_HIT_DISTANCE: f32 = 0.05;

pub const EYE: Point3 = Point3 { x: 0.0, y: 0.0, z: -15.0 };
pub const RIGHT: Point3 = Point3 { x: 1.0, y: 0.0, z: 0.0 };
pub const UP: Point3 = Point3 { x: 0.0, y: 1.0, z: 0.0 };
pub const FORWARD: Point3 = Point3 { x: 0.0, y: 0.0, z: 1.0 };

pub const LIGHT: Point3 = Point3 { x: 0.0, y: 30.0, z: 0.0 };

pub const COLOR: ColorRGB = ColorRGB { R: 200, G: 230, B: 0 };
pub const ImageAspectRatio: f32 = WIDTH as f32 / HEIGHT as f32;


