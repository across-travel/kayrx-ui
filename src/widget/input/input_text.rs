use crate::fabric::prelude::*;

pub struct InputText {
    props: Props,
    node_ref: NodeRef,
}

#[derive(Clone, Properties)]
pub struct Props {
    #[prop_or_else(Callback::noop)]
    pub oninput: Callback<InputData>,
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub disabled: bool,
}

impl Component for InputText {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        InputText {
            props,
            node_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="bow-input-text">
                <input ref=self.node_ref.clone()
                    disabled=self.props.disabled
                    value=&self.props.value
                    oninput=self.props.oninput.reform(|d| d)>
                </input>
            </div>
        }
    }
}