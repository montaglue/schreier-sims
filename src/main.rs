use std::{
    collections::{BTreeMap, HashSet},
    ops::Range,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Permutation {
    pub p: Vec<usize>,
}

impl Permutation {
    pub fn len(&self) -> usize {
        self.p.len()
    }

    pub fn is_empty(&self) -> bool {
        self.p.is_empty()
    }

    pub fn inv(&self) -> Permutation {
        let mut res = vec![0; self.len()];

        for i in 0..self.len() {
            res[self.p[i]] = i;
        }

        Permutation { p: res }
    }
}

impl From<Range<usize>> for Permutation {
    fn from(r: Range<usize>) -> Self {
        Permutation { p: r.collect() }
    }
}

impl<'a> std::ops::Mul for &'a Permutation {
    type Output = Permutation;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.len(), rhs.len());

        let mut res = vec![0; self.len()];

        for (i, p) in self.p.iter().enumerate() {
            res[i] = rhs.p[*p];
        }

        Permutation { p: res }
    }
}

impl std::ops::Mul for Permutation {
    type Output = Permutation;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.len(), rhs.len());

        let mut res = vec![0; self.len()];

        for (i, p) in self.p.iter().enumerate() {
            res[i] = rhs.p[*p];
        }

        Permutation { p: res }
    }
}

pub fn build_schreier_tree(
    w: usize,
    s: &HashSet<Permutation>,
    orbit: &mut BTreeMap<usize, Permutation>,
) {
    for g in s {
        if !orbit.contains_key(&g.p[w]) {
            orbit.insert(g.p[w], g * orbit.get(&w).unwrap());
            build_schreier_tree(g.p[w], s, orbit);
        }
    }
}

pub fn make_gen(
    s: HashSet<Permutation>,
    orbit: &BTreeMap<usize, Permutation>,
) -> HashSet<Permutation> {
    let mut new_s = HashSet::new();

    for elem in s {
        for u in orbit.keys() {
            new_s.insert(orbit[&elem.p[*u]].inv() * (&elem * &orbit[u]));
        }
    }

    new_s
}

pub fn normalize(n: usize, s: HashSet<Permutation>) -> HashSet<Permutation> {
    let mut new_s = HashSet::new();

    let mut base = vec![BTreeMap::<usize, Permutation>::new(); n];

    for mut elem in s {
        for x in 0..n {
            if elem.p[x] != x {
                if base[x].contains_key(&elem.p[x]) {
                    elem = &elem.inv() * base[x].get(&elem.p[x]).unwrap();
                } else {
                    let y = elem.p[x];
                    let map = &mut base[x];
                    map.insert(y, elem.clone());
                    new_s.insert(elem);
                    break;
                }
            }
        }
    }

    new_s
}

pub fn schreier_sims(n: usize, mut s: HashSet<Permutation>) -> Vec<Vec<Permutation>> {
    let mut ans = vec![];
    let mut w = 0;

    while !s.is_empty() {
        let mut orbit: BTreeMap<usize, Permutation> = BTreeMap::new();
        orbit.insert(w, (0..n).into());

        build_schreier_tree(w, &s, &mut orbit);

        s = normalize(n, make_gen(s, &orbit));

        ans.push(orbit.into_values().collect::<Vec<_>>());

        w += 1;
    }

    ans
}

fn main() {
    let n = 12;
    let s = vec![
        Permutation {
            p: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 0],
        },
        Permutation {
            p: vec![0, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1],
        },
    ];

    let s = s.into_iter().collect::<HashSet<_>>();

    let ans = schreier_sims(n, s);

    for (i, a) in ans.iter().enumerate() {
        println!("Level {}", i);
        for g in a {
            println!("{:?}", g);
        }
    }
}
