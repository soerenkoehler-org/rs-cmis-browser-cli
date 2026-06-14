macro_rules! io_error {
    ($err:expr) => {
        Err(std::io::Error::other($err.to_string()))
    };
}

// TODO macro for io_error!(format!(...)) chain

macro_rules! not_implemented {
    () => {
        io_error!("not implemented")
    };
}

macro_rules! get_session {
    ($self:expr) => {
        match $self.session.as_mut() {
            Some(session) => Ok(session),
            None => io_error!("not connected"),
        }?
    };
}

// FIXME macro for get connection-config from session