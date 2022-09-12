use serde::Serialize;

#[derive(Serialize,Clone)]
pub struct ErrorDto
{
    pub message: String,
}