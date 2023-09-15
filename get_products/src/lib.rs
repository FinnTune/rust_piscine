pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    if arr.len() <= 1 {
        return Vec::new();
    }

    let total_product: usize = arr.iter().product();

    arr.into_iter()
        .map(|num| {
            if num == 0 {
                return total_product;
            }
            total_product / num
        })
        .collect()
}
