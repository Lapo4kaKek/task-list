use leptos::*;
use task_list_leptos::*;

fn main() {
    mount_to_body(|cx| view! { cx,
        <Board />
    })
}
