use yew::prelude::*;

use web_sys::HtmlSelectElement;

pub struct Select {
    props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
    #[prop_or_else(Callback::noop)]
    pub onchange: Callback<HtmlSelectElement>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub disabled: bool,
}

impl Component for Select {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Select { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="bow-select">
                <select onchange=self.props.onchange.reform(|v: ChangeData| {
                    match v {
                        ChangeData::Select(v) => v,
                        _ => unreachable!()
                    }
                })>
                   <option value="" selected=true disabled=true hidden=true>{"Select value"}</option>
                  { self.props.children.render() }
                </select>
            </div>
        }
    }
}