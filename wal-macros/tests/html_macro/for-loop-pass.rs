use wal_macros::html;

fn empty_iter() -> impl std::iter::Iterator<Item = wal::Html> {
    std::iter::empty()
}

fn empty_vec() -> std::vec::Vec<wal::Html> {
    std::vec::Vec::<wal::Html>::new()
}

fn main() {
    html! { for empty_iter() };
    let empty_iter = empty_iter();
    html! { for empty_iter };

    html! { for { empty_iter() } };
    let empty_iter = empty_iter();
    html! { for { empty_iter } };

    html! { for empty_vec() };
    let empty_vec = empty_vec();
    html! { for empty_vec };

    html! { for { empty_vec() } };
    let empty_vec = empty_vec();
    html! { for { empty_vec } };

    html! { for std::iter::IntoIterator::into_iter(empty_vec()) };

    html! { for vec![0, 1, 2] };

    html! { for std::iter::Iterator::map(0..2, |num| {html! { <div>{ num }</div> }}) };

    html! {
        <>
            <div/>
            { for empty_vec() }
        </>
    };
}
