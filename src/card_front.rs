use leptos::*;
use leptos_icons::*;

#[component]
pub fn CardFront() -> impl IntoView {
    view! {
        <div class="flex flex-row h-full">
            <FrontLeft/>
            <FrontRight/>
        </div>
    }
}

#[component]
fn FrontLeft() -> impl IntoView {
    view! {
        <div class=" w-[60%] pt-10 pb-10  h-full">
            <div class="border-r-2 h-full border-slate-500">
                <p class="pt-20 pr-9 pl-16">
                    <h1 class="text-4xl font-bold">
                        "Hi, I'm Adam!"
                    </h1>
                    <p class="pt-12">
                        "My name is Adam Payzant, a software developer based in Ottawa, Ontario.
                         I have a Bachelor's of Computer Science. I'm very passionate about learning
                         new technologies, as well as expanding my understanding of existing tech. "
                    </p>
                    <p class="pt-10">
                        "My focus has been on networking, low level programming, high performance computing,
                         systems programming, and programming language design and I'm always interested in exploring
                         new areas. My most used programming languages are C, Rust, and Python, additionally I have 
                         experience with C++, Go, and Javascript as well as more niche languages like Zig."
                    </p>
                </p>
            </div>
        </div>
    }
}

#[component]
fn FrontRight() -> impl IntoView {
    view! {
        <div class="pt-10 pb-10">
            <div class="absolute top-5 right-5">
                <div class="flex flex-row">
                    <IconWithLink href="https://www.linkedin.com/in/adam-payzant/".to_string() icon=Icon::from(FaIcon::FaLinkedinBrands) />
                    <IconWithLink href="https://github.com/AdamPayzant/".to_string() icon=Icon::from(AiIcon::AiGithubFilled) />
                    <IconWithLink href="mailto:payzantedwardiv@gmail.com".to_string() icon=Icon::from(FaIcon::FaEnvelopeRegular) />
                    <IconWithLink href="resume.pdf".to_string() icon=Icon::from(FaIcon::FaFileRegular) />
                </div>
            </div>
            <div class="relative pt-10 left-4">
                <p class="border-b-2 border-slate-500 py-2 text-2xl">
                    "Education:"
                </p>
                <div class="py-2">
                    <ExperienceHeader title="Bachelor's of Computer Science".to_string() who="Carleton University".to_string()
                     start="September 2017".to_string() end="June 2021".to_string()/>
                </div>
                <p class="border-b-2 border-slate-500 py-2 text-2xl">
                    "Experience:"
                </p>
                <div class="py-2">
                    <ExperienceHeader title="Software Engineer".to_string() who="Cisco Systems".to_string()
                     start="May 2021".to_string() end="Present".to_string()/>
                </div>
                <div class="py-1">
                    <ExperienceHeader title="Intern".to_string() who="Oak Ridge National Labs".to_string()
                    start="May 2018".to_string() end="August 2018".to_string()/>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ExperienceHeader(title: String, who: String, start: String, end: String) -> impl IntoView {
    view! {
        <h1 class="text-xl">
            {title}
        </h1>
        <h2 class="text-lg">
            {who}
        </h2>
        <p class="pl-3 flex">
            <div class="px-1">
                <Icon icon=Icon::from(FaIcon::FaCalendarDaysRegular) width="20px" height="20px" style="color: fdf6e3;"/>
            </div>
            {start} - {end}
        </p>
    }
}


#[component]
fn IconWithLink(href: String, #[prop(into)] icon: MaybeSignal<Icon>) -> impl IntoView {
    view! {
        <a href=href class="px-1.5 transition duration-100 hover:scale-125">
            <Icon icon=icon width="35px" height="35px" style="color: fdf6e3;"/>
        </a>
    }
}