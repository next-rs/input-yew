use yew::prelude::*;

use crate::components::login_form_one::LoginFormOne;

#[function_component(LoginPageOne)]
pub fn login_page_one() -> Html {
    html! {
        <LoginFormOne />
    }
}
