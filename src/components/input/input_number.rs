use web_sys::HtmlInputElement;
use yew::prelude::*;

pub struct InputNumber {
    link: ComponentLink<Self>,
    oninput: Callback<Option<f64>>,
    value: String,
    node_ref: NodeRef,
    disabled: bool,
}
pub enum Msg {
    Input(String),
}

#[derive(Clone, Properties)]
pub struct Props {
    #[prop_or_else(Callback::noop)]
    pub oninput: Callback<Option<f64>>,
    #[prop_or_default]
    pub value: Option<f64>,
    #[prop_or_default]
    pub disabled: bool,
}
impl Component for InputNumber {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        InputNumber {
            link,
            oninput: props.oninput,
            value: props.value.map_or("".into(), |v| v.to_string()),
            disabled: props.disabled,
            node_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Input(v) => {
                let old = if let Ok(fv) = self.value.parse::<f64>() {
                    Some(fv)
                } else {
                    None
                };
                let res = if v.len() == 0 {
                    self.value = "".into();
                    None
                } else if let Ok(fv) = v.parse::<f64>() {
                    self.value = v.clone().into();
                    Some(fv)
                } else {
                    old
                };

                if old != res {
                    self.oninput.emit(res);
                }
                if self.value != v {
                    if let Some(el) = self.node_ref.cast::<HtmlInputElement>() {
                        if el.value() != self.value {
                            el.set_value(&self.value);
                        }
                    }
                }
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.value = props.value.map_or("".into(), |v| v.to_string());
        self.oninput = props.oninput;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="bow-input-number">
                <input ref=self.node_ref.clone()
                    value=&self.value
                    disabled=self.disabled
                    oninput=self.link.callback(|v: InputData| Msg::Input(v.value))>
                </input>
            </div>
        }
    }
}