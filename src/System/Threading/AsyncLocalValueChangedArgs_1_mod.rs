#[cfg(feature = "System+Threading+AsyncLocalValueChangedArgs_1")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct AsyncLocalValueChangedArgs_1<T: quest_hook::libil2cpp::Type> {
    pub _PreviousValue_k__BackingField: T,
    pub _CurrentValue_k__BackingField: T,
    pub _ThreadContextChanged_k__BackingField: bool,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "System+Threading+AsyncLocalValueChangedArgs_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::AsyncLocalValueChangedArgs_1
    < T > => "System.Threading"."AsyncLocalValueChangedArgs`1<T>" < T >
);
#[cfg(feature = "System+Threading+AsyncLocalValueChangedArgs_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::System::Threading::AsyncLocalValueChangedArgs_1<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Threading+AsyncLocalValueChangedArgs_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::System::Threading::AsyncLocalValueChangedArgs_1<T> {
    pub fn _ctor(
        &mut self,
        previousValue: T,
        currentValue: T,
        contextChanged: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (previousValue, currentValue, contextChanged),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CurrentValue(&mut self) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_CurrentValue",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
