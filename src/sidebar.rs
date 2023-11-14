use leptos::*;

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <aside id="Sidebar" class="fixed top-0 left-0 z-40 w-64 h-screen transition-transform -translate-x-full sm:translate-x-0" aria-label="Sidebar">
            <div class="h-full px-3 py-4 overflow-y-auto bg-gray-50 dark:bg-gray-700">
                <p class="text-center text-4xl">
                    Adam Payzant
                </p>
                <ul class="space-y-2 font-medium text-center">
                    <SidebarItem item="Test".to_string() />
                </ul>
            </div>
        </aside>
    }
}


#[component]
pub fn SidebarItem(item: String) -> impl IntoView {
    view! {
        <li class="text-white">
            {item}
        </li>
    }
}