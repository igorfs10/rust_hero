use core::fmt;

#[must_use]
#[derive(PartialEq, Debug)]
pub struct TestStruct<T, U> {
    pub esperado: T,
    pub parametros: Option<U>,
    pub funcao: Option<fn(U) -> T>,
}

impl<T, U> TestStruct<T, U>
where
    T: PartialEq + fmt::Debug,
    U: PartialEq,
{
    pub fn espera(esperado: T) -> Self {
        TestStruct {
            esperado,
            parametros: None,
            funcao: None,
        }
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
            self.esperado,
            self.funcao.unwrap()(self.parametros.unwrap())
        );
    }
}
