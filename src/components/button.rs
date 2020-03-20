use web_sys::MouseEvent;
use yew::html;
use yew::callback::Callback;
use yew::html::{Html, ComponentLink, ShouldRender, Renderable, Component, Children};

pub struct Button {
    props: Props,
}

pub enum Msg {
    Clicked,
}

#[derive(Clone, Properties)]
pub struct Props {
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub disabled: bool,
}

impl Component for Button {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Button { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.onclick = props.onclick;
        true
    }

    fn view(&self) -> Html {
        html! {
            <button
                disabled=self.props.disabled
                class="bow-button"
                onclick=self.props.onclick.reform(|e| e)>
                { self.props.children.render() }

            </button>
        }
    }
}