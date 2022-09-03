use bigint_calc::Parser;
use std::collections::HashMap;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let mut state = HashMap::new();
    let parser = Parser::new();
    let result = parser
        .parse(&mut state, "29090850183098142912 * 12384294890289031;")
        .unwrap()
        .unwrap();
    html! {
        <h1>{ result }</h1>
    }
}

fn main() {
    yew::start_app::<App>();
}
