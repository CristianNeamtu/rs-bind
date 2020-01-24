
#[derive(Debug)]
pub struct BindError {
    message: String
}

#[allow(dead_code)]
impl BindError {
    pub(crate) fn new(message: String) -> Self {
        BindError { message }
    }

    pub fn get_message(self) -> String {
        self.message
    }
}
