pub trait PromptChoice<T> {
    fn get_result(&self) -> NavigationResult<T>;
}

pub enum NavigationResult<T> {
    AskAgain,
    Ok(T),
}
