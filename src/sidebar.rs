use leptos::*;

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <aside id="Sidebar" class="fixed top-0 left-0 z-40 w-96 h-screen transition-transform -translate-x-full sm:translate-x-0" aria-label="Sidebar">
            <div class="h-full px-3 py-4 overflow-y-auto bg-gray-50 dark:bg-gray-700">
                <h1 class="text-center text-4xl text-white"> Adam Payzant </h1>
                <p class="text-center text-2l text-white">
                    Software Developer
                </p>
                <ul class="pt-4 mt-4 space-y-2 font-medium text-center border-t">
                    <SidebarItem item="Home".to_string() href="#Home".to_string() />
                    <SidebarItem item="About".to_string() href="#About".to_string() />
                    <SidebarItem item="Projects".to_string() href="#Projects".to_string() />
                    <SidebarItem item="Contact".to_string() href="#Contact".to_string() />
                    <SidebarItem item="Resume".to_string() href="#Resume".to_string() />
                </ul>
            </div>
        </aside>
    }
}


#[component]
pub fn SidebarItem(item: String, href: String) -> impl IntoView {
    view! {
        <a href={href}>
            <li class="block px-4  py-2 mt-2 text-sm font-semibold
                       text-gray-900 bg-transparent rounded-lg hover:bg-gray-200
                       dark:text-white dark:bg-transparent dark:hover:bg-gray-600">
                {item}
            </li>
        </a>
    }
}