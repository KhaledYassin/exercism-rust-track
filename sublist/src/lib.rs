use std::clone::Clone;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn compare_differet_size_lists<T: PartialEq + Copy>(
    bigger_list: &[T],
    smaller_list: &[T],
) -> bool {
    if bigger_list.len() < smaller_list.len() {
        return false;
    }

    if bigger_list.starts_with(smaller_list) || bigger_list.ends_with(smaller_list) {
        return true;
    }

    let header_range = 1..;

    compare_differet_size_lists(&bigger_list[header_range], smaller_list)
}

pub fn sublist<T: PartialEq + Copy>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let f_size = _first_list.len();
    let s_size = _second_list.len();

    if _first_list == _second_list {
        return Comparison::Equal;
    } else {
        let first_list_bigger = f_size > s_size;
        let bigger_list = if first_list_bigger {
            _first_list
        } else {
            _second_list
        };

        let smaller_list = if first_list_bigger {
            _second_list
        } else {
            _first_list
        };

        let comparison = compare_differet_size_lists(bigger_list, smaller_list);

        let result = if !comparison {
            Comparison::Unequal
        } else {
            if first_list_bigger {
                Comparison::Superlist
            } else {
                Comparison::Sublist
            }
        };

        return result;
    }
}
