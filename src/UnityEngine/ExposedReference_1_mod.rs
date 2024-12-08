#[cfg(feature = "UnityEngine+ExposedReference_1")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ExposedReference_1<T: quest_hook::libil2cpp::Type> {
    pub exposedName: crate::UnityEngine::PropertyName,
    pub defaultValue: *mut crate::UnityEngine::Object,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "UnityEngine+ExposedReference_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ExposedReference_1 < T > =>
    "UnityEngine"."ExposedReference`1<T>" < T >
);
#[cfg(feature = "UnityEngine+ExposedReference_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ExposedReference_1<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ExposedReference_1")]
impl<T: quest_hook::libil2cpp::Type> crate::UnityEngine::ExposedReference_1<T> {
    pub fn Resolve(
        &mut self,
        resolver: *mut crate::UnityEngine::IExposedPropertyTable,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Resolve",
            (resolver),
        )?;
        Ok(__cordl_ret)
    }
}
