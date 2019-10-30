use crate::Aux::Definiciones::{Point3, ColorRGB, Materiales};


pub trait Objeto {
    fn getId(&self) -> u8;
    fn distancia(&self, punto:Point3) ->f32;
    fn getColor(&self) -> ColorRGB;
    fn getMaterial(&self) -> Materiales;
}