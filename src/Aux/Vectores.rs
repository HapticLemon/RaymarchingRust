use crate::Aux::Definiciones::Point3;

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

pub fn Dot(a: Point3, b: Point3) -> f32{
    return a.x * b.x + a.y * b.y + a.z * b.z;
}

pub fn Length(a: Point3) -> f32{
    return Dot(a,a).sqrt();
}

pub fn Normalize(a: Point3) -> Point3{
    return (MultiplyByScalar(a, 1.0 / Length(a)));
}