//Would be better if the terms could be typechecked (to make strongly normalising).
//Currently working out the minimum required to enforce terms of only elementary-affine-logic
//typeable terms.

#[derive(Clone)]
enum Term {
    Lam(Box<Term>),
    App(Box<Term>, Box<Term>),
    Var(i32),
}

fn reduce (term: Term) -> Term {
    fn substitute(value: Term, subs: bool, depth: i32, wrap: i32, term: Term) -> Term {
        match term {
            Term::App(l,r) => {
                let v = value.clone();
                Term::App(Box::new(substitute(value, subs, depth, wrap, *l))
                , Box::new(substitute(v, subs, depth, wrap, *r)))
            },
            Term::Lam(t) => Term::Lam(Box::new(substitute(value, subs, depth+1, wrap, *t))),
            Term::Var(x) => if subs && x == depth {
                substitute(Term::Var(0), false, -1, depth, value)
            } else {
                let delta = if x > depth {wrap} else {0};
                Term::Var(x + delta)
            }
        }
    }
    match term {
        Term::Var(x) => Term::Var(x),
        Term::Lam(t) => Term::Lam(Box::new(reduce(*t))),
        Term::App(l, r) => {
            let left  = reduce(*l);
            let right = reduce(*r);
            match left {
                Term::Lam(_) => reduce(substitute(right, true, 0, -1, left)),
                _ => Term::App(Box::new(left), Box::new(right)),
            }
        },
    }
}

fn fold<T>(var: &Fn(i32) -> T, lam: &Fn(T) -> T, app: &Fn(T, T) -> T, term: Term) -> T{
    match term {
        Term::Var(x) => var(x),
        Term::Lam(t) => lam(fold(var, lam, app, *t)),
        Term::App(l, r) => app(fold(var, lam, app, *l), fold(var, lam, app, *r)),
    }
}

fn nat(num: u32) -> Term {
    fn go(x: u32) -> Term {
        match x {
            0 => Term::Var(0),
            x => Term::App(Box::new(Term::Var(1)), Box::new(go(x-1)))
        }
    }

    Term::Lam(Box::new(Term::Lam(Box::new(go(num)))))
}

fn nat_(term: Term) -> u32 {
    size(term)-1
}

fn size(term: Term) -> u32 {
    fold(&|x: i32| 1, &|t: u32| t, &|l: u32, r: u32| l + r, term)
}
