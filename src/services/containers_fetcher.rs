use anyhow::Error;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchTask, Request, Response};
use yew::services::FetchService;

use crate::models::container::Container;

pub struct ContainersFetcher<T>
where
    T: Component,
{
    link: ComponentLink<T>,
    fetch_task: Option<FetchTask>,
}

impl<T> ContainersFetcher<T>
where
    T: Component,
{
    pub fn new(link: ComponentLink<T>) -> Self {
        Self {
            link,
            fetch_task: None,
        }
    }

    pub fn fetch(
        &mut self,
        on_completed: impl Fn(Vec<Container>) -> T::Message + 'static,
        on_failed: impl Fn() -> T::Message + 'static,
    ) -> Result<(), Error> {
        if self.is_busy() {
            return Err(Error::msg("The fetcher is busy."));
        }

        Request::builder()
            .method("GET")
            .uri("/api.json")
            .body(Nothing)
            .map_err(|error| error.into())
            .and_then(|request| {
                FetchService::fetch(
                    request,
                    self.link.callback(
                        move |response: Response<Json<Result<Vec<Container>, Error>>>| {
                            if let (meta, Json(Ok(body))) = response.into_parts() {
                                if meta.status.is_success() {
                                    return on_completed(body);
                                }
                            }

                            on_failed()
                        },
                    ),
                )
            })
            .map(|t| {
                self.fetch_task = Some(t);
            })
    }

    pub fn is_busy(&self) -> bool {
        self.fetch_task.is_some()
    }
}
