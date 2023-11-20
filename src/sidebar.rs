use leptos::*;
use leptos_icons::*;

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <header id="Sidebar" class="fixed top-0 left-0 z-40 w-1/4 h-screen transition-transform -translate-x-full sm:translate-x-0">
            <div class="h-full px-3 py-4 overflow-y-auto bg-gradient-to-br from-purple-900 to-red-600">
                <h1 class="text-center text-4xl text-white"> Adam Payzant </h1>
                <p class="text-center text-2l text-white">
                    Software Developer
                </p>

                <div id="Socials" class="justify-center flex pt-10">
                    <IconWithLink href="https://www.linkedin.com/in/adam-payzant/".to_string() icon=Icon::from(FaIcon::FaLinkedinBrands) />
                    <IconWithLink href="https://github.com/AdamPayzant/".to_string() icon=Icon::from(AiIcon::AiGithubFilled) />
                    <IconWithLink href="mailto:payzantedwardiv@gmail.com".to_string() icon=Icon::from(FaIcon::FaEnvelopeRegular) />
                </div>
                <ul class="pt-4 mt-4 space-y-2 text-center border-t">
                    <SidebarItem item="Home".to_string() href="#Home".to_string() />
                    <SidebarItem item="About".to_string() href="#About".to_string() />
                    <SidebarItem item="Projects".to_string() href="#Projects".to_string() />
                    <SidebarItem item="Contact".to_string() href="#Contact".to_string() />
                    <SidebarItem item="Resume".to_string() href="#Resume".to_string() />
                </ul>
            </div>
        </header>
    }
}


#[component]
pub fn SidebarItem(item: String, href: String) -> impl IntoView {
    view! {
        <a href={href}>
            <li class="block px-4  py-2 mt-2 text-lg font-semibold
                       text-gray-900 bg-transparent rounded-lg hover:bg-gray-200
                       dark:text-white dark:bg-transparent dark:hover:bg-gray-600">
                {item}
            </li>
        </a>
    }
}

#[component]
fn IconWithLink(
    href: String,
    #[prop(into)]
    icon: MaybeSignal<Icon>
) -> impl IntoView {
    // TODO: Implement a system for detecting color mode. Right now these are preventing me for using a light mode
    // Then again, if I'm using a background image for the sidebar, I may keep the color theme consistent. TBD

    view! {
        <a href=href class="px-1">
            <Icon icon=icon width="35px" height="35px" style="color: white;"/>
        </a>
    }
}
