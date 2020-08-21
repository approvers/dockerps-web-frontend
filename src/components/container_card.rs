use yew::prelude::*;

use crate::models::container::Container;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub container: Container,
}

pub struct ContainerCard {
    props: Props,
}

impl Component for ContainerCard {
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
        let container = &self.props.container;

        html! {
            <article class="info-card">
                <div class="title" data-status={ &container.status }>{ &container.name }</div>
                <div class="id">
                    <span>{ "ID: " }</span>
                    <span>{ &container.id }</span>
                </div>
                <details>
                    <summary>{ "More Info:" }</summary>
                    <table>
                        <tr>
                            <th>{ "Image:" }</th>
                            <td>{ &container.image }</td>
                        </tr>
                        <tr>
                            <th>{ "Command:" }</th>
                            <td>{ &container.command }</td>
                        </tr>
                        <tr>
                            <th>{ "Created:" }</th>
                            <td>{ &container.created }</td>
                        </tr>
                        <tr>
                            <th>{ "Status:" }</th>
                            <td>{ &container.status }</td>
                        </tr>
                        <tr>
                            <th>{ "Ports:" }</th>
                            <td>{ &container.ports }</td>
                        </tr>
                    </table>
                </details>
            </article>
        }
    }
}
