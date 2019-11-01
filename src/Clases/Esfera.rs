use crate::Clases::Objeto::Objeto;
use crate::Aux::Definiciones::{Point3, ColorRGB, Materiales};
use crate::Aux::Vectores::{Length, Sub};

#[derive(Clone)]
pub struct Esfera{
    pub id : u8,
    pub tipo : String,
    pub radio : u8,
    pub traslacion : Point3,
    pub color : ColorRGB,
    pub material : Materiales
}

impl Objeto for Esfera {
    fn getId(&self) -> u8 {
        return self.id
    }

    fn getColor(&self) -> ColorRGB {return self.color}

    fn getMaterial(&self) -> Materiales {return self.material}

    fn distancia(&self,punto:Point3) -> f32 {
        //(Length(punto) - self.radio as f32)
        return Length(Sub(punto, self.traslacion)) - self.radio as f32;
    }
}