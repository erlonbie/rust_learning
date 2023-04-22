// // use yew::prelude::*;
// use yew::{function_component, html, Html, Properties};
//
// #[derive(Properties, PartialEq)]
// pub struct Props {
//     #[prop_or_default]
//     pub is_loading: bool,
// }
//
// #[function_component]
// fn HelloWorld(props: &Props) -> Html {
//     if props.is_loading {
//         html! { "loading"}
//     } else {
//         html! { "Hello World" }
//     }
// }
//
// #[function_component]
// fn App() -> Html {
//     html! {
//         <>
//         <div>
//             <HelloWorld is_loading={true}/>
//             // <img src="img/cat_leaves.png" alt="Girl in a jacket 2" width="400" height="300" />
//         </div>
//         </>
//     }
// }
//
// fn main() {
//     yew::Renderer::<App>::new().render();
// }

use yew::{function_component, html, props, virtual_dom::AttrValue, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(AttrValue::from("Erlon"))]
    pub name: AttrValue,
}

#[function_component]
fn HelloWorld(props: &Props) -> Html {
    html! {
        <>
        {"Hello world, "}{props.name.clone()}
        <img src="img/cat_leaves.png" alt="Girl in a jacket 2" width="400" height="300" />
        </>
    }
}

#[function_component]
fn App() -> Html {
    let pre_made_props = props! {
        Props {} // Notice we did not need to specify name prop
    };
    html! {
        <HelloWorld ..pre_made_props />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
