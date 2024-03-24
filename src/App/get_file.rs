use std::env;
pub fn get_file() -> String
{
    let args: Vec<String> = env::args().collect();
    args[1].clone()
}