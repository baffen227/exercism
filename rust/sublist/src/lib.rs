#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    //todo!("Determine if the first list is equal to, sublist of, superlist of or unequal to the second list.");
    if _first_list == _second_list {
        Comparison::Equal
    } else {
        let _first_len = _first_list.len();
        let _second_len = _second_list.len();


        /*
        let mut longer_len = 0;
        if _first_len <= _second_len {
            longer_len = _second_len;
        } else {
            longer_len = _first_len;
        }
        */


        // TODO: refine the flow
        if _first_len <= _second_len {
            let mut start_idx = 0;
            let mut end_idx = start_idx + _first_len;
            while end_idx <= _second_len {
                if _first_list == &_second_list[start_idx..end_idx] {
                    return Comparison::Sublist;
                } else {
                    start_idx += 1;
                    end_idx = start_idx + _first_len;
                }
            }
            return Comparison::Unequal;

        } else {
            let mut start_idx = 0;
            let mut end_idx = start_idx + _second_len;
            while end_idx <= _first_len {
                if _second_list == &_first_list[start_idx..end_idx] {
                    return Comparison::Superlist;
                } else {
                    start_idx += 1;
                    end_idx = start_idx + _second_len;
                }
            }
            return Comparison::Unequal;
        }

    }
}
