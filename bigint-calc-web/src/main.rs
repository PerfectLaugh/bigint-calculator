use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

use bigint_calc::{ParseError, Parser};
use std::{collections::HashMap, ops::Deref};

#[function_component(App)]
fn app() -> Html {
    let result = use_state(|| "NaN".to_string());
    let onkeyup = {
        let result = result.clone();
        Callback::from(move |e: KeyboardEvent| {
            if let Some(elem) = e.target_dyn_into::<HtmlTextAreaElement>() {
                let mut content = elem.value();
                content = content.trim().to_string();
                if content.is_empty() {
                    result.set("NaN".to_string());
                    return;
                }

                let parser = Parser::new();
                let mut state = HashMap::new();
                let mut final_res = None;
                for line in content.split('\n') {
                    let mut err = false;

                    for segment in line.split(';') {
                        let segment = segment.trim();
                        if segment.is_empty() {
                            continue;
                        }

                        let res = parser.parse(&mut state, segment);
                        final_res.replace(res.clone());
                        if res.is_err() {
                            err = true;
                            break;
                        }
                    }
                    if err {
                        break;
                    }
                }
                if let Some(res) = final_res {
                    match res {
                        Ok(Some(res)) => result.set(res.to_string()),
                        Ok(None) => result.set("NaN".to_string()),
                        Err(err) => match err {
                            ParseError::User { error } => result.set(error.to_string()),
                            _ => result.set("Syntax error".to_string()),
                        },
                    }
                } else {
                    result.set("NaN".to_string());
                }
            }
        })
    };
    html! {
        <>
            <p>
                { result.deref().clone() }
            </p>
            <textarea {onkeyup} />
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
