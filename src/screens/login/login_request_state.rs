#[derive(Default, PartialEq, Clone)]
pub enum LoginRequestState {
    #[default]
    Idle,
    Pending,
    Fulfilled,
    Rejected,
}

impl From<u8> for LoginRequestState {
    fn from(val: u8) -> Self {
        match val {
            1 => LoginRequestState::Pending,
            2 => LoginRequestState::Fulfilled,
            3 => LoginRequestState::Rejected,
            _ => LoginRequestState::Idle,
        }
    }
}

impl From<LoginRequestState> for u8 {
    fn from(val: LoginRequestState) -> Self {
        match val {
            LoginRequestState::Idle => 0,
            LoginRequestState::Pending => 1,
            LoginRequestState::Fulfilled => 2,
            LoginRequestState::Rejected => 3,
        }
    }
}
