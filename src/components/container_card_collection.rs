use std::rc::Rc;
use yew::prelude::*;

use crate::components::container_card::ContainerCard;
use crate::models::container::Container;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub containers: Rc<Vec<Container>>,
}

pub struct ContainerCardCollection {
    props: Props,
}

impl Component for ContainerCardCollection {
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
                {
                    for self.props.containers.iter().map(|container| html! {
                        <ContainerCard container={ container.clone() } />
                    })
                }
            </>
        }
    }
}
