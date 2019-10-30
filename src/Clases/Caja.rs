use crate::Clases::Objeto::Objeto;
use crate::Aux::Definiciones::{Point3, ColorRGB, Materiales};
use crate::Aux::Vectores::{Length, Sub, Abs};
use std::cmp;

pub struct Caja{
    pub id : u8,
    pub tipo : String,
    pub posicion : Point3,
    pub dimensiones : Point3,
    pub traslacion : Point3,
    pub color : ColorRGB,
    pub material : Materiales
}

impl Objeto for Caja {
    fn getId(&self) -> u8 {
        return self.id
    }

    fn getColor(&self) -> ColorRGB {return self.color}

    fn getMaterial(&self) -> Materiales {return self.material}

    fn distancia(&self,punto:Point3) -> f32 {
        let mut puntoTrasladado: Point3 = Point3 {x:0.0, y:0.0, z:0.0};
        let mut puntoAbsoluto : Point3 = Point3 {x:0.0, y:0.0, z:0.0};

        puntoTrasladado = Sub(punto, self.traslacion);
        puntoAbsoluto = Abs(puntoTrasladado);

        puntoAbsoluto.x = (puntoTrasladado.x - self.posicion.x).abs() - self.dimensiones.x;
        puntoAbsoluto.y = (puntoTrasladado.y - self.posicion.y).abs() - self.dimensiones.y;
        puntoAbsoluto.z = (puntoTrasladado.z - self.posicion.z).abs() - self.dimensiones.z;

        return puntoAbsoluto.z.max(puntoAbsoluto.x.max(puntoAbsoluto.y));

    }
}