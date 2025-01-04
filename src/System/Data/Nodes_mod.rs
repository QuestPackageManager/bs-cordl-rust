#[cfg(feature = "System+Data+Nodes")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Nodes {
    #[default]
    Binop = 3i32,
    BinopSpec = 4i32,
    Call = 6i32,
    Const = 7i32,
    Conv = 10i32,
    Name = 8i32,
    Noop = 0i32,
    Paren = 9i32,
    Unop = 1i32,
    UnopSpec = 2i32,
    Zop = 5i32,
}
#[cfg(feature = "System+Data+Nodes")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Data::Nodes => "System.Data"."Nodes"
);
