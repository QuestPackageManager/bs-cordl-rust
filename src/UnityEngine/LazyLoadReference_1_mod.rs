#[cfg(feature = "UnityEngine+LazyLoadReference_1")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct LazyLoadReference_1<T: quest_hook::libil2cpp::Type> {
    pub m_InstanceID: i32,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "UnityEngine+LazyLoadReference_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::LazyLoadReference_1 < T > =>
    "UnityEngine"."LazyLoadReference`1<T>" < T >
);
#[cfg(feature = "UnityEngine+LazyLoadReference_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::LazyLoadReference_1<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+LazyLoadReference_1")]
impl<T: quest_hook::libil2cpp::Type> crate::UnityEngine::LazyLoadReference_1<T> {}
