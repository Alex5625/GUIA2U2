use std::io;
use std::fs::File;
use std::path::Path;
use std::fs::OpenOptions;


pub fn texto_numero(campo:String) -> u32 {
    loop {
        println!("Ingrese un número para el/la {}: ",campo);
        let mut numero = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut numero).unwrap();
        let numero: u32 = match numero.trim().parse(){
            Ok(numero) => numero,
            Err(_) => {
                println!("Error, no es un número");
                continue;
            },
        };
        return numero;
    }
}

// ESTO SIRVE PARA AÑADIR NOTAAAAAS
pub fn texto_numero_float() -> f32 {
    loop {
        println!("Ingrese un número: ");
        let mut numero = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut numero).unwrap();
        let numero: f32 = match numero.trim().parse(){
            Ok(numero) => numero,
            Err(_) => {
                println!("Error, no es un número");
                continue;
            },
        };
        if numero <= 7.0{
            println!("");
            return numero;
        } else {
            println!("ingrese un numero entre el 1 y el 7");
        }
    }
}


pub fn ingreso_texto(campo: String) -> String {

    println!("Ingrese {}", campo);
    let mut texto = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut texto).unwrap();
    return texto

}

pub fn si_no() -> bool {

    loop{
        println!("digite 1 para un SI, 0 para un NO\n");
        let desicion = texto_numero("desicion".to_string());
        if desicion == 1 {
            return true;
        }
        if desicion == 0{
            return false;
        } 
    }
}


pub fn numero_max(numero1:u32,numero2:u32) -> bool{

    if numero1 >= numero2{
        return true;    
    } else {
        return false;
    }
}
/*-------------------------------------------------------------------------------------------- */
// MANEJO DE ARCHIVOS!!!! 


// fn main() {
//     //CREAR ARCHIVO
//     let path = Path::new("NOMBRE_ARCHIVO.txt");
//     open_file(path);
//     
//     lo de abajo es para 
//     let file = open_file_to_append(path);
// }



// inicializa archivo nuevo
pub fn create_blank_file(p: &Path) {
    let _file = File::create(p).expect("El archivo no pudo crearse");
    println!("El archivo fue creado");
}
// abre archivo para agregar cosas 
pub fn open_file_to_append(p: &Path) -> File{
    let mut binding = OpenOptions::new();
    let binding = binding.append(true);
    let file = match binding.open(p){
        Err(_why) => panic!("No se puede abrir el archivo"),
        Ok(file) => file,
    };
    return file
}
// abre archivo creado
pub fn open_file(p: &Path) -> (){
    create_blank_file(p);
    let _file = match File::open(&p){
        Err(_why) => panic!("El archivo no se puede abrir..."),
        Ok(file) => file,
    };
}


