pub fn product<T0, T1, I0, I1>(
    outer: I0,
    inner: I1)
 -> impl Iterator<Item = (T0, T1)>
where
    T0: Clone,
    I0: IntoIterator<Item = T0>,
    I1: IntoIterator<Item = T1> + Clone {
    outer.into_iter().flat_map(move |v0| {
        inner.clone().into_iter().map(move |v1| {
            (v0.clone(), v1)
        })
    })
}
