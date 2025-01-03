#[cfg(feature = "System+ByReference_1")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ByReference_1<T: quest_hook::libil2cpp::Type> {
    pub _value: crate::System::IntPtr,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "System+ByReference_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::ByReference_1 < T > => "System"
    ."ByReference`1<T>" < T >
);
#[cfg(feature = "System+ByReference_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::System::ByReference_1<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+ByReference_1")]
impl<T: quest_hook::libil2cpp::Type> crate::System::ByReference_1<T> {
    pub fn _ctor(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::ByRefMut<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<T> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Value",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
