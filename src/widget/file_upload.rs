use web_sys::{DragEvent, HtmlInputElement, MouseEvent, MouseEventInit};
use crate::fabric::prelude::*;
use crate::fabric::services::reader::File;

pub struct FileUpload {
    props: Props,
    link: ComponentLink<Self>,
    input_ref: NodeRef,
    file_name: String,
}

pub enum Msg {
    Files(Vec<File>),
    Click,
    Clear,
    Nop,
}

#[derive(Clone, Properties)]
pub struct Props {
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub multiple: bool,
    #[prop_or_else(Callback::noop)]
    pub onchange: Callback<Vec<File>>,
}

impl Component for FileUpload {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        FileUpload {
            props,
            link,
            input_ref: NodeRef::default(),
            file_name: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Files(v) => {
                if let Some(f) = v.first() {
                    self.file_name = f.name();
                }
                self.props.onchange.emit(v);
                return true;
            }
            Msg::Click => {
                self.open_file_dialog();
            }
            Msg::Clear => {
                self.file_name = String::new();
                self.props.onchange.emit(Vec::new());
                return true;
            }
            Msg::Nop => {}
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        self.file_name = String::new();
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
            <div class="bow-file-upload"
                ondrop=ondrop
                ondragover=ondragover
                disabled=self.props.disabled onclick=self.link.callback(|_| Msg::Click)>

                <input type="file" hidden=true
                ref=self.input_ref.clone(),
                multiple=self.props.multiple
                onchange=onchange></input>

                { self.render_icon() }
                { self.render_caption() }
            </div>
        }
    }
}

impl FileUpload {
    fn open_file_dialog(&self) {
        if let Some(el) = self.input_ref.cast::<HtmlInputElement>() {
            let mut dict = MouseEventInit::new();
            dict.bubbles(false);
            dict.cancelable(false);
            el.dispatch_event(&MouseEvent::new_with_mouse_event_init_dict("click", &dict).unwrap())
                .unwrap();
            return;
        }

        unreachable!()
    }

    fn render_icon(&self) -> Html {
        html! {
        <svg class="bow-file-upload__icon" width="52" height="32" viewBox="0 0 52 32" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M42.1345 12.4828C41.4785 12.4828 40.8238 12.5473 40.1809 12.676C39.6961 10.2827 38.1288 8.2411 35.9282 7.13606C33.7281 6.03103 31.1369 5.98468 28.8974 7.01001C26.8707 1.48786 20.7021 -1.36331 15.1194 0.641434C9.53679 2.64618 6.65439 8.74835 8.68109 14.2701C3.79081 14.423 -0.0721116 18.4268 0.00102147 23.2658C0.0745925 28.1053 4.0575 31.9922 8.95042 32H42.1345C47.5827 32 52 27.631 52 22.2414C52 16.8518 47.5827 12.4828 42.1345 12.4828V12.4828Z" fill="#3182CE"/>
        </svg>
        }
    }

    fn render_caption(&self) -> Html {
        html! {
        <>
            <span class="bow-file-upload__caption">
            {
                if self.file_name.len() == 0 {
                    html!{
                        <>
                        {"Drop file"}{if self.props.multiple {"s"} else {""}}
                        {" here"}<br/>{"or click to select"}
                        </>
                    }
                } else { html!{&self.file_name}}
            }</span>
            { if self.file_name.len() > 0 {self.render_clear_button() } else { html!{} }}
        </>
        }
    }
    fn render_clear_button(&self) -> Html {
        html! {
            <div class="bow-file-upload__reset-button" onclick=self.link.callback(|_|Msg::Clear)>
                <svg width="12" height="12" viewBox="0 0 12 12" fill="none" xmlns="http://www.w3.org/2000/svg">
                <rect y="0.666626" width="0.942809" height="16.0278" transform="rotate(-45 0 0.666626)" fill="white"/>
                <rect width="0.942809" height="16.0278" transform="matrix(-0.707107 -0.707107 -0.707107 0.707107 12 0.666626)" fill="white"/>
                </svg>
            </div>
        }
    }
}