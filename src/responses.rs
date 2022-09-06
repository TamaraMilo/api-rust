use std::fmt;
pub enum ResponseText {

    LoggedOut
}

impl fmt::Display for ResponseText {
    fn fmt(&self,f : &mut fmt::Formatter) -> fmt::Result{
        match self {
            ResponseText::LoggedOut=> write!(f,"You are logged out")
        }
    }
}

