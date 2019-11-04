use crate::Aux::Definiciones::{Point3, ColorRGB};

// Coordenadas del punto en valores absolutos.
//
pub fn Abs(a: Point3) -> Point3 {
    return Point3{x : a.x.abs(), y : a.y.abs(), z : a.z.abs() }
}

pub fn Add(a: Point3, b : Point3) -> Point3{
    let mut resultado:Point3 = Point3 { x: 0.0, y: 0.0, z: 0.0 };

    resultado.x  = a.x + b.x;
    resultado.y  = a.y + b.y;
    resultado.z  = a.z + b.z;

    return(resultado);
}

pub fn Sub(a: Point3, b : Point3) -> Point3{
    let mut resultado:Point3 = Point3 { x: 0.0, y: 0.0, z: 0.0 };

    resultado.x  = a.x - b.x;
    resultado.y  = a.y - b.y;
    resultado.z  = a.z - b.z;

    return(resultado);
}

pub fn MultiplyByScalar(a: Point3, scalar : f32) -> Point3{
    let mut resultado:Point3 = Point3 { x: 0.0, y: 0.0, z: 0.0 };

    resultado.x = a.x * scalar;
    resultado.y = a.y * scalar;
    resultado.z = a.z * scalar;

    return(resultado);
}

pub fn MultiplyColorByScalar(a: ColorRGB, scalar : f32) -> ColorRGB{
    let mut resultado:ColorRGB = ColorRGB { R: 0, G: 0, B: 0 };

    resultado.R = (a.R as f32 * scalar) as u8;
    resultado.G = (a.G as f32 * scalar) as u8;
    resultado.B = (a.B as f32 * scalar) as u8;

    return(resultado);
}

// Interpolación linear de dos colores. Pásalo a Vectores.
//
pub fn mixColor(x : ColorRGB, y : ColorRGB, a : f32) -> ColorRGB{
    let mut resultado : ColorRGB = ColorRGB{R:0, G:0, B:0};

    resultado.R = (x.R as f32 * (1.0 - a) + y.R as f32 * a) as u8;
    resultado.G = (x.G as f32 * (1.0 - a) + y.G as f32 * a) as u8;
    resultado.B = (x.B as f32 * (1.0 - a) + y.B as f32 * a) as u8;

    return resultado;
}

pub fn Dot(a: Point3, b: Point3) -> f32{
    return a.x * b.x + a.y * b.y + a.z * b.z;
}

pub fn Length(a: Point3) -> f32{
    return Dot(a,a).sqrt();
}

pub fn Normalize(a: Point3) -> Point3{
    return (MultiplyByScalar(a, 1.0 / Length(a)));
}