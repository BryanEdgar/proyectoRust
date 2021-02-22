use substring::Substring;
use tide::prelude::*;
use tide::Request;
// use num_bigint::{BigUint, ToBigInt};
extern crate num_traits;
// use num_traits::{FromPrimitive, One, ToPrimitive, Zero};
// use std::{mem::replace, str::FromStr};

// use crate::big_digit::{self, BigDigit};

// use crate::ParseBigIntError;

#[derive(Debug, Deserialize)]
struct Numero {
    numero: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/").post(numero_letras);
    app.listen("127.0.0.1:8087").await?;
    Ok(())
}

async fn numero_letras(mut req: Request<()>) -> tide::Result {
    let Numero { numero } = req.body_json().await?;

    //Inicializacion de variables
    let tamano = numero.len();
    let mut pos_aux= 0;
    let mut count = 0;
    let mut posicion = 0;
    let mut decimalletras = " ";
    let mut enteroletras = " ";
    let mut numdecimal: i128 = 0;
    let mut entero: i128 = 0;
    let mut letrasdecimal = String::from(" ");
    //inicializo la variable letras aqui se guardara el texto final del numero
    let mut letras = String::from(" ");
    //inicializo el vector donde se guardar en cada item ..1000 en 1000
    let mut vector: Vec<i128> = vec![0; 19];

    //me navego cada caracter del numero para saber la posicion del decimal (.)
    for c in numero.chars() {
        count = count + 1;

        if c.to_string() == "." {
            posicion = count;
        }
    }
    //aqui creo una variable auxiliar para identificar el segmento del entero
    if posicion == 0 {
        pos_aux = tamano;
    } else {
        pos_aux = posicion - 1
    }
    //almacena el numero entero
    enteroletras = numero.substring(0, pos_aux);
    //variables para validar la estructura tanto del entero como el decimal
    let mut flag_entero = "NO";
    let mut flag_decimal = "NO";

    if let Err(e) = enteroletras.parse::<i128>() {
        println!("Failed conversion to i128 para entero: {}", e);
        flag_entero = "SI";
    } else {
        entero = enteroletras.parse::<i128>().unwrap();
    }
    //aqui identifico si es hay decimales
    if posicion > 0 {
        decimalletras = numero.substring(posicion, tamano);
        // agregar validacion cuando falla la conversion
        if let Err(ed) = decimalletras.parse::<i128>() {
            println!("Failed conversion to i128 para decimal: {}", ed);
            flag_decimal = "SI";
        } else {
            numdecimal = decimalletras.parse::<i128>().unwrap();
        }
    }

    let unidades = [
        " ",
        " ",
        " UN",
        "DOS",
        "TRES",
        "CUATRO",
        "CINCO",
        "SEIS",
        "SIETE",
        "OCHO",
        "NUEVE",
        "DIEZ",
        "ONCE",
        "DOCE",
        "TRECE",
        "CATORCE",
        "QUINCE",
        "DIECISEIS",
        "DIECISIETE",
        "DIECIOCHO",
        "DIECINUEVE",
        "VEINTE",
        "VEINTIUNO",
        "VEINTIDOS",
        "VEINTITRES",
        "VEINTICUATRO",
        "VEINTICINCO",
        "VEINTISEIS",
        "VEINTISIETE",
        "VEINTIOCHO",
        "VEINTINUEVE",
    ];

    let decena = [
        " ",
        " ", // se queda con espacio en blanco por la decena
        " ", // se queda con espacio en blanco por la segunda decena que ya se menciona en las unidades
        "TREINTA",
        "CUARENTA",
        "CINCUENTA",
        "SESENTA",
        "SETENTA",
        "OCHENTA",
        "NOVENTA",
    ];

    let centena = [
        " ",
        "CIEN",
        "DOSCIENTOS",
        "TRESCIENTOS",
        "CUATROCIENTOS",
        "QUINIENTOS",
        "SEISCIENTOS",
        "SETECIENTOS",
        "OCHOCIENTOS",
        "NOVECIENTOS",
    ];

    let separa = [
        " NONILLON ",
        " MIL",
        " OCTILLON ",
        " MIL ",
        " SEPTILLON ",
        " MIL ",
        " SEXTILLON ",
        " MIL ",
        " QUINTILLON ",
        " MIL ",
        " CUATRILLON ",
        " MIL ",
        " TRILLON ",
        " MIL ",
        " BILLON ",
        " MIL ",
        " MILLON",
        " MIL ",
        " ",
    ];

    println!("entero:{}", entero);

    if flag_entero == "NO" && flag_decimal == "NO" {
        if entero != 0 {
            for j in (0..19).rev() {
                vector[j] = entero - ((entero / 1000) * 1000);
                entero = (entero - vector[j]) / 1000;
            }

            for x in &vector {
                println!("valor de vector:{}", x);
            }

            let mut i = 0;

            while i <= 18 {
                if vector[i] > 0 {
                    let c = vector[i] / 100;
                    let du = vector[i] - c * 100;
                    let u = du - du / 10 * 10;
                    let d = du / 10;

                    if c > 0 {
                        letras = String::from(letras) + centena[c as usize];
                        if c == 1 && du > 0 {
                            letras = String::from(letras) + "TO";
                        }
                        letras = String::from(letras) + " ";
                    }

                    if du < 30 {
                        letras = String::from(letras) + unidades[du as usize + 1];
                    } else {
                        letras = String::from(letras) + decena[d as usize];
                        if u > 0 {
                            letras = String::from(letras) + " Y " + unidades[u as usize + 1];
                        }
                    }
                    if i == 18 && u == 1 && d != 1 && d != 2 {
                        letras = String::from(letras) + "O";
                    }
                    // println!("index:{}",i);
                    // println!("separa:{}",separa[i]);

                    letras = String::from(letras) + separa[i];

                    // println!("indice:{}",i);

                    if i == 2
                        || i == 4
                        || i == 6
                        || i == 8
                        || i == 10
                        || i == 12
                        || i == 14
                        || i == 16
                    {
                        if vector[i] == 001 {
                            letras = String::from(letras) + " ";
                        } else {
                            letras = String::from(letras.trim_end()) + "ES ";
                        }
                    }
                }
                i = i + 1;
            }
        } else {
            letras = String::from("CERO ");
        }

        //====================LETRAS DE LOS DECIMALES==================================

        if numdecimal > 0 {
            for j in (0..19).rev() {
                vector[j] = numdecimal - ((numdecimal / 1000) * 1000);
                numdecimal = (numdecimal - vector[j]) / 1000;
            }
            let mut i = 0;
            while i <= 18 {
                if vector[i] > 0 {
                    let c = vector[i] / 100;
                    let du = vector[i] - c * 100;
                    let u = du - du / 10 * 10;
                    let d = du / 10;
                    if c > 0 {
                        letrasdecimal = String::from(letrasdecimal) + centena[c as usize];
                        if c == 1 && du > 0 {
                            letrasdecimal = String::from(letrasdecimal) + "TO";
                        }
                        letrasdecimal = String::from(letrasdecimal) + " ";
                    }
                    if du < 30 {
                        letrasdecimal = String::from(letrasdecimal) + unidades[du as usize + 1];
                    } else {
                        letrasdecimal = String::from(letrasdecimal) + decena[d as usize];
                        if u > 0 {
                            letrasdecimal =
                                String::from(letrasdecimal) + " Y " + unidades[u as usize + 1];
                        }
                    }
                    if i == 3 && u == 1 && d != 1 && d != 2 {
                        letrasdecimal = String::from(letrasdecimal) + "O";
                    }
                    letrasdecimal = String::from(letrasdecimal) + separa[i];
                    if i == 2
                        || i == 4
                        || i == 6
                        || i == 8
                        || i == 10
                        || i == 12
                        || i == 14
                        || i == 16
                    {
                        if vector[i] == 001 {
                            letrasdecimal = String::from(letrasdecimal) + " ";
                        } else {
                            letrasdecimal = String::from(letrasdecimal.trim_end()) + "ES ";
                        }
                    }
                }
                i = i + 1;
            }
            let aux: String = letrasdecimal.to_owned();
            let auxcompleto: &str = &aux[..];
            letras = letras + "CON ";
            letras.push_str(auxcompleto);
        }
    } else {
        letras = String::from("REVISE QUE EL NUMERO TENGA EL FORMATO CORRECTO, Ejemplo:123.456 ");
    }

    Ok(format!("RESULTADO: {} ", letras).into())
}
