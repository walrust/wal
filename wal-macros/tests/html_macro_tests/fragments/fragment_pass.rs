use wal_macros::html;

fn main() {
    html! { <></> };
    html! {
        <>
            <div></div>
            <div></div>
        </>
    };
    html! {
        <div>
            <></>
        </div>
    }
    html! { <key="value"></> };

    let children = vec![1, 2];
    html! { <> { children } </> };
}
