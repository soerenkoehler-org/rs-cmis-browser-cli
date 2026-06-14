use std::io::Result;

use indicatif::{ProgressBar, ProgressStyle};
use serde::Deserialize;
use ureq::{RequestBuilder, typestate::WithoutBody};

use crate::{
    cmis::def::{CMIS_MAX_ITEMS, CMIS_SKIP_COUNT, CMIS_SUCCINCT, PageReader},
    http::read_object,
    repl::Session,
};

impl<T> PageReader<T> {
    pub fn new() -> PageReader<T> {
        Self {
            progress: ProgressBar::new_spinner()
                .with_style(ProgressStyle::with_template("{elapsed_precise} {pos}").unwrap()),
            error: None,
            objects: vec![],
        }
    }

    pub fn read_paged<U>(
        mut self,
        session: &Session,
        set_params: impl Fn(RequestBuilder<WithoutBody>) -> RequestBuilder<WithoutBody>,
        extract_objects: impl Fn(U) -> Vec<T>,
    ) -> PageReader<T>
    where
        U: for<'de> Deserialize<'de>,
    {
        // FIXME as macro??? error handling here is different (no Result<> return type) 🤔
        let Some(connection) = session.config.connections.get(&session.connection_name) else {
            return self;
            // return io_error!(format!(
            //     "No config for connection {}",
            //     session.connection_name
            // ));
        };
        let mut skip_count = 0;
        loop {
            if self.error.is_none() {
                match session
                    .http_session
                    .get_request(connection, &session.repository.root_folder_url)
                {
                    Err(err) => self.error = Some(err),
                    Ok(request) => {
                        let request = set_params(
                            request
                                .query(CMIS_SKIP_COUNT, format!("{}", skip_count))
                                .query(
                                    CMIS_MAX_ITEMS,
                                    format!("{}", session.config.global.paging_size),
                                )
                                .query(CMIS_SUCCINCT, "true"),
                        );

                        match read_object(request) {
                            Err(err) => self.error = Some(err),
                            Ok(partial_result) => {
                                let new_objects = extract_objects(partial_result);
                                let new_objects_count = new_objects.len();

                                self.objects.push(new_objects);
                                skip_count += new_objects_count;

                                if new_objects_count < session.config.global.paging_size {
                                    // The flag "hasMoreItems" may not be accurate, therefore
                                    // our assumption that the last page contains less elements
                                    // than "maxItems".
                                    break;
                                } else if let Ok(progress_update) = u64::try_from(new_objects_count)
                                {
                                    self.progress.inc(progress_update);
                                }
                            }
                        }
                    }
                }
            }
        }
        self
    }

    pub fn collect<U>(self, conversion: impl Fn(T) -> U) -> Result<Vec<U>> {
        self.progress.finish_and_clear();
        match self.error {
            Some(err) => Err(err),
            _ => Ok(self.objects.into_iter().flatten().map(conversion).collect()),
        }
    }
}
