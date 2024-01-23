use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::contact_page_one::ContactPageOne;
use crate::pages::error::Error;
use crate::pages::login_page_one::LoginPageOne;
use crate::pages::login_page_three::LoginPageThree;
use crate::pages::login_page_two::LoginPageTwo;
use crate::pages::multi_step_page_one::MultiStepPageOne;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/error")]
    Error,
    #[at("/login/1")]
    LoginPageOne,
    #[at("/login/2")]
    LoginPageTwo,
    #[at("/login/3")]
    LoginPageThree,
    #[at("/contact/1")]
    ContactPageOne,
    #[at("/multi-step/1")]
    MultiStepPageOne,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::LoginPageOne => html! { <LoginPageOne /> },
        Route::LoginPageTwo => html! { <LoginPageTwo /> },
        Route::LoginPageThree => html! { <LoginPageThree /> },
        Route::ContactPageOne => html! { <ContactPageOne /> },
        Route::MultiStepPageOne => html! { <MultiStepPageOne /> },
        Route::Error => html! { <Error /> },
    }
}
