use web_sys::DragEvent;
use yew::prelude::*;
use yew::services::reader::File;

pub struct FileDropArea {
    props: Props,
    link: ComponentLink<Self>,
    input_ref: NodeRef,
}

pub enum Msg {
    Files(Vec<File>),
    Nop,
}

#[derive(Clone, Properties)]
pub struct Props {
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_else(Callback::noop)]
    pub onchange: Callback<Vec<File>>,
    #[prop_or_default]
    pub children: Children,
}

impl Component for FileDropArea {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        FileDropArea {
            props,
            link,
            input_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Files(v) => {
                self.props.onchange.emit(v);
                return true;
            }
            Msg::Nop => {}
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let ondragover = self.link.callback(|e: DragEvent| {
            e.prevent_default();
            Msg::Nop
        });
        let ondrop = self.link.callback(|e: DragEvent| {
            e.prevent_default();
            if let Some(ft) = e.data_transfer() {
                return Msg::Files(
                    js_sys::try_iter(&ft.files().unwrap())
                        .unwrap()
                        .unwrap()
                        .map(|v| File::from(v.unwrap()))
                        .collect(),
                );
            }

            Msg::Nop
        });
        let onchange = self.link.callback(|e| {
            let res = match e {
                ChangeData::Files(f) => js_sys::try_iter(&f)
                    .unwrap()
                    .unwrap()
                    .map(|v| File::from(v.unwrap()))
                    .collect(),
                _ => unreachable!(),
            };
            Msg::Files(res)
        });
        html! {
            <div class="bow-drop-area"
                ondrop=ondrop
                ondragover=ondragover
                disabled=self.props.disabled>

                <input type="file" hidden=true
                ref=self.input_ref.clone(),
                multiple=true
                onchange=onchange></input>

                { self.props.children.render() }
            </div>
        }
    }
}