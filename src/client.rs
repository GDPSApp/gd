use bon::Builder;

use crate::credentials::Credentials;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthenticatedState<'a> {
    pub credentials: Credentials<'a>,
}

impl<'a> AuthenticatedState<'a> {
    pub fn new(credentials: Credentials<'a>) -> Self {
        Self { credentials }
    }
}

pub trait State: private::Sealed {}

mod private {
    pub trait Sealed {}
}

impl private::Sealed for SimpleState {}
impl private::Sealed for AuthenticatedState<'_> {}

impl State for SimpleState {}
impl State for AuthenticatedState<'_> {}

#[derive(Debug, Clone, PartialEq, Eq, Default, Builder)]
pub struct Client<S: State> {
    pub state: S,
}

pub type Simple = Client<SimpleState>;
pub type Authenticated<'c> = Client<AuthenticatedState<'c>>;

impl Simple {
    pub fn new() -> Self {
        Self::builder().state(SimpleState).build()
    }
}

impl<'a> Authenticated<'a> {
    pub fn new(credentials: Credentials<'a>) -> Self {
        Self::builder()
            .state(AuthenticatedState::new(credentials))
            .build()
    }
}

impl Simple {
    pub async fn login(self) -> Authenticated<'static> {
        todo!()
    }
}

impl Authenticated<'_> {
    pub async fn logout(self) -> Simple {
        todo!()
    }
}
