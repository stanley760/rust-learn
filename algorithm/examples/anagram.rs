use std::collections::HashMap;

struct HashSolution;

impl HashSolution {
    pub(crate) fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // let mut map: HashMap<Vec<u8>, Vec<String>> = HashMap::new();
        // for ele in strs {
        //     let mut e = ele.as_bytes().to_vec();
        //     e.sort_unstable();
        //     map.entry(e).or_default().push(ele);
        // }
        // map.into_values().collect()

        let x = strs
            .iter()
            .map(|s| {
                let mut v = s.as_bytes().to_vec();
                v.sort_unstable();
                (v, s)
            })
            .fold(HashMap::new(), |mut acc, (k, v)| {
                acc.entry(k).or_insert(vec![]).push(v.to_string());
                acc
            })
            .into_values()
            .collect();
        x
    }
}

fn main() {
    let strs = vec![
        String::from("eat"),
        String::from("tea"),
        String::from("tan"),
        String::from("ate"),
        String::from("nat"),
        String::from("bat"),
    ];
    let res = HashSolution::group_anagrams(strs);
    println!("{:?}", res);
}
