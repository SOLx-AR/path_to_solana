use std::env;

fn main() {
    let texto = "Hola, ";
    let args: Vec<String> = env::args().collect();
    let nombre: String = args[1].clone();
    let age: u8 = args[2].parse().unwrap();
    let retorno =saludar(texto, &nombre);
    let retorno = arbitro(age);
    println!("{retorno}");
let color:(u8, u8, u8)  = (255,255,225);

}

fn saludar (texto: &str, nombre: &str) {
     let saludo = texto.to_string() + &nombre;
    println!("{saludo}");
}

fn arbitro(age: u8) -> String {
    if age >= 'a' {
        "sos mayor de edad".to_string()
    } else {
        "gracias, vuelva pronto".to_string()
    }
}

//challenge uno: dbg!() persiste cargo build --release?

//u8
//bool
//char

// string
// &srt
// si necesito crear un texto nuevo, desconocido al momento de compilacion
// es un string
// si necesito retornar de un modo nuevo algo relacionado a otro texto es un &str
// si es conocido en el momento de compilacion &srt
// fn
// parametros &str y como salida String

// para convertir un &str a String
// texto.to_string()


// tuplas
//tupla vacia
// ()


// (5)
// (true,6)
// (falso, 6, "no")

//(r,g,b)
