use std::io::Result;

use crate::{
    cmis::{
        Object,
        def::{
            CMIS_QUERY, CMIS_SELECTOR, CMIS_SELECTOR_QUERY, CMIS_TYPE_ID_DOCUMENT,
            CMIS_TYPE_ID_FOLDER, PageReader, ResultsResponse,
        },
    },
    repl::Session,
};

pub struct QuerySet {
    queries: Vec<String>,
}

pub struct QueryBuilder<'a> {
    from: Vec<&'a str>,
    columns: &'a str,
    condition: &'a str,
}

impl QuerySet {
    pub fn new() -> Self {
        Self { queries: vec![] }
    }

    pub fn add(mut self, query: String) -> Self {
        self.queries.push(query);
        self
    }

    pub fn call(self, session: &Session) -> Result<Vec<Object>> {
        self.queries
            .iter()
            .fold(PageReader::new(), |reader, query| {
                reader.read_paged(
                    session,
                    |request| {
                        request
                            .query(CMIS_SELECTOR, CMIS_SELECTOR_QUERY)
                            .query(CMIS_QUERY, query)
                    },
                    |response: ResultsResponse| response.results,
                )
            })
            .collect(|x| x)
    }
}

#[allow(dead_code)]
impl<'a> QueryBuilder<'a> {
    pub fn from_documents() -> Self {
        Self {
            from: vec![CMIS_TYPE_ID_DOCUMENT],
            columns: "*",
            condition: "",
        }
    }

    pub fn from_folders() -> Self {
        Self {
            from: vec![CMIS_TYPE_ID_FOLDER],
            columns: "*",
            condition: "",
        }
    }

    pub fn from_objects() -> Self {
        Self {
            from: vec![CMIS_TYPE_ID_DOCUMENT, CMIS_TYPE_ID_FOLDER],
            columns: "*",
            condition: "",
        }
    }

    pub fn select(mut self, columns: &'a str) -> Self {
        self.columns = columns;
        self
    }

    pub fn condition(mut self, condition: &'a str) -> Self {
        self.condition = condition;
        self
    }

    pub fn read_paged(self, session: &Session) -> Result<Vec<Object>> {
        let where_statement = match self.condition.is_empty() {
            true => "",
            false => "where",
        };
        self.from
            .iter()
            .map(|from| {
                format!(
                    "select {} from {} {} {}",
                    self.columns, from, where_statement, self.condition
                )
            })
            .fold(QuerySet::new(), |queries, query| queries.add(query))
            .call(session)
    }
}

#[allow(dead_code)]
pub mod ops {
    pub fn id(id: &str) -> String {
        format!("cmis:objectId='{id}'")
    }

    pub fn name(folder: &str) -> String {
        format!("cmis:name='{folder}'")
    }

    pub fn in_folder(folder: &str) -> String {
        format!("in_folder('{folder}')")
    }

    pub fn not(cond: &str) -> String {
        format!("not ({cond})")
    }

    pub fn and(left: &str, right: &str) -> String {
        format!("({left}) and ({right})")
    }

    pub fn or(left: &str, right: &str) -> String {
        format!("({left}) or ({right})")
    }
}
