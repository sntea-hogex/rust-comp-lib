fn sliding_minimum<T>(a: Vec<T>, width: usize) -> Vec<T>
where
    T: std::cmp::PartialOrd + Clone + std::fmt::Debug,
{
    let mut res = Vec::with_capacity(a.len()-width);
    let mut set = std::collections::VecDeque::new();
    for (i, e) in a.iter().enumerate().take(width) {
        while let Some((v, i)) = set.pop_back() {
            if v < e {
                set.push_back((v, i));
                break;
            }
        }
        set.push_back((e, i));
    }
    for (s, e) in a.iter().skip(width).enumerate() {
        res.push(set.front().expect("sliding_minimum error").0.clone());
        while let Some((v, i)) = set.pop_back() {
            if v < e {
                set.push_back((v, i));
                break;
            }
        }
        set.push_back((e, s+width));
        while let Some((v, i)) = set.pop_front() {
            if s+1 <= i {
                set.push_front((v, i));
                break;
            }
        }
    }
    res.push(set.front().expect("sliding_minimum error").0.clone());
    
    res
}
