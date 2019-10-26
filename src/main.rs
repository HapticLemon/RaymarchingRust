mod Aux;
mod Clases;

use crate::Aux::Definiciones::*;
use crate::Aux::Vectores::*;

// Hay que incuir el nombre con la versión en el toml
extern crate image;
use image::*;
use rgb::{RGB, RGB8};

use std::cmp;
use crate::Clases::Esfera::Esfera;
use crate::Clases::Objeto::Objeto;

// De momento defino un solo objeto. El siguiente paso será el de montar una lista
// genérica.
//
static esfera_0 : Esfera = Esfera{id : 1, radio : 5};

// Cálculo de la distancia a los elementos de la escena.
// Por el momento y para simplificar, solamente se contempla un
// objeto esfera.
//
fn mapTheWorld(punto : Point3) -> f32{
    return (esfera_0.distancia(punto));
}

// Cálculo de color en cada punto de la escena.
//

fn calculateNormal(punto : Point3) -> Point3{
    let mut gradiente: Point3 = Point3 { x: 1.0, y: 0.0, z: 0.0 };

    gradiente.x = mapTheWorld(Point3 { x: punto.x + EPSILON, y: punto.y, z: punto.z }) - mapTheWorld(Point3 { x: punto.x - EPSILON, y: punto.y, z: punto.z });
    gradiente.y = mapTheWorld(Point3 { x: punto.x , y: punto.y + EPSILON, z: punto.z }) - mapTheWorld(Point3 { x: punto.x, y: punto.y - EPSILON, z: punto.z });
    gradiente.z = mapTheWorld(Point3 { x: punto.x , y: punto.y, z: punto.z + EPSILON}) - mapTheWorld(Point3 { x: punto.x , y: punto.y, z: punto.z - EPSILON});

    Aux::Vectores::MultiplyByScalar(gradiente,-1.0);

    return (Aux::Vectores::Normalize(gradiente))
}

fn ilumina(punto : Point3, diffuseIntensity : f32, normal :Point3) -> ColorRGB{
    let mut color:ColorRGB = ColorRGB { R: 0, G: 0, B: 0 };;

    color.R = (COLOR.R as f32 * diffuseIntensity) as u8;
    color.G = (COLOR.G as f32 * diffuseIntensity) as u8;
    color.B = (COLOR.B as f32 * diffuseIntensity) as u8;

    return (color);

}

fn raymarching(ro : Point3, rd : Point3)  -> ColorRGB {

    let mut punto:Point3;
    let mut directionToLight:Point3;
    let mut normal:Point3;

    let mut t: f32 = 0.0;
    let mut diffuseIntensity: f32 = 0.0;
    let mut distancia: f32 = 0.0;

    let mut color:ColorRGB = ColorRGB { R: 0, G: 0, B: 0 };


    for x in 0..MAXSTEPS{
        punto = Aux::Vectores::Add(ro,MultiplyByScalar(rd,t));
        distancia = mapTheWorld(punto);
        if distancia < MINIMUM_HIT_DISTANCE {
            directionToLight = Aux::Vectores::Normalize(Sub(punto,LIGHT));
            normal = calculateNormal(punto);
            diffuseIntensity = Aux::Vectores::Dot(normal, directionToLight).max(0.0);
            color = ilumina(punto, diffuseIntensity, normal);
            return color
        }
        t += distancia
    }

    return(color)
}

// Bucle principal. Prepara los elementos usados en el cálculo del color.
//
fn main() {
    let mut NDC_X:f32;
    let mut NDC_Y:f32;

    let mut PixelScreen_X:f32;
    let mut PixelScreen_Y:f32;

    let mut PixelCamera_X:f32 = 0.0;
    let mut PixelCamera_Y:f32 = 0.0;

    let mut ro:Point3;
    let mut rd:Point3;

    let mut color:ColorRGB;

    // Habrá que mirar el tema de los parámetros de entrada.
    let fileIn:&str;
    let fileOut = "Rustmarching.jpg";

    // Defino el archivo de la imagen.
    //let mut imgbuf: image::ImageBuffer<image::Rgba<u8>, _> = image::ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
    let mut imgbuf = image::ImageBuffer::new(WIDTH as u32, HEIGHT as u32);

    // Proceso de la imagen
    //
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        NDC_X = (x as f32 + CORRECCION) / WIDTH as f32;
        NDC_Y = (y as f32 + CORRECCION) / HEIGHT as f32;

        PixelScreen_X = 2.0 * NDC_X - 1.0;
        PixelScreen_Y = 2.0 * NDC_Y - 1.0;

        PixelCamera_X = PixelScreen_X * ImageAspectRatio;
        PixelCamera_Y = PixelScreen_Y;

        ro = Aux::Vectores::Add(Aux::Vectores::Add(Aux::Vectores::Add(EYE,Aux::Vectores::MultiplyByScalar(FORWARD,FL)),Aux::Vectores::MultiplyByScalar(RIGHT, PixelCamera_X)),Aux::Vectores::MultiplyByScalar(UP, PixelCamera_Y));
        rd = Normalize(Sub(ro, EYE));

        color = raymarching(ro,rd);
        *pixel = image::Rgb([color.R,color.G,color.B]);
    }

    // Guardo la imagen
    imgbuf.save(fileOut).unwrap();

}
