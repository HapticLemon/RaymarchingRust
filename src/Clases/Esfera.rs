use crate::Clases::Objeto::Objeto;
use crate::Aux::Definiciones::Point3;
use crate::Aux::Vectores::Length;

pub struct Esfera{
    pub id : u8,
    pub radio : u8
}

impl Objeto for Esfera {
    fn getId(&self) -> u8 {
        return self.id
    }

    fn distancia(&self,punto:Point3) -> f32 {
        (Length(punto) - self.radio as f32)
    }
}