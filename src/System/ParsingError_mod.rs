#[cfg(feature = "System+ParsingError")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParsingError {
    BadAuthority = 3i32,
    BadAuthorityTerminator = 11i32,
    BadFormat = 1i32,
    BadHostName = 8i32,
    BadPort = 10i32,
    BadScheme = 2i32,
    CannotCreateRelative = 12i32,
    EmptyUriString = 4i32,
    MustRootedPath = 7i32,
    NonEmptyHost = 9i32,
    None = 0i32,
    SchemeLimit = 5i32,
    SizeLimit = 6i32,
}
#[cfg(feature = "System+ParsingError")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::ParsingError => "System"."ParsingError"
);
