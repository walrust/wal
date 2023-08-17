use wal_macros::html;

fn main() {
    html! {
        if true {}
        else if let std::option::Option::Some(_value) = std::option::Option::Some("hello") {}
    };
    html! {
        if true {}
        else if let std::option::Option::Some(_value) = std::option::Option::Some("hello") {}
        else {}
    };
    html! {
        if let std::option::Option::Some(_value) = std::option::Option::Some("hello") {}
        else if true {}
    };
    html! {
        if let std::option::Option::Some(_value) = std::option::Option::Some("hello") {}
        else if true {}
        else {}
    };
}
