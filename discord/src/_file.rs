use std::path::PathBuf;
use std::fs;

use futures::future::OrElse;
use serde::de;

#[derive(Debug, Clone)]
pub struct File<'a> {
    pub fp: Option<PathBuf>,
    pub filename: Option<&'a str>,
    pub description: Option<&'a str>,
    pub spoiler: Option<bool>,
    pub uri: Option<&'a str>,
}

impl <'a>File<'a> {
    pub fn new(
        uri: Option<&'a str>,
        fp: Option<PathBuf>,
        filename: Option<&'a str>,
        description: Option<&'a str>,
        spoiler: Option<bool>,
    ) -> Result<Self, &'static str> {
        if uri.is_none() && fp.is_none() {
            return Err("Either uri or fp must be given");
        }

        Ok(Self {
            fp,
            filename,
            description,
            spoiler,
            uri,
        })
    }
}