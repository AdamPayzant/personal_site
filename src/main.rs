use leptos::*;

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


// This is gross, I'd like a better way of doing this
const CARD_BORDER: &str = r#"
    border-radius: 8px;
    background: 
        radial-gradient(circle at 100% 100%, #334454 0, #334454 3px, transparent 3px) 0% 0%/8px 8px no-repeat,
        radial-gradient(circle at 0 100%, #334454 0, #334454 3px, transparent 3px) 100% 0%/8px 8px no-repeat,
        radial-gradient(circle at 100% 0, #334454 0, #334454 3px, transparent 3px) 0% 100%/8px 8px no-repeat,
        radial-gradient(circle at 0 0, #334454 0, #334454 3px, transparent 3px) 100% 100%/8px 8px no-repeat,
        linear-gradient(#334454, #334454) 50% 50%/calc(100% - 10px) calc(100% - 16px) no-repeat,
        linear-gradient(#334454, #334454) 50% 50%/calc(100% - 16px) calc(100% - 10px) no-repeat,
        linear-gradient(147deg, transparent 25%, #581c87 25%, #9A2157 50%, transparent 50%, transparent 75%, #dc2626 75%);
"#;

#[component]
fn Card() -> impl IntoView {
    let flip_state = create_rw_signal(false);
    view! {
        // <div class="flex aspect-7/4 max-w-screen-lg flex-col mx-auto rounded-lg
        //             bg-gradient-to-br from-purple-900 to-red-600
        //             text-white">
        // </div>
        <div class="flex aspect-7/4 max-w-screen-lg flex-col mx-auto text-white" style=CARD_BORDER>
            <CardFront/>
        </div>
    }
}

#[component]
fn CardFront() -> impl IntoView {
    view! {
        Adam Payzant
    }
}

#[component]
fn CardBack() -> impl IntoView {
    view! {

    }
}

