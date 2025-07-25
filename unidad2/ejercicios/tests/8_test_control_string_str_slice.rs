#[cfg(test)]
mod string_slice_tests {

    // En Rust, hay dos tipos principales de strings: `String` y `&str`.
    // `&str` (string slice): Es una referencia a una secuencia de bytes. Es inmutable y prestado.
    // `String`: Es un tipo poseído (owned), modificable y que se almacena en el heap.

    #[test]
    fn test_crear_string_y_str() {
        // Objetivo: Entender las formas básicas de crear `String` y `&str`.

        // 1. `&str` se crea a partir de literales de cadena.
        let mi_slice: &str = "Hola, mundo!";
        assert_eq!(mi_slice, "Hola, mundo!");

        // 2. `String` se puede crear a partir de un `&str` usando `.to_string()`.


        // 3. También se puede usar `String::from()`.


        // 4. Un `&str` puede ser una vista de una parte de un `String`.

    }

    #[test]
    fn test_modificar_un_string() {
        // Objetivo: Aprender a modificar un `String` mutable.

        // 1. Para modificar un `String`, debe ser mutable (`mut`).


        // 2. Usa `push_str()` para añadir un slice de string (`&str`).


        // 3. Usa `push()` para añadir un solo carácter (`char`).

    }

    #[test]
    fn test_concatenacion_con_operador_mas() {
        // Objetivo: Entender cómo funciona la concatenación con el operador `+`.

        let s1 = String::from("Hola, ");
        let s2 = String::from("mundo!");

        // El operador `+` toma posesión (ownership) del primer operando (`s1`).
        // `s1` es movido y ya no se puede usar después de esta línea.
        // El segundo operando (`s2`) es una referencia (`&String`), por lo que no se mueve.


        // Descomenta la siguiente línea y verás un error de compilación porque `s1` fue movido.
        // println!("{}", s1);
    }

    #[test]
    fn test_concatenacion_con_macro_format() {
        // Objetivo: Usar la macro `format!` para concatenar strings sin tomar posesión.

        let s1 = String::from("Tic");
        let s2 = String::from("Tac");
        let s3 = String::from("Toe");

        // 1. Usa la macro `format!` para combinar `s1`, `s2` y `s3`.
        // `format!` no toma posesión de sus argumentos, por lo que `s1`, `s2` y `s3` siguen siendo válidos.


        // 2. Verifica el resultado.


        // 3. Verifica que `s1`, `s2` y `s3` todavía se pueden usar.

    }

}
