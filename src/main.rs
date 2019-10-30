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
use crate::Clases::Octaedro::Octaedro;
use crate::Clases::Caja::Caja;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
use serde_json::to_string;

use std::io::BufReader;
use std::fs::File;

use noise::{NoiseFn, Perlin, Worley, Turbulence, HybridMulti};

// Cálculo de la distancia a los elementos de la escena.
// Por el momento y para simplificar, solamente se contempla un
// objeto esfera.
//
fn mapTheWorld(punto : Point3, Escena : &Vec<Box<dyn Objeto>>) -> (f32, u8, ColorRGB, Materiales){
    let mut distancia : f32 = 1000.0;
    let mut distanciaObjeto: f32 = 0.0;
    let mut idObjeto : u8 = 0;
    let mut contador : u8 = 0;
    let mut color:ColorRGB = ColorRGB { R: 0, G: 0, B: 0 };
    let mut material : Materiales = Materiales::NOMAT;


    for item in Escena.iter() {
        distanciaObjeto = item.distancia(punto);
        if distanciaObjeto < distancia {
            distancia = distanciaObjeto;
            idObjeto = contador;
            color = item.getColor();
            material = item.getMaterial();
        }
        contador +=1 ;
    }
    return (distancia, idObjeto, color, material);
}

fn calculateNormal(punto : Point3, Escena : &Vec<Box<dyn Objeto>>, idObjeto :usize) -> Point3{
    let mut gradiente: Point3 = Point3 { x: 1.0, y: 0.0, z: 0.0 };

    gradiente.x = &Escena[idObjeto].distancia(Point3 { x: punto.x + EPSILON, y: punto.y, z: punto.z }) - &Escena[idObjeto].distancia(Point3 { x: punto.x - EPSILON, y: punto.y, z: punto.z });
    gradiente.y = &Escena[idObjeto].distancia(Point3 { x: punto.x , y: punto.y + EPSILON, z: punto.z }) - &Escena[idObjeto].distancia(Point3 { x: punto.x , y: punto.y - EPSILON, z: punto.z });
    gradiente.z = &Escena[idObjeto].distancia(Point3 { x: punto.x , y: punto.y , z: punto.z + EPSILON}) - &Escena[idObjeto].distancia(Point3 { x: punto.x , y: punto.y, z: punto.z - EPSILON});

    MultiplyByScalar(gradiente,-1.0);

    return (Normalize(gradiente))
}

fn ilumina(punto : Point3, diffuseIntensity : f32, normal :Point3, colorObjeto : ColorRGB, materialObjeto : Materiales) -> ColorRGB{
    let mut color:ColorRGB = ColorRGB { R: 0, G: 0, B: 0 };;

    //let perlinValue = perlin.get([42.4, 37.7, 2.8]);

    match materialObjeto{
        Materiales::NOMAT =>{
            color.R = (colorObjeto.R as f32 * diffuseIntensity) as u8;
            color.G = (colorObjeto.G as f32 * diffuseIntensity) as u8;
            color.B = (colorObjeto.B as f32 * diffuseIntensity) as u8;
        }
        Materiales::PERLIN =>{
            let perlin = Perlin::new();
            let perlinValue = perlin.get([punto.x as f64, punto.y as f64, punto.z as f64]).abs();

            color.R = (colorObjeto.R as f32 * perlinValue as f32) as u8;
            color.G = (colorObjeto.G as f32 * perlinValue as f32) as u8;
            color.B = (colorObjeto.B as f32 * perlinValue as f32) as u8;
        }
        Materiales::WORLEY =>{
            let worley = Worley::new();
            let worleyValue = worley.get([punto.x as f64, punto.y as f64, punto.z as f64]).abs();

            color.R = (colorObjeto.R as f32 * worleyValue as f32) as u8;
            color.G = (colorObjeto.G as f32 * worleyValue as f32) as u8;
            color.B = (colorObjeto.B as f32 * worleyValue as f32) as u8;
        }
        Materiales::HYBRID =>{
            let hybrid = HybridMulti::new();
            let hybridValue = hybrid.get([punto.x as f64, punto.y as f64, punto.z as f64]).abs();

            color.R = (colorObjeto.R as f32 * hybridValue as f32) as u8;
            color.G = (colorObjeto.G as f32 * hybridValue as f32) as u8;
            color.B = (colorObjeto.B as f32 * hybridValue as f32) as u8;
        }
        _ => {/* No hace nada, default */}
    }


    return (color);

}

