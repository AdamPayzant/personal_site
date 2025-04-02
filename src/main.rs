// use leptos::*;
use leptos::prelude::*;
use leptos_icons::*;
use icondata as i;

mod card_back;
mod card_front;

// Using https://dunks1980.com/ for inspiration
// Have it like a card, and then "flip" to projects on the back

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Card/>
    }
}

const CARD_BORDER: &str = r#"
    border-radius: 8px;
    background: 
        radial-gradient(circle at 100% 100%, #002b36 0, #002b36 3px, transparent 3px) 0% 0%/8px 8px no-repeat,
        radial-gradient(circle at 0 100%, #002b36 0, #002b36 3px, transparent 3px) 100% 0%/8px 8px no-repeat,
        radial-gradient(circle at 100% 0, #002b36 0, #002b36 3px, transparent 3px) 0% 100%/8px 8px no-repeat,
        radial-gradient(circle at 0 0, #002b36 0, #002b36 3px, transparent 3px) 100% 100%/8px 8px no-repeat,
        linear-gradient(#002b36, #002b36) 50% 50%/calc(100% - 10px) calc(100% - 16px) no-repeat,
        linear-gradient(#002b36, #002b36) 50% 50%/calc(100% - 16px) calc(100% - 10px) no-repeat,
        linear-gradient(147deg, #581c87 25%, transparent 35%, transparent 65%, #dc2626 95%);
"#;

#[component]
fn Card() -> impl IntoView {
    let flip_state = RwSignal::new(false);
    view! {
        <div class="pt-20">
            <div class=move || {
                    format!("relative aspect-7/4 max-w-screen-lg flex-col mx-auto text-[#fdf6e3]
                             perspective transform duration-1000 origin-center page {}",
                             if !flip_state.get() {""} else {"flip"})
                }
                style=CARD_BORDER
            >
                <div class="absolute w-full h-full front">
                    <card_front::CardFront/>
                </div>
                <div class="absolute w-full h-full back">
                    <card_back::CardBack/>
                </div>
                <a class="absolute bottom-4 right-5 text-center transition duration-100 hover:scale-125 cursor-pointer"
                on:click=move |_| { flip_state.set(!flip_state.get()); }>
                    <Icon icon={i::BsArrowRight} width="35px" height="35px" style="color: #fdf6e3;"/>
                </a>
            </div>
        </div>
    }
}
