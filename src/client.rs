// use std::time::Duration;

use bon::Builder;

use crate::{
    auth::{Auth, Credentials},
    internals::session::Session,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthenticatedState<'a> {
    pub auth: Auth<'a>,
}

impl<'a> AuthenticatedState<'a> {
    pub fn new(auth: Auth<'a>) -> Self {
        Self { auth }
    }
}

pub trait State: sealed::Sealed {}

mod sealed {
    pub trait Sealed {}
}

impl sealed::Sealed for SimpleState {}
impl sealed::Sealed for AuthenticatedState<'_> {}

impl State for SimpleState {}
impl State for AuthenticatedState<'_> {}

/// Represents clients generic over their *state*.
#[derive(Debug, Clone, Default, Builder)]
pub struct Client<S: State = SimpleState> {
    /// The session to use.
    pub session: Session,
    /// The state of the client, either *simple* or *authenticated*.
    pub state: S,
}

impl<S: State> Client<S> {
    // pub async fn ping(&self) -> Result<Duration, http::Error> {
    //     self.session.ping().await
    // }
}

/// Represents *simple* clients.
pub type Simple = Client<SimpleState>;

/// Represents *authenticated* clients.
pub type Authenticated<'a> = Client<AuthenticatedState<'a>>;

impl Simple {
    pub fn new(session: Session) -> Self {
        Self::builder().session(session).state(SimpleState).build()
    }

    pub async fn login<'c>(&self, credentials: Credentials<'c>) {}

    pub fn with_auth(self, auth: Auth<'_>) -> Authenticated<'_> {
        Authenticated::new(self.session, auth)
    }
}

impl Default for Simple {
    fn default() -> Self {
        Self::new(Session::default())
    }
}

impl<'a> Authenticated<'a> {
    pub fn new(session: Session, auth: Auth<'a>) -> Self {
        Self::builder()
            .session(session)
            .state(AuthenticatedState::new(auth))
            .build()
    }

    pub fn into_simple(self) -> Simple {
        Simple::new(self.session)
    }
}
