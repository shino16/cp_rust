fn get_q() -> Vec<Vec<bool>> {
    unimplemented!()
}

fn next_state(mut s: Vec<bool>, op: Op) -> Vec<bool> {
    unimplemented!()
}

fn is_accepted(v: &Vec<bool>) -> bool {
    unimplemented!()
}

fn run<F, G, H, State: Clone + Ord, Alphabet: Clone>(
    get_q: F,
    is_accepted: G,
    next_state: H,
    alphabet: &[Alphabet],
) -> Vec<BTreeSet<State>>
where
    F: FnOnce() -> Vec<State>,
    G: Fn(&State) -> bool,
    H: Fn(State, Alphabet) -> State,
{
    let q = get_q();
    let (f, notf) = q
        .iter()
        .cloned()
        .partition::<BTreeSet<_>, _>(|v| is_accepted(v));

    let mut p = vec![f.clone(), notf.clone()];
    let mut p2 = Vec::new();
    let mut w = vec![f, notf];
    while let Some(a) = w.pop() {
        for op in alphabet {
            let x: BTreeSet<_> = q
                .iter()
                .cloned()
                .filter(|s| a.get(&next_state(s.clone(), op.clone())).is_some())
                .collect();
            for y in p.drain(..) {
                let inter = x.intersection(&y);
                let inter: BTreeSet<_> = inter.cloned().collect();
                let imply = y.difference(&x);
                let imply: BTreeSet<_> = imply.cloned().collect();
                if inter.is_empty() || imply.is_empty() {
                    p2.push(y);
                    continue;
                }
                p2.push(inter.clone());
                p2.push(imply.clone());
                if let Some((i, _)) = w.iter().enumerate().filter(|&(_, s)| s == &y).next() {
                    w.swap_remove(i);
                    w.push(inter);
                    w.push(imply);
                } else if inter.len() <= imply.len() {
                    w.push(inter);
                } else {
                    w.push(imply);
                }
            }
            std::mem::swap(&mut p, &mut p2);
        }
    }
    p.sort_unstable();
    p
}
