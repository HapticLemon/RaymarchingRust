use crate::Aux::Definiciones::{Point3, ColorRGB, Materiales};

pub trait Objeto : ObjetoClone{
    fn getId(&self) -> u8;
    fn distancia(&self, punto:Point3) ->f32;
    fn getColor(&self) -> ColorRGB;
    fn getMaterial(&self) -> Materiales;
}

trait ObjetoClone {
    fn clone_box(&self) -> Box<Objeto>;
}

impl<T> ObjetoClone for T
    where
        T: 'static + Objeto + Clone,
{
    fn clone_box(&self) -> Box<Objeto> {
        Box::new(self.clone())
    }
}

impl Clone for Box<Objeto> {
    fn clone(&self) -> Box<Objeto> {
        self.clone_box()
    }
}