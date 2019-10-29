use crate::Clases::Objeto::Objeto;
use crate::Aux::Definiciones::{Point3, ColorRGB};
use crate::Aux::Vectores::{Length, Sub, Abs};

pub struct Octaedro{
    pub id : u8,
    pub radio : u8,
    pub traslacion : Point3,
    pub color : ColorRGB
}

impl Objeto for Octaedro {
    fn getId(&self) -> u8 {
        return self.id
    }

    fn getColor(&self) -> ColorRGB {return self.color}

    fn distancia(&self,punto:Point3) -> f32 {
        let mut puntoTrasladado: Point3 = Point3 {x:0.0, y:0.0, z:0.0};
        let mut puntoAbsoluto : Point3 = Point3 {x:0.0, y:0.0, z:0.0};

        puntoTrasladado = Sub(punto, self.traslacion);
        puntoAbsoluto = Abs(puntoTrasladado);

        return (puntoAbsoluto.x + puntoAbsoluto.y + puntoAbsoluto.z - self.radio as f32) * 0.57735027
    }
}