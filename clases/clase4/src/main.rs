// las reglas de ownership
// 1. Cada valor en Rust tiene una variable que es su dueña (owner)
// 2. Solo puede haber un dueño a la vez.
// 3. Cuando un dueño sale del scope el valor es eliminado (dropped)

// Las reglas del Borrowing
// 1. Pueden tener multiples referencias inmutables al mismo tiempo
// 2. Solo puedes tener una referencia mutable a la vez
// 3. No podemos tener refencias mutables e inmutables al mismo tiempo

fn main() {
    // char, bool, i32..., f64..., las tuplas si todos los componentes a su vez son de los tipos mencionados

    let s1 = String::from("hola");
    let s2 = s1;

    //let s1 = 5;
    //let s2 = s1;
    
    println!("{s1}");
}

fn recivo_referencia_inmutable(texto:&String) {
    println!("adentro de la funcion: {texto}");
}

fn recivo_referencia_mutable(texto:&mut String) {
    println!("adentro de la funcion: {texto}");
}

fn primera_palabra(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}