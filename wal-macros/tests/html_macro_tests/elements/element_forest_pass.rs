use wal_macros::html;

fn main() {
    html! {
        <div></div>
        <span></span>
        <input/>
        <div>
            <span/>
            { String::from("Hello world") }
        </div>
        <div/>
    };
}
