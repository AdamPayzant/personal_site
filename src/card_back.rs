use leptos::*;
use leptos_icons::*;


// Types and constants
#[derive(Clone, Copy)]
struct Project {
    index: usize,
    name: &'static str,
    link: &'static str,
    completed: bool,
    description: &'static str,
}

const PROJECTS: [Project; 3] = [
    Project {
        index: 0,
        name: "Operating System",
        link: "https://github.com/AdamPayzant/rtos_project",
        completed: false,
        description: "A simple RISC-V microkernel with plans to add real time guarantees written in Zig.
                      This was inspired because I felt that OS development was a major gap in
                      my knowledge, as I knew high level OS concepts from userspace as well as some
                      embedded knowledge, but lacked the knowledge with concepts like scheduling,
                      virtual memory, processes, and syscalls on an implementation level.
                      Currently the project is restricted to booting in QEMU, a UART driver, and an
                      interrupt/syscall system, with plans to implement virtual memory and process handling.",
    },
    Project {
        index: 1,
        name: "doit.rs",
        link: "https://github.com/AdamPayzant/doit_rs",
        completed: true,
        description: "A privledge escalation tool for Linux written akin to sudo
                      or doas. I was inspired to work on it after reading about CVE-2021-3156 Baron
                      Samedit vulnerability in sudo as well as the simplicity of opendoas.
                      Obviously this program is not exceedingly well audited and as such should
                      not be trusted for practical use, but it was an excellant learning oppurtunity
                      both for privledge escalation, general Rust programming, and FFI code to
                      interface with Shadow and PAM for authentication.",
    },
    Project {
        index: 2,
        name: "Personal Site",
        link: "https://github.com/AdamPayzant/personal_site",
        completed: true,
        description: "This is the site that you're currently looking at.
                      It's been developed using the Rust web framework Leptos.
                      Previously I was using a Hugo generated static site as my
                      personal site, and still feel that Leptos was massively
                      overkill for what I was wanting from a website. However, I chose Leptos
                      as it provided a great learning oppurtunity both for getting back
                      into web frameworks as well as seeing what modern WASM can do.",
    },
];


// Components
#[component]
pub fn CardBack() -> impl IntoView {
    let active_project = create_rw_signal::<Option<usize>>(None);
    let project_list = PROJECTS.into_iter().map(|p| {
        view! {
            <div class="cursor-pointer py-1 transition duration-100 hover:scale-105 origin-center" on:click=move |_| {
                active_project.set(Some(p.index));
            }>
                {p.name}
            </div>
        }
    }).collect_view();
    let project_details = PROJECTS.into_iter().map(|p| {
        view! {
            <div class=move || format!("absolute top-15% m-16 h-full text-center transition origin-center {}", match active_project.get() {
                Some(id) => if p.index == id {"scale-100"} else {"scale-0"},
                None => {"scale-0"},
            })>
                <ProjectEntry project=p />
            </div>
        }
    }).collect_view();

    view! {
        <div class="flex flex-row h-full">
            <div class="w-[40%] pt-16 pl-16 pb-16 h-full">
                <h1 class="text-2xl">
                    Projects:
                </h1>
                <ul class="flex flex-col h-full m-2 overflow-hidden overflow-y-scroll font-bold text-lg pl-4 no-scrollbar">
                    {project_list}
                </ul>
            </div>
            <div class="w-full h-full">
                // Exit button
                <div class=move || format!("absolute pt-5 right-5 transition {}",
                    if active_project.get().is_some() {"cursor-pointer opacity-100"} else {"opacity-0"}
                )>
                    <div class="transition duration-100 hover:scale-125" on:click=move |_| {active_project.set(None)}>
                        <Icon icon=Icon::from(BiIcon::BiXRegular) width="35px" height="35px" style="color: fdf6e3;" />
                    </div>
                </div>
                {project_details}
            </div>
        </div>
    }
}

#[component]
fn ProjectEntry(project: Project) -> impl IntoView {
    view! {
        <h1 class="flex text-3xl flex-row justify-center border-b-2 border-slate-500">
                <a href=project.link class="px-1.5 transition duration-100 hover:scale-125">
                    <Icon icon=Icon::from(AiIcon::AiGithubFilled) width="25px" height="25px" style="color: fdf6e3;"/>
                </a>
                {project.name}
                <p class="text-sm">
                    {if !project.completed {"(in progress)"} else {""}}
                </p>
        </h1>
        <p class="text-lg pt-5">
            {project.description}
        </p>
    }
}
