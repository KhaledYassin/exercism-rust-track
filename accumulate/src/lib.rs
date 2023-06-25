/// What should the type of _function be?
pub fn map<F, T, R>(input: Vec<T>, mut function: F) -> Vec<R>
where
    F: FnMut(T) -> R,
{
    let mut output = vec![];

    input
        .into_iter()
        .for_each(|element| output.push(function(element)));

    output
}
