use crate::fabric::prelude::*;

pub struct Navbar {
    link: ComponentLink<Self>,
    props: Props,
}

pub enum Msg {
    Clicked,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub onclick: Callback<()>,
    pub children: Children,
}

impl Component for Navbar {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Navbar { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                self.props.onclick.emit(());
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.onclick = props.onclick;
        true
    }

    fn view(&self) -> Html {
        html! {
            <button class="navbar"
                onclick=self.link.callback(|_| Msg::Clicked)>
                { self.props.children.render() }

            </button>
        }
    }
}
