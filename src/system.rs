use num_enum::{TryFromPrimitive, TryFromPrimitiveError};

pub trait Device {
	type Method: TryFrom<usize>;

	fn switch(&self, state: bool);
	fn method(&self, _method: Self::Method, _a: isize, _b: isize) -> Result {
		Err(Error::NoFunction)
	}
}

#[derive(TryFromPrimitive)]
#[repr(isize)]
/// List of common system errors
pub enum Error {
	/// Can mean anything. Used by lazy people.
	GeneralError = -1,

	/// Specified function doesn't exist. Usually returned by [`syscall`]
	/// if wrong call is specified but also can be returned
	/// by underlying system function
	NoFunction = -2,

	/// Specified device doesn't exist
	NoDevice = -3,

	/// Bad parameter was specified
	BadValue = -4
}

impl<T: TryFromPrimitive> From<TryFromPrimitiveError<T>> for Error {
	fn from(_: TryFromPrimitiveError<T>) -> Self {
		Error::BadValue
	}
}

pub type Result = core::result::Result<isize, Error>;
