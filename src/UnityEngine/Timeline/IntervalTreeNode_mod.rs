#[cfg(feature = "UnityEngine+Timeline+IntervalTreeNode")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct IntervalTreeNode {
    pub center: i64,
    pub first: i32,
    pub last: i32,
    pub left: i32,
    pub right: i32,
}
#[cfg(feature = "UnityEngine+Timeline+IntervalTreeNode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::IntervalTreeNode =>
    "UnityEngine.Timeline"."IntervalTreeNode"
);
#[cfg(feature = "UnityEngine+Timeline+IntervalTreeNode")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Timeline::IntervalTreeNode {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Timeline+IntervalTreeNode")]
impl crate::UnityEngine::Timeline::IntervalTreeNode {}
