use leptos::*;

mod sidebar;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <sidebar::Sidebar/>
    }
}
