use wal_macros::html;

fn main() {
    let empty = || ();
    html! {
        empty()
    };

    let not_node = || "not node";
    html! {
        not_node()
    };
}
