#[cfg(feature = "JetBrains+Annotations+AssertionConditionType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssertionConditionType {
    IS_FALSE = 1i32,
    IS_NOT_NULL = 3i32,
    IS_NULL = 2i32,
    IS_TRUE = 0i32,
}
#[cfg(feature = "JetBrains+Annotations+AssertionConditionType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::JetBrains::Annotations::AssertionConditionType
    => "JetBrains.Annotations"."AssertionConditionType"
);
