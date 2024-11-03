use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct NoFap {
    pub goal: u32,
    pub relapsed: Vec<String>
}
