use std::fmt;

pub enum ResponseText {
    LoggedIn,
    SingIn,
    LoggedOut
}

impl fmt::Display for ResponseText {
    fn fmt(&self,f : &mut fmt::Formatter) -> fmt::Result{
        match self {
            ResponseText::LoggedIn => write!(f,"You are logged in."),
            ResponseText::SingIn=> write!(f,"You are singed in."),
            ResponseText::LoggedOut=> write!(f,"You are logged out")
        }
    }
}