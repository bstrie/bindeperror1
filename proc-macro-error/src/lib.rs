pub trait ResultExt<T> {
    fn unwrap_or_abort(self) -> T;
}

impl<T, E: Into<Diagnostic>> ResultExt<T> for Result<T, E> {
    fn unwrap_or_abort(self) -> T {
        panic!()
    }
}

pub struct Diagnostic;

impl From<syn::Error> for Diagnostic {
    fn from(_: syn::Error) -> Self {
        panic!()
    }
}
