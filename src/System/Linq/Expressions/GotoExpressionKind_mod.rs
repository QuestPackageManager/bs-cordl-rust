#[cfg(feature = "System+Linq+Expressions+GotoExpressionKind")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GotoExpressionKind {
    #[default]
    Break = 2i32,
    Continue = 3i32,
    Goto = 0i32,
    Return = 1i32,
}
#[cfg(feature = "System+Linq+Expressions+GotoExpressionKind")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Expressions::GotoExpressionKind =>
    "System.Linq.Expressions"."GotoExpressionKind"
);
