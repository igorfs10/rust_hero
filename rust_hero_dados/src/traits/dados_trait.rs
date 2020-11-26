pub trait DadosTrait {
    fn get_id(&self) -> usize;
    fn get_dados(&self) -> String;

    // Implementar quando lançarem traits constantes
    // fn get_id(&self) -> usize;
}

#[derive(Default, Debug)]
pub struct Repo<T>(T);

impl<T> BaseRepo<T> for Repo<T>
where
    T: DadosTrait + Default + Clone,
{
    fn get_by_id(lista: Vec<T>, id: usize) -> Option<T> {
        let mut ret = None;
        for item in lista.iter() {
            if item.get_id() == id {
                ret = Some(item.clone());
                break;
            }
        }
        ret
    }
    // fn get_all()->Vec<T>{
    //     return vec![43]
    // }
}

pub trait BaseRepo<T> {
    // fn get_all() -> Vec<T>;
    fn get_by_id(lista: Vec<T>, id: usize) -> Option<T>;
}
