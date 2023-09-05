use wal_macros::html;

fn empty_iter() -> impl std::iter::Iterator<Item = i32> {
    std::iter::empty()
}

fn empty_vec() -> std::vec::Vec<i32> {
    std::vec::Vec::new()
}

fn main() {
    html! { for empty_iter() };
    html! { for empty_vec() };

    html! { for vec![0, 1, 2] };

    html! { for std::iter::Iterator::map(0..2, |num| { html! { <div>{ num }</div> } }) };

    html! {
        <>
            <div/>
            for { empty_vec() }
        </>
    };
    html! {
        <div>
            for { empty_vec() }
        </div>
    };
}
