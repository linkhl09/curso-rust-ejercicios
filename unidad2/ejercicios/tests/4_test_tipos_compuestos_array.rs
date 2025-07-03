#[cfg(test)]
mod array_tests {

    // Los arrays en Rust son una colección de objetos del mismo tipo almacenados en memoria contigua.
    // Tienen una longitud fija, que se conoce en tiempo de compilación.

    #[test]
    fn test_crear_y_acceder_arrays() {
        // Objetivo: Aprender a declarar un array y acceder a sus elementos.

        // 1. Declara un array `meses` que contenga los nombres de los primeros 3 meses del año.
        // La firma de tipo de un array es `[tipo; longitud]`.
        let meses: [&str; 3] = ["Enero", "Febrero", "Marzo"];

        // 2. Accede al primer elemento del array (índice 0) y verifica su valor.
        assert_eq!(meses[0], "Enero");

        // 3. Accede al tercer elemento y verifica que sea "Marzo".
        assert_eq!(meses[2], "Marzo");

        // 4. Obtén la longitud del array usando el método `.len()` y verifícala.
        assert_eq!(meses.len(), 3);
    }

    #[test]
    fn test_inicializacion_de_arrays() {
        // Objetivo: Aprender a inicializar un array con un valor por defecto.

        // 1. Crea un array `buffer` de 1024 bytes (u8), todos inicializados a 0.
        // La sintaxis `[valor; longitud]` es una forma conveniente de hacerlo.
        let buffer: [u8; 1024] = [0;1024];

        // 2. Verifica que el primer y el último elemento sean 0.
        assert_eq!(buffer[0], 0);
        assert_eq!(buffer[buffer.len() - 1 ], 0);


        // 3. Verifica la longitud total del buffer.
        assert_eq!(buffer.len(), 1024);

        // 4. Crea un array de 5 booleanos, todos inicializados a `true`.
        let boolean_array: [bool;5] = [true;5];
        
        for bt in boolean_array {
            assert_eq!(bt, true);
        }
    }

    #[test]
    fn test_arrays_multidimensionales() {
        // Objetivo: Entender cómo se pueden crear arrays de arrays (matrices).

        // 1. Declara una matriz de 2x3 (2 filas, 3 columnas).
        let matriz: [[i32; 3]; 2] = [
            [1, 2, 3],
            [4, 5, 6],
        ];

        // 2. Accede al elemento en la primera fila, segunda columna (debería ser 2).
        let elemento_0_1 = matriz[0][1];
        assert_eq!(elemento_0_1, 2);

        // 3. Accede al elemento en la segunda fila, tercera columna (debería ser 6).
        let elemento_1_2 = matriz[1][2];
        assert_eq!(elemento_1_2, 6);
    }

}
