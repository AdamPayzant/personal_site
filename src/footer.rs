use leptos::*;
use leptos_icons::*;


#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="aboslute text-center mb-0 min-w-full bg-gray-700 p-10 text-gray-100">
            <div id="Socials" class="justify-center flex pb-10">
                <IconWithLink href="https://www.linkedin.com/in/adam-payzant/".to_string() icon=Icon::from(FaIcon::FaLinkedinBrands) />
                <IconWithLink href="https://github.com/AdamPayzant/".to_string() icon=Icon::from(AiIcon::AiGithubFilled) />
                <IconWithLink href="mailto:payzantedwardiv@gmail.com".to_string() icon=Icon::from(FaIcon::FaEnvelopeRegular) />
            </div>
            <div>
                <p>
                    Developed using <a href="https://leptos.dev/" class="text-blue-200"> Leptos </a> with icons from 
                    <a href="https://fontawesome.com/license" class="text-blue-200"> Font Awesome </a> and 
                    <a href="https://github.com/ant-design/ant-design-icons" class="text-blue-200"> Ant Design Icons </a>!
                </p>
                <p>
                    "©" Adam Payzant
                </p>
            </div>
        </footer>
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
