// one of the genius ideas I saw in one youtube tutorial
// simplify code repition using macros, follow

// creating a set using a list of arguments
// * 1,2,3 =>  Struct { 1, 2, 3 }

// credits: https://www.youtube.com/watch?v=IsCBibC0PZE
macro_rules! set {
        ($($s:expr), +) => { HashSet::from([$($s)*])}
    }

pub(crate) use set;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let a = set!(1, 2, 3);
        assert_eq!(a, HashSet::from([1, 2, 3]));
    }
}
