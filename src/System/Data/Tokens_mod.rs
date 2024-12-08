#[cfg(feature = "System+Data+Tokens")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tokens {
    BinaryConst = 5i32,
    BinaryOp = 13i32,
    Child = 14i32,
    Date = 7i32,
    Decimal = 3i32,
    Dot = 16i32,
    EOS = 18i32,
    Float = 4i32,
    LeftParen = 9i32,
    ListSeparator = 8i32,
    Name = 1i32,
    None = 0i32,
    Numeric = 2i32,
    Parent = 15i32,
    RightParen = 10i32,
    StringConst = 6i32,
    UnaryOp = 12i32,
    Unknown = 17i32,
    ZeroOp = 11i32,
}
#[cfg(feature = "System+Data+Tokens")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Data::Tokens => "System.Data"."Tokens"
);
