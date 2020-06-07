extern crate custom_error;
use self::custom_error::custom_error;

custom_error!{pub ItemNaoExiste{numero_usado: usize, total_elementos: usize} = "You tried to use element {numero_usado} but the array has just {total_elementos} elements."}

pub fn item_nao_existe(numero_item: usize, total_itens_array: usize) -> Result<(), ItemNaoExiste> {
    if numero_item < total_itens_array {
        Ok(())
    } else {
        Err(ItemNaoExiste{numero_usado: numero_item, total_elementos: total_itens_array})
    }
}