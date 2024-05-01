#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        Comparison::Equal
    } else {
        let mut shorter_list = _first_list;
        let mut longer_list = _second_list;
        if _first_list.len() > _second_list.len() {
            shorter_list = _second_list;
            longer_list = _first_list;
        }

        let mut start_idx = 0;
        let mut end_idx = start_idx + shorter_list.len();
        while end_idx <= longer_list.len() {
            if shorter_list == &longer_list[start_idx..end_idx] {
                if shorter_list == _first_list {
                    return Comparison::Sublist;
                } else {
                    return Comparison::Superlist;
                }
            } else {
                start_idx += 1;
                end_idx = start_idx + shorter_list.len(); 
            }
        }
        return Comparison::Unequal;
    }
}
