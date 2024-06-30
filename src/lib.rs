use leptos::prelude::*;
use leptos_router::{components::*, StaticSegment};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "An error occurred">
                <ParentRoute path=StaticSegment("") view=Index>
                    <Route path=StaticSegment("") view=Default/>
                    <Route path=StaticSegment("home") view=Home/>
                </ParentRoute>
            </Routes>
        </Router>
    }
}

#[component]
fn Index() -> impl IntoView {
    let (show_outlet, set_show_outlet) = signal(false);
    view! {
        <div>
            <button on:click=move |_| set_show_outlet(true)>"Show outlet"</button>
        </div>

        {move || {
            if show_outlet() {
                view! { <Outlet/> }.into_any()
            } else {
                view! { <div>"No outlet"</div> }.into_any()
            }
        }}
    }
}

#[component]
fn Default() -> impl IntoView {
    view! { "defualt" }
}

#[component]
fn Home() -> impl IntoView {
    view! { "home" }
}
