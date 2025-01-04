#[cfg(feature = "BGLib+DotnetExtension+CommandLine+ArgumentType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ArgumentType {
    #[default]
    Boolean = 0i32,
    Integer = 3i32,
    IntegerOptional = 4i32,
    String = 1i32,
    StringOptional = 2i32,
}
#[cfg(feature = "BGLib+DotnetExtension+CommandLine+ArgumentType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::BGLib::DotnetExtension::CommandLine::ArgumentType =>
    "BGLib.DotnetExtension.CommandLine"."ArgumentType"
);
