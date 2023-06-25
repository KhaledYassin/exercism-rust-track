use std::collections::BTreeMap;
pub fn lowest_price(books: &[u32]) -> u32 {
    if books.is_empty() {
        0
    } else {
        let books_mapped = books.iter().fold(BTreeMap::new(), |mut map, book| {
            *map.entry(book).or_insert(0) += 1;
            map
        });

        let mut minimum_price = u32::MAX;

        for book_group in 1..=books_mapped.len() {
            let mut total_price = 0;
            let mut remaining_books = books_mapped.clone();

            while !remaining_books.is_empty() {
                let mut books_drawn = 0;

                for (_, quantity) in remaining_books.iter_mut() {
                    if *quantity > 0 {
                        *quantity -= 1;

                        books_drawn += 1;

                        if books_drawn == book_group {
                            break;
                        }
                    }
                }

                remaining_books.retain(|_, quantity| *quantity > 0);

                total_price += discount_price(books_drawn);
            }

            minimum_price = minimum_price.min(total_price);
        }

        minimum_price
    }
}
fn discount_price(count: usize) -> u32 {
    match count {
        2 => 1520,
        3 => 2160,
        4 => 2560,
        5 => 3000,
        _ => 800,
    }
}
