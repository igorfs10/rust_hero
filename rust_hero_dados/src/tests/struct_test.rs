#[cfg(test)]
pub mod tests {
    use core::fmt;

    #[must_use]
    pub struct TestStruct<T, U> {
        pub esperado: Option<T>,
        pub parametros: Option<U>,
        pub funcao: Option<fn(U) -> T>,
    }

    impl<T, U> TestStruct<T, U>
    where
        T: PartialEq + fmt::Debug,
    {
        pub fn novo() -> Self {
            TestStruct {
                esperado: None,
                parametros: None,
                funcao: None,
            }
        }

        pub fn espera(mut self, esperado: T) -> Self {
            self.esperado = Some(esperado);
            self
        }

        pub fn parametriza(mut self, parametros: U) -> Self {
            self.parametros = Some(parametros);
            self
        }

        pub fn funcao(mut self, funcao: fn(U) -> T) -> Self {
            self.funcao = Some(funcao);
            self
        }

        pub fn testar(self) {
            assert_eq!(
                self.esperado.unwrap(),
                self.funcao.unwrap()(self.parametros.unwrap())
            );
        }
    }
}
