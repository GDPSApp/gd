use std::borrow::Cow;

pub fn wrap_into_owned<T: ?Sized + ToOwned>(cow: Cow<'_, T>) -> Cow<'static, T> {
    Cow::Owned(cow.into_owned())
}
