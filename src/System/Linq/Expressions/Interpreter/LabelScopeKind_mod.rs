#[cfg(feature = "System+Linq+Expressions+Interpreter+LabelScopeKind")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LabelScopeKind {
    #[default]
    Block = 1i32,
    Catch = 5i32,
    Expression = 8i32,
    Filter = 7i32,
    Finally = 6i32,
    Lambda = 3i32,
    Statement = 0i32,
    Switch = 2i32,
    Try = 4i32,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LabelScopeKind")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::LabelScopeKind =>
    "System.Linq.Expressions.Interpreter"."LabelScopeKind"
);
