pub fn item_nao_existe(numero_item: usize, total_itens_array: usize) -> Result<(), String> {
    if numero_item < total_itens_array {
        return Ok(());
    } else {
        return Err(format!("You tried to use element {} but the array goes from 0 to {}.", numero_item, total_itens_array - 1));
    }
}