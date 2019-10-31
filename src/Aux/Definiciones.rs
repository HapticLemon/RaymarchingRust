use std::f32::consts::PI;

#[derive(Clone, Copy, Deserialize, Default)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy, Deserialize)]
pub struct ColorRGB {
    pub R: u8,
    pub G: u8,
    pub B: u8,
}

#[derive(Clone, Copy, Deserialize)]
pub enum Materiales {
    NOMAT,
    PERLIN,
    WORLEY,
    HYBRID,
    OPENSIMPLEX
}

// Estructura usada para leer los elementos del JSON
//
#[derive(Deserialize)]
pub struct itemJSON {
    pub id: u8,
    pub tipo : String,
    #[serde(default)]
    pub radio: u8,
    // Nos permite hacer el campo opcional.
    #[serde(default)]
    pub traslacion : Point3,
    #[serde(default)]
    pub posicion : Point3,
    #[serde(default)]
    pub dimensiones : Point3,
    pub color : ColorRGB,
    pub material : Materiales
}

pub const WIDTH: u32 = 1024;
pub const HEIGHT: u32 = 683;

pub const MAXSTEPS: u32 = 32;

pub const CORRECCION: f32 = 0.5;
pub const EPSILON: f32 = 0.01;
pub const INFINITE: f32 = 100000.0;

pub const FL: f32 = 0.5;
pub const MINIMUM_HIT_DISTANCE: f32 = 0.01;

pub const EYE: Point3 = Point3 { x: 0.0, y: 0.0, z: 0.0 };
pub const RIGHT: Point3 = Point3 { x: 1.0, y: 0.0, z: 0.0 };
pub const UP: Point3 = Point3 { x: 0.0, y: 1.0, z: 0.0 };
pub const FORWARD: Point3 = Point3 { x: 0.0, y: 0.0, z: 1.0 };

pub const LIGHT: Point3 = Point3 { x: 200.0, y: 200.0, z: 180.0 };

pub const COLOR: ColorRGB = ColorRGB { R: 200, G: 230, B: 0 };
pub const ImageAspectRatio: f32 = WIDTH as f32 / HEIGHT as f32;

// Ángulo para el FOV. Actúa como una especie de zoom.
pub const ALPHA :f32 = 55.0;






