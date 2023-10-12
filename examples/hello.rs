use viewbuilder::prelude::*;

#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        view { "Hello World!" }
    })
}

fn main() {
    viewbuilder::run(App)
}