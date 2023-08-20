use wal_macros::html;

fn main() {
    html! { if let std::option::Option::Some(_value) = std::option::Option::Some("hello") { "hello" } };
    html! { if let std::option::Option::Some(value) = std::option::Option::Some("hello") { value } };
    html! { if let std::option::Option::Some(value) = std::option::Option::Some("hello") { <div>{ value }</div> } };

    html! {
        if let std::option::Option::Some(_value) = std::option::Option::Some("hello") { "hello1" }
        else { "hello2" }
    };
    html! {
        if let std::option::Option::Some(_value) = std::option::Option::Some("hello") { "hello1" }
        else if let std::option::Option::Some(_value) = std::option::Option::Some("hello") { "hello2" }
    };
    html! {
        if let std::option::Option::Some(_value) = std::option::Option::Some("hello") { "hello1" }
        else if let std::option::Option::Some(_value) = std::option::Option::Some("hello") { "hello2" }
        else if let std::option::Option::Some(_value) = std::option::Option::Some("hello") { "hello3" }
    };
    html! {
        if let std::option::Option::Some(_value) = std::option::Option::Some("hello") { "hello1" }
        else if let std::option::Option::Some(_value) = std::option::Option::Some("hello") { "hello2" }
        else { "hello3" }
    };
    html! {
        if let std::option::Option::Some(_value) = std::option::Option::Some("hello") { "hello1" }
        else if let std::option::Option::Some(_value) = std::option::Option::Some("hello") { "hello2" }
        else if let std::option::Option::Some(_value) = std::option::Option::Some("hello") { "hello3" }
        else { "hello4" }
    };
}
