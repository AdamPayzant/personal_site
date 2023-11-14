use leptos::*;

mod sidebar;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <sidebar::Sidebar/>
        <button on:click=move |_| {
            set_count.set(count.get() + 1);
        }>
            "Click me: "
            { count }
        </button>
    }
}
