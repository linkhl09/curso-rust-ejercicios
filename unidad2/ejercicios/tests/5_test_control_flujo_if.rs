#[cfg(test)]
mod if_statement_tests {

    // La construcción `if` permite bifurcar el código dependiendo de una condición.
    // En Rust, `if` es una expresión, lo que significa que puede devolver un valor.

    #[test]
    fn basic_if_else_test() {
        // Objetivo: Entender la estructura básica de un `if-else`.

        let numero = 10;
        let resultado: &str;

        // 1. Escribe una condición `if` para verificar si `numero` es mayor que 5.
        if numero > 5 {
            resultado = "It will be";
        }
        else {
            resultado = "It will never be";
        }

        // 2. Verifica que el resultado es el esperado.
        assert_eq!(resultado, "It will be");
    }

    #[test]
    fn test_if_else_if_else() {
        // Objetivo: Manejar múltiples condiciones con `else if`.

        let n = 6;

        // 1. Escribe una cadena de `if-else if-else` para clasificar `n`.
        // - Si `n` es divisible por 4, devuelve "Divisible por 4".
        // - Si `n` es divisible por 3, devuelve "Divisible por 3".
        // - Si `n` es divisible por 2, devuelve "Divisible por 2".
        // - De lo contrario, devuelve "No es divisible por 2, 3 ni 4".
        let if_result: &str;
        if n % 4 == 0 {
            if_result = "Divisible by 4";
        } else if n % 3 == 0 {
            if_result = "Divisible by 3";
        }
        else if n % 2 == 0 {
            if_result = "Divisible by 2";
        }
        else {
            if_result = "Not divisible by 2, 3 or 4";
        }

        // 2. Verifica el resultado para `n = 6`.
        assert_eq!(if_result, "Divisible by 3")
    }

    #[test]
    fn test_if_en_una_declaracion_let() {
        // Objetivo: Usar `if` como una expresión para asignar un valor a una variable.

        let mut condicion = true;

        // 1. Usa una expresión `if` para asignar un valor a la variable `numero`.
        // Si `condicion` es verdadera, `numero` debe ser 5. Si no, debe ser 0.
        // Nota: Ambos bloques (`if` y `else`) deben devolver el mismo tipo.
        let mut numero = if condicion { 5 } else { 0 };

        // 2. Verifica el valor de `numero`.
        assert_eq!(numero, 5);

        // 3. Haz lo mismo con la condición en `false`.
        condicion = false;
        numero = if condicion { 5 } else { 0 };
        assert_eq!(numero, 0);
    }

    #[test]
    fn test_condiciones_combinadas() {
        // Objetivo: Usar operadores lógicos `&&` (Y) y `||` (O) en las condiciones.

        let edad = 25;
        let tiene_licencia = true;

        // 1. Escribe una condición que verifique si la persona es mayor de edad (>= 18) Y tiene licencia.
        if edad > 18 && tiene_licencia {}

        let es_estudiante = true;
        let es_fin_de_semana = false;

        // 2. Escribe una condición que verifique si la persona puede descansar: si es fin de semana O no es estudiante.
        if es_fin_de_semana || !es_estudiante {}
    }
}
