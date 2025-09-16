#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    pub fn is_sublist_of<T: PartialEq>(small: &[T], big: &[T]) -> bool {
        if small.is_empty() {
            return true;
        } // empty list is sublist of anything
        if small.len() > big.len() {
            return false;
        }

        big.windows(small.len()).any(|window| window == small)
    }
    if first_list == second_list {
        Comparison::Equal
    } else if is_sublist_of(first_list, second_list) {
        Comparison::Sublist
    } else if is_sublist_of(second_list, first_list) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}
