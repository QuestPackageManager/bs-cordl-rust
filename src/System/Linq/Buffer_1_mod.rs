#[cfg(feature = "System+Linq+Buffer_1")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Buffer_1<TElement: quest_hook::libil2cpp::Type> {
    pub items: *mut quest_hook::libil2cpp::Il2CppArray<TElement>,
    pub count: i32,
    __cordl_phantom_TElement: std::marker::PhantomData<TElement>,
}
#[cfg(feature = "System+Linq+Buffer_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Buffer_1 < TElement > =>
    "System.Linq"."Buffer`1<TElement>" < TElement >
);
#[cfg(feature = "System+Linq+Buffer_1")]
unsafe impl<TElement: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::System::Linq::Buffer_1<TElement> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Linq+Buffer_1")]
impl<TElement: quest_hook::libil2cpp::Type> crate::System::Linq::Buffer_1<TElement> {
    pub fn _ctor(
        &mut self,
        source: *mut crate::System::Collections::Generic::IEnumerable_1<TElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (source),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToArray(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<TElement>>
    where
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<TElement> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToArray",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
