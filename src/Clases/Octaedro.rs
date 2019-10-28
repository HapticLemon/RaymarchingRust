use crate::Clases::Objeto::Objeto;
use crate::Aux::Definiciones::Point3;
use crate::Aux::Vectores::Length;

pub struct Octaedro{
    pub id : u8,
    pub radio : u8
}

impl Objeto for Octaedro {
    fn getId(&self) -> u8 {
        return self.id
    }

    fn distancia(&self,punto:Point3) -> f32 {

        return (punto.x.abs() + punto.y.abs() + punto.z.abs() - self.radio as f32) * 0.57735027
    }
}