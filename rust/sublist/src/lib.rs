#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list.len() == _second_list.len() {
        if _first_list.iter()
            .zip(_second_list)
            .all(|(x, y)| *x == *y) {
            return Comparison::Equal;
        }
    } else if _first_list.len() > _second_list.len() {
        if sublist(_second_list, _first_list) == Comparison::Sublist {
            return Comparison::Superlist;
        }
    }
    else if _first_list.is_empty() || _second_list.windows(_first_list.len())
        .any(|x| sublist(x, _first_list) == Comparison::Equal) {
        return Comparison::Sublist;
    }
    Comparison::Unequal
}
