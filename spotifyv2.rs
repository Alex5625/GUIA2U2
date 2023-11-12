use std::io::BufReader;
use std::io::{self, BufRead};
use std::path::Path;
use std::u32;
use std::fs::File;

mod utiles;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn loop_rango() -> u32{
    loop{
        println!("tienes para escoger 5 numeros. \nn3: cantidad de dias que ha pasado desde el lanazmiento.\nn4: cantidad de veces que estuvo en el top 10.\nn6: cantidad de veces que estuvo en el top 1.\nn7: numero de reproducciones en su peak.\nn8: cantidad de reproducciones totales.\nn9: ACCEDE A EJERCICIO 6 ");
        let numero = utiles::texto_numero("columna".to_string());
        if numero == 3 || numero == 4 || numero == 6 || numero == 7 || numero == 8 || numero == 9{
            print_tipo_columna(numero);
            return numero;
        }
    }
}

fn print_tipo_columna(numero:u32) -> (){
    match numero{
        3 => println!("El artista y cancion que ha pasado desde el lanzamiento"),
        4 => println!("El artista y cancion que ha estado una mayor cantidad de veces en el top 10"),
        6 => println!("El artista y cancion que ha estado mayor cantidad de veces en el top 1"),
        7 => println!("El artista y cancion que tiene mayor numero de reproducciones en su peak"),
        8 => println!("El artista y cancion que ha tenido mayor cantidad de reproducciones en general"),
        _ => (),
    }

}

fn ej6() -> () {

    let palabra:String = utiles::ingreso_texto("nombre del artista que desea encontrar".to_string());
    println!("EL ARTISTA ES {}",palabra);
    let mut cantidad = 0;

    let mut contador_lineas = 0;
    if let Ok(lines) = read_lines("./dataSpoti.csv") {
        // recorrer lineas
        for line in lines {

            if let Ok(ip) = line {
                let ip_copy = ip.clone();
                let split = ip_copy.split(";");
            //  QUEDA DE LA FORMA ***   ****   **  * *** * 
                let mut contador_columnas:u32 = 0;
                // recorrer el split
                for s in split {
                    // quiero que me lea desde la segunda linea 
                    if contador_columnas == 1 && contador_lineas >= 1{
                        let aaaaa = s.to_lowercase().to_string();
                        println!("ENTRA A LA CONDICION ARTISTA {}",aaaaa);
                        // ESTA CONDICION ES EL PROBLEMA Y LA BASE DE LA PREGUNTA 
                        // si la palabra que yo ingrese anteriormente y la que esta en la posicion 1 de la base de datos
                        // son iguales, aumenta un contador que se llama cantidad
                        if palabra == aaaaa{
                            println!("COINCIDE");
                            cantidad += 1;
                        }
                    }   
                    contador_columnas += 1;
                }
            }
            contador_lineas += 1;
        }
    }
    println!("El artista {} aparece {} cantidad de veces en la base de datos", palabra,cantidad);
    
    if cantidad == 0 {
        println!("No se encontró en la base de datos, porfavor ingrese bien el nombre del artista\n se reiniciará el programa.\n\n");
        todo_proceso();
    } else {
        println!("El artista {} aparece {} cantidad de veces en la base de datos", palabra,cantidad);
    }
}




fn todo_proceso() -> () {
    let mut num_max: u32 = 0;
    let mut arreglo_final: [String;3] = [String::new(),String::new(),String::new()];

    println!("numero de columna a buscar:");
    let columna = loop_rango();
    if columna == 9{
        ej6();
        return;
    }
    let mut contador_lineas = 0;
    if let Ok(lines) = read_lines("./dataSpoti.csv") {

        for line in lines {
            let mut arreglo_provisional: [String;3] = [String::new(),String::new(),String::new()];

            if let Ok(ip) = line {
                let ip_copy = ip.clone();
                let split = ip_copy.split(";");
            //  QUEDA DE LA FORMA ***   ****   **  * *** * 
                let mut contador_columnas:u32 = 0;

                for s in split {
                    if contador_columnas == 1 && contador_lineas >= 1{
                        arreglo_provisional[0] = s.to_string();
                    }

                    if contador_columnas == 2 && contador_lineas >= 1{
                        arreglo_provisional[1] = s.to_string();
                    }

                    if contador_columnas == columna  && contador_lineas >= 1{
                        arreglo_provisional[2] = s.to_string();
                        if let Ok(num) = s.parse::<u32>(){
                            if utiles::numero_max(num,num_max){
                                if num_max == num{
                                    println!("{:?}",arreglo_provisional);
                                }
                                num_max = num;
                                arreglo_final = arreglo_provisional.clone();
                            }
                        }
                    }

                    contador_columnas += 1;

                }
                
            }

            contador_lineas += 1;
        }
    }
    println!("{:?}", arreglo_final);
}



fn main() {
    //CREAR ARCHIVO
    todo_proceso();
}