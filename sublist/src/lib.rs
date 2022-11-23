#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn has_sublist<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    a.windows(b.len()).any(|w| w.eq(b))
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list.eq(second_list) {
        return Comparison::Equal;
    }
    if first_list.is_empty() {
        return Comparison::Sublist;
    }
    if second_list.is_empty() {
        return Comparison::Superlist;
    }
    if first_list.len() < second_list.len() && has_sublist(second_list, first_list) {
        return Comparison::Sublist;
    }
    if first_list.len() > second_list.len() && has_sublist(first_list, second_list) {
        return Comparison::Superlist;
    }
    Comparison::Unequal
}
