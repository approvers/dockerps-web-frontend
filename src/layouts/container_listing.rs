use std::rc::Rc;
use yew::prelude::*;
use yew::services::ConsoleService;

use crate::components::container_card_collection::ContainerCardCollection;
use crate::models::container::Container;
use crate::services::containers_fetcher::ContainersFetcher;

pub enum Msg {
    FetchContainersCompleted(Vec<Container>),
    FetchContainersFailed,
}

pub struct ContainerListing {
    _fetcher: ContainersFetcher<Self>,
    containers: Rc<Vec<Container>>,
}

impl Component for ContainerListing {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut fetcher = ContainersFetcher::new(link);

        fetcher
            .fetch(
                |containers| Msg::FetchContainersCompleted(containers),
                || Msg::FetchContainersFailed,
            )
            .expect("Failed to fetch containers.");

        Self {
            _fetcher: fetcher,
            containers: Rc::new(vec![]),
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchContainersCompleted(containers) => {
                self.containers = Rc::new(containers);
                true
            }
            Msg::FetchContainersFailed => {
                ConsoleService::error("Failed to fetch containers.");
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <ContainerCardCollection containers={ self.containers.clone() } />
            </>
        }
    }
}
