use uuid::Uuid;

pub fn generate()
    -> String
{
    Uuid::new_v4().to_string()
}

pub trait HasKey
{
    fn key(&self) -> &str;
}