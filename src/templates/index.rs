use ory_kratos_client::models::session_authentication_method::Method::{V06LegacySession, Webauthn};
use crate::components::{counter, hello};
use perseus::{Html, RenderFnResultWithCause, SsrNode, Template};
use sycamore::prelude::{view, Scope, View};

#[perseus::make_rx(IndexPageStateRx)]
pub struct IndexPageState {
    pub greeting: String,
}

#[perseus::template_rx]
pub fn index_page<'a, G: Html>(cx: Scope<'a>, state: IndexPageStateRx<'a>) -> View<G> {
    view! { cx,
        p { (state.greeting.get()) }
        a(href = "about", id = "about-link") { "About!" }
        counter::Counter {}
        hello::Hello {}
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index")
        .build_state_fn(get_build_state)
        .template(index_page)
        .head(head)
}

pub fn auth(){
    let config = ory_kratos_client::apis::configuration::Configuration{
        base_path: "http://localhost:4000".to_string(),
        user_agent: None,
        client: Default::default(),
        basic_auth: None,
        oauth_access_token: None,
        bearer_access_token: None,
        api_key: None
    };

    let flow_body = ory_kratos_client::models::submit_self_service_registration_flow_body::SubmitSelfServiceRegistrationFlowWithWebAuthnMethodBody{};

    let ory = ory_kratos_client::apis::v0alpha2_api::submit_self_service_registration_flow(&config, "registration", flow_body, None);
}

#[perseus::head]
pub fn head(cx: Scope, _props: IndexPageState) -> View<SsrNode> {
    view! { cx,
        title { "Index Page | Perseus Example – Basic" }
    }
}

#[perseus::build_state]
pub async fn get_build_state(
    _path: String,
    _locale: String,
) -> RenderFnResultWithCause<IndexPageState> {
    Ok(IndexPageState {
        greeting: "Hello World!".to_string(),
    })
}