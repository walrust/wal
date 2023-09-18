use serde::Serialize;

type PropertiesHash = u64;

#[derive(Debug, Serialize, PartialEq)]
pub struct VChild
{
    hash: PropertiesHash,
}