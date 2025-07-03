#[cfg(test)]
mod conversion_tests {
    #[test]
    fn test_widening_conversion_as() {
        // Objetivo: Practicar la conversión "widening" (ampliación) donde no se pierde información.

        // 1. Declara una variable `small_number` de tipo u8 con el valor 100.
        let small_number: u8 = 100;

        // 2. Convierte `small_number` a un tipo u32 usando la palabra clave `as`.
        // Esta conversión es segura porque u32 puede contener todos los valores de u8.
        let small_number_u32: u32 = small_number as u32;

        // 3. Verifica que la conversión fue exitosa.
        assert_eq!(small_number_u32, 100);

        // 4. Declara un i16 y conviértelo a i64.
        let small_int: i16 = 25;
        let small_int_i64: i64 = small_int as i64;
        assert_eq!(small_int_i64, 25);
    }

    #[test]
    fn test_narrowing_conversion_as() {
        // Objetivo: Entender los riesgos de la conversión "narrowing" (reducción).

        // 1. Declara una variable `big_number` de tipo i32 con el valor 300.
        let mut big_number: i32 = 300;

        // 2. Convierte `big_number` a un tipo i8 usando `as`.
        // ¡Atención! i8 solo puede contener valores de -128 a 127.
        // La conversión truncará los bits. 300 en binario es 0001 0010 1100.
        // Al truncar a 8 bits, nos quedamos con 0010 1100, que es 44 en decimal.
        let mut big_number_i8: i8 = big_number as i8;

        // 3. Verifica el resultado del truncamiento.
        assert_eq!(big_number_i8, 44);

        // 4. ¿Qué pasa con un número negativo?
        // 1000 en binario es 0011 1110 1000. Truncado a 8 bits es 1110 1000.
        // En complemento a dos, 1110 1000 representa -24.
        big_number = 1_000;
        big_number_i8 = big_number as i8;
        assert_eq!(big_number_i8, -24);
    }

    #[test]
    fn test_float_to_integer_conversion() {
        // Objetivo: Observar cómo se maneja la conversión de flotante a entero.

        // 1. Declara una variable `pi` de tipo f64.
        let mut pi: f64 = std::f64::consts::PI;

        // 2. Convierte `pi` a un i32 usando `as`.
        // La parte decimal se trunca (se elimina, no se redondea).
        let int_pi: i32 = pi as i32;

        // 3. Verifica que la parte decimal fue eliminada.
        assert_eq!(int_pi, 3);

        // 4. Prueba con un número negativo.
        pi = -pi;
        let neg_int_pi: i32 = pi as i32;
        
        assert_eq!(neg_int_pi, -3);
    }

    #[test]
    fn test_integer_to_float_conversion() {
        // Objetivo: Practicar la conversión de entero a flotante.

        // 1. Declara un entero `my_integer`.
        let my_int = 64;

        // 2. Conviértelo a f64. Esta conversión generalmente es segura y no pierde precisión
        // para enteros de 32 bits.
        let int_to_float: f64 = my_int as f64;
        
        // 3. Verifica la conversión.
        assert_eq!(int_to_float, 64.0);
        

        // 4. Para enteros muy grandes (i64), la conversión a f32 puede perder precisión.
        let big_integer: i64 = 9_007_199_254_740_991; // Límite de precisión para f64
        let float_from_big: f64 = big_integer as f64;

        let bigger_integer: i64 = 9_007_199_254_740_993; // Un número mayor
        let imprecise_float: f64 = bigger_integer as f64;

        assert_eq!(float_from_big, 9007199254740991.0);
        // Debido a la falta de precisión, el resultado de la conversión no es exacto.
        // El entero se redondea al valor f64 más cercano representable.
        assert_eq!(imprecise_float, 9007199254740992.0, "La conversión debería redondear al flotante más cercano.");
    }

    #[test]
    fn test_from_trait_conversion() {
        // Objetivo: Usar el trait `From` para conversiones seguras y explícitas.
        // `From` es una forma más idiomática y segura de realizar conversiones "widening".

        // 1. Declara un u8.
        let one_u8: u8 = 12;

        // 2. Usa `u16::from()` para convertir el u8 a u16.
        let one_u16: u16 = u16::from(one_u8); 

        // 3. Haz lo mismo para convertir un i8 a i32.
        let one_i8: i8 = 22;
        let one_i32: i32 = i32::from(one_i8);
        
        // 4. El trait `Into` es la contraparte de `From`. Si `T::from(U)` está implementado,
        // entonces `U` tiene una implementación de `into()` para `T`.
        let one_u16_but_different: u16 = one_u16.into();
        let one_i32_but_different: i32 = one_i8.into();
        
        assert_eq!(one_u16, 12);
        assert_eq!(one_i32, 22);
        assert_eq!(one_u16_but_different, 12);
        assert_eq!(one_i32_but_different, 22);
    }
}
