#[cfg(test)]
mod loops_tests {
    use std::ops::Add;

    #[test]
    fn test_loop_con_break() {
        // Objetivo: Entender el bucle `loop` y cómo usar `break` para salir y devolver un valor.

        let mut contador = 0;

        // 1. Usa un bucle `loop` para incrementar `contador`.
        //    El `loop` debe detenerse cuando `contador` llegue a 10.
        //    Cuando se detenga, debe devolver el valor de `contador` multiplicado por 2.
        let modified_counter = loop {
            contador += 1;
            if contador == 10{
                break contador * 2;
            }
        };
        // 2. Verifica que el resultado devuelto por el `loop` es 20.
        assert_eq!(modified_counter, 20);
    }

    #[test]
    fn test_while_loop() {
        // Objetivo: Usar un bucle `while` para ejecutar código mientras una condición sea verdadera.

        let mut numero = 3;

        // 1. Usa un bucle `while` para decrementar `numero` hasta que llegue a 0.
        while numero != 0 {
            numero -= 1;
        }
        // 2. Verifica que, después del bucle, `numero` es 0.
        assert_eq!(numero, 0);

    }

    #[test]
    fn test_for_con_rango() {
        // Objetivo: Usar un bucle `for` para iterar sobre un rango de números.

        let mut suma = 0;

        // 1. Usa un bucle `for` para iterar sobre los números del 1 al 5 (incluido).
        //    Suma cada número a la variable `suma`.
        //    El rango `1..=5` incluye el 5.
        for num in 0..=5 {
            suma += num;
        }

        // 2. Verifica que la suma sea la correcta (1 + 2 + 3 + 4 + 5 = 15).
        assert_eq!(suma, 15);

        // 3. Usa un bucle `for` con `.rev()` para contar hacia atrás.
        let expected_array: [i32;6] = [5,4,3,2,1,0];
        let mut new_array: [i32;6] = [0;6];
        let mut i = 0;
        for num in (0..=5).rev() {
            new_array[i] = num;
            i += 1;
        }
        for it in 0..5 {
            let one = new_array[it];
            let two = expected_array[it];
            assert_eq!(one, two);()
        }
    }

    #[test]
    fn test_for_con_iterador_de_coleccion() {
        // Objetivo: Usar un bucle `for` para iterar sobre los elementos de una colección (array).

        // 1. Usa un bucle `for` para iterar sobre cada elemento del array `a`.
        //    El método `.iter()` crea un iterador que nos permite recorrer la colección.
        let a = ["a";10];
        let mut sum = String::from("");
        for an_a in a.iter() {
            sum = sum.add(an_a);
        }
        // 2. Verifica que la suma de los elementos sea correcta.
        assert_eq!(sum, "aaaaaaaaaa");

    }
}
