


// use std::io;
// use thiserror::Error;
//
// #[derive(Error, Debug)]
// pub enum DataStoreError {
//     #[error("data store disconnected")]
//     Disconnect(#[from] io::Error),
//     #[error("the data for key `{0}` is not available")]
//     Redaction(String),
//     #[error("invalid header (expected {expected:?}, found {found:?})")]
//     InvalidHeader {
//         expected: String,
//         found: String,
//     },
//     #[error("unknown data store error")]
//     Unknown,
// }
//
// #[derive(Error, Debug)]
// pub enum Error2 {
//     #[error("invalid rdo_lookahead_frames {0} (expected < {})", i32::MAX)]
//     InvalidLookahead(u32),
// }
//
//
//
// pub fn sss() -> Box<dyn std::error::Error> {
//     let x: DataStoreError = DataStoreError::Unknown;
//     let y: Error2 = Error2::InvalidLookahead(44);
//     y
// }

// use std::fmt::{Debug, Display, Formatter};
// use std::io;
//
// pub trait FFXIVError<T: Display + Clone>: std::error::Error + ErrorKind<T> {
//
// }
//
// pub struct Error<T: Display> {
//     pub kind: T,
// }
//
// pub trait ErrorKind<T: Display> {
//     fn kind(&self) -> T;
// }
//
// impl <T: Display + Clone> ErrorKind<T> for Error<T> {
//     fn kind(&self) -> T {
//         self.kind.clone()
//     }
// }
//
// impl <T: Display> Error<T> {
//     pub fn new(kind: T) -> Error<T> {
//         Error {
//             kind
//         }
//     }
// }
//
// impl <T: Display> Debug for Error<T> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.kind.to_string())
//     }
// }
//
// impl <T: Display> Display for Error<T> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{{ file: {}, line: {}, msg: {} }}", file!(), line!(), self.kind.to_string())
//     }
// }
//
// impl <T: Display> std::error::Error for Error<T> {
//
// }
//
// //==================================================================================================
//
// #[derive(Clone)]
// pub enum PathError {
//     Invalid(String)
// }
//
// // impl PathError {
// //     fn invalid(value: &str) -> Error<> {
// //         todo!()
// //     }
// // }
//
// impl std::error::Error for PathError {}
//
// impl Debug for PathError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{{ file: {}, line: {}, msg: {} }}", file!(), line!(), self)
//     }
// }
//
// impl Display for PathError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", match self {
//             PathError::Invalid(msg) => format!("Invalid: {}", msg)
//         })
//     }
// }
//
//
//
// // pub trait From<T>: Sized {
// //     fn from(_: T) -> Self;
// // }
// //
// //
// //
// // impl <T: Display + From<String>> From<io::Error> for Error<T> {
// //     fn from(err: io::Error) -> Self {
// //         Error {
// //             kind: T::from(err.to_string())
// //         }
// //     }
// // }
//
// // impl <T> FromResidual<String> for Error<T> {
// //
// // }
// //
// // impl <T: Display> std::error::Error for Error<T> {
// //
// // }
// //