fn raymarching(ro : Point3, rd : Point3, Escena : &Vec<Box<dyn Objeto>>)  -> ColorRGB {

    let mut punto:Point3;
    let mut directionToLight:Point3;
    let mut normal:Point3;

    let mut t: f32 = 0.0;
    let mut diffuseIntensity: f32 = 0.0;
    let mut distancia: f32 = 0.0;

    let mut color:ColorRGB = ColorRGB { R: 0, G: 0, B: 0 };
    let mut colorObjeto:ColorRGB = ColorRGB { R: 0, G: 0, B: 0 };

    let mut idObjeto : u8 = 0;
    let mut materialObjeto : Materiales = Materiales::NOMAT;

    for x in 0..MAXSTEPS{
        punto = Add(ro,MultiplyByScalar(rd,t));
        let (distancia, idObjeto, colorObjeto, materialObjeto) = mapTheWorld(punto, Escena);
        if distancia < MINIMUM_HIT_DISTANCE {
            directionToLight = Normalize(Sub(punto,LIGHT));
            normal = calculateNormal(punto, &Escena, idObjeto as usize);
            diffuseIntensity = Dot(normal, directionToLight).max(0.0);
            color = ilumina(punto, diffuseIntensity, normal, colorObjeto, materialObjeto);
            return color
        }
        t += distancia
    }

    return(color)
}

// Cargamos los objetos leyendo un archivo JSON con la definición de la escena.
//
fn cargaEscena() -> Vec<Box<Objeto>>{

    let file = File::open("/home/john/CLionProjects/RaymarchingRust/src/Aux/Escena.json").unwrap();
    let reader = BufReader::new(file);
    let itemJSONs: Vec<itemJSON> = serde_json::from_reader(reader).unwrap();

    let mut Escena: Vec<Box<Objeto>> = Vec::new();

    // Como hemos usado un default, le pondrá valor de ceros si no se incluye en el json.
    for itemJSON in itemJSONs{
        println!("itemJSON id: {}\tRadio: {}\t Traslacion ({},{},{})",itemJSON.id,itemJSON.radio, itemJSON.traslacion.x, itemJSON.traslacion.y,itemJSON.traslacion.z);

        match itemJSON.tipo.as_ref(){
            "Esfera" => {
                let esfera: Esfera = Esfera{ id: itemJSON.id , tipo : itemJSON.tipo, radio : itemJSON.radio, traslacion : itemJSON.traslacion ,color :itemJSON.color, material :itemJSON.material};
                Escena.push(Box::new(esfera));
            }
            "Octaedro" => {
                let octaedro: Octaedro = Octaedro{ id: itemJSON.id , tipo : itemJSON.tipo, radio : itemJSON.radio, traslacion : itemJSON.traslacion ,color :itemJSON.color, material :itemJSON.material};
                Escena.push(Box::new(octaedro));
            }
            "Caja" => {
                let caja: Caja = Caja{ id: itemJSON.id , tipo : itemJSON.tipo, posicion : itemJSON.posicion, dimensiones : itemJSON.dimensiones, traslacion : itemJSON.traslacion ,color :itemJSON.color, material :itemJSON.material};
                Escena.push(Box::new(caja));
            }
            _ => {/* No hace nada, default */}
        }
    }

    return Escena;
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
    let mut imgbuf = image::ImageBuffer::new(WIDTH as u32, HEIGHT as u32);

    // Declaración del vector venérico.
    // Iniciamos la lista de objetos.
    //
    let mut Escena: Vec<Box<Objeto>> = Vec::new();
    Escena = cargaEscena();

    // Proceso de la imagen
    //
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        NDC_X = (x as f32 + CORRECCION) / WIDTH as f32;
        NDC_Y = (y as f32 + CORRECCION) / HEIGHT as f32;

        PixelScreen_X = 2.0 * NDC_X - 1.0;
        PixelScreen_Y = 2.0 * NDC_Y - 1.0;

        PixelCamera_X = PixelScreen_X * ImageAspectRatio;
        PixelCamera_Y = PixelScreen_Y;

        ro = Add(Add(Add(EYE,MultiplyByScalar(FORWARD,FL)),MultiplyByScalar(RIGHT, PixelCamera_X)),MultiplyByScalar(UP, PixelCamera_Y));
        rd = Normalize(Sub(ro, EYE));

        // Pasa referencias & o el compilador se quejará de que quieres pasar algo movido.
        color = raymarching(ro,rd, &Escena);
        *pixel = image::Rgb([color.R,color.G,color.B]);
    }

    // Guardo la imagen
    imgbuf.save(fileOut).unwrap();

}
