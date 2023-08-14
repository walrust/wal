use wal_macros::html;

fn main() {
    html! { if true {} };
    html! { if true { {"hello"} } };
    let value = "hello";
    html! { if true { { value } } };
    html! { if true { <div/>} };
    html! { if true { <div/><div/> } };
    html! { if true { { html! {} } } };
    html! { if true { { let _x = 1; html! {} } } };

    html! { if true { {"hello1"} } else { {"hello2"} } };
    html! { if true { {"hello1"} } else if true { {"hello2"} } };
    html! { if true { {"hello1"} } else if true { {"hello2"} } else { {"hello3"} } };
    html! { if true { {"hello1"} } else if true { {"hello2"} } else if true { {"hello3"} } else { {"hello4"} } };

    html! {
        </>
            <div/>
            if true {}
            <div/>
        </>
    };
    html! {
        <div>
            if true {}
        </div>
    };

    let cond = true;
    html! { if cond {} };
    html! { if cond { {"hello1"} } else if cond { {"hello2"} } };
    html! { if cond { {"hello1"} } else if cond { {"hello2"} } else { {"hello2"} } };

    html! { if 1 == 1 {} };
    html! { if 1 == 1 { {"hello1"} } else if 1 == 1 { {"hello2"} } };

    let x = 1;
    html! { if x == 1 {} };
    html! { if x == 1 { {"hello1"} } else if x == 1 { {"hello2"} } };

    html! { if { let _x = 1; true } {} };
    html! { if { let _x = 1; true } {} else if { let _x = 1; true } {} };
}
