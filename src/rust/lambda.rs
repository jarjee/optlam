//Would be better if the terms could be typechecked (to make strongly normalising).
//Currently working out the minimum required to enforce terms of only elementary-affine-logic
//typeable terms.
enum Term {
    Lam(Term),
    App(Term, Term),
    Var(u32),
}

fn reduce (term: Term) -> Term {
    match term {
        Term::Var(x) => Term::Var(x),
        Term::Lam(t) => Term::Lam(reduce(t)),
        Term::App(l, r) => {
            let left  = reduce(l);
            let right = reduce(r);
            match left {
                Term::Lam(t) => reduce(substitute(right, true, 0, -1, left)),
                _ => Term::App(left, right),
            }
        }
    }
    fn substitute(value: Term, subs: bool, depth: i32, wrap: i32, term: Term) -> Term {
        match term {
            Term::App(l,r) => Term::App(substitute(value, subs, depth, wrap, l)
                                      ,substitute(value, subs, depth, wrap, r)),
            Term::Lam(t) => Term::Lam(substitute(value, subs, depth+1, wrap, t)),
            Term::Var(x) => if subs && x == depth {
                substitute(Term::Var(0), false, -1, depth, value)
            } else {
                let delta = if x > depth {wrap} else {0};
                Term::Var(x + delta)
            }
        }
    }
}

fn fold<T>(var: Fn(u32) -> T, lam: Fn(T) -> T, app: Fn(T, T) -> T, term: Term) -> T{
    match term {
        Term::Var(x) => var(x),
        Term::Lam(t) => lam(fold(var, lam, app, t)),
        Term::App(l, r) => app(fold(var, lam, app, l), fold(var, lam, app, r)),
    }
}

fn nat(num: u32) -> Term {
    fn go(x: u32) -> Term {
        match x {
            0 => Term::Var(0),
            x => Term::App(Term::Var(1), go(x-1))
        }
    }

    Term::Lam(Term::Lam(go(num)));
}

fn nat_(term: Term) -> u32 {
    size(term)-1
}

fn size(term: Term) -> u32 {
    fold(|x: u32| 1, |t: Term| term, |l: Term, r: Term| l + r)
}
