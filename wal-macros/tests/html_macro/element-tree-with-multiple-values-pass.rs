use wal_macros::html;

fn main() {
    html! {
        <div>
            { String::from("Hello world1") }
            <div>
                "Hello world"
                { String::from("Hello world2") }
                { String::from("Hello world3") }
                <input/>
                { String::from("Hello world4") }
                <div></div>
            </div>
            <div/>
        </div>
    };
}
