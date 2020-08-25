use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub count: usize,
}

pub struct TitleBar {
    props: Props,
}

impl Component for TitleBar {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props != self.props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <>
                <div class="soft-title">{ "Dockerps-web" }</div>
                <div class="container-count">{ format!("{} containers", self.props.count) }</div>
            </>
        }
    }
}
