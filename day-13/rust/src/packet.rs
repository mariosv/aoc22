use core::cmp::Ordering;

#[derive(Debug, Clone)]
pub enum Packet {
    Num(u32),
    List(Vec<Packet>)
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Packet::Num(n1) => match other {
                Packet::Num(n2) => n1 == n2,
                Packet::List(v2) => {
                    if v2.len() != 1 {
                        return false;
                    }
                    match v2[0] {
                        Packet::Num(n2) => n1 == &n2,
                        _ => self == &v2[0]
                    }
                }
            },
            Packet::List(v1) => match other {
                Packet::Num(_) => other == self,
                Packet::List(v2) => {
                    if v1.len() != v2.len() {
                        return false;
                    }
                    for i in 0..v1.len() {
                        if v1[i] != v2[i] {
                            return false;
                        }
                    }
                    true
                }
            }
        }
    }
}

impl Eq for Packet {}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Packet::Num(n1) => match other {
                Packet::Num(n2) => {
                    if n1 < n2 {
                        Ordering::Less
                    } else if n1 > n2 {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                },
                Packet::List(_) => {
                    let mut v1 = Vec::<Packet>::new();
                    v1.push(Packet::Num(*n1));
                    Packet::List(v1).cmp(other)
                }
            },
            Packet::List(v1) => match other {
                Packet::Num(n2) => {
                    let mut v2 = Vec::<Packet>::new();
                    v2.push(Packet::Num(*n2));
                    return self.cmp(&Packet::List(v2));
                },
                Packet::List(v2) => {
                    for i in 0..v1.len() {
                        if i == v2.len() {
                            return Ordering::Greater;
                        }
                        let ord = v1[i].cmp(&v2[i]);
                        if ord != Ordering::Equal {
                            return ord;
                        }
                    }
                    if v1.len() < v2.len() {
                        return Ordering::Less;
                    }
                    Ordering::Equal
                }
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}
