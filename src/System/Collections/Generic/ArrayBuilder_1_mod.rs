#[cfg(feature = "System+Collections+Generic+ArrayBuilder_1")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ArrayBuilder_1<T: quest_hook::libil2cpp::Type> {
    pub _array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    pub _count: i32,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "System+Collections+Generic+ArrayBuilder_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Collections::Generic::ArrayBuilder_1 < T
    > => "System.Collections.Generic"."ArrayBuilder`1<T>" < T >
);
#[cfg(feature = "System+Collections+Generic+ArrayBuilder_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::System::Collections::Generic::ArrayBuilder_1<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Collections+Generic+ArrayBuilder_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::System::Collections::Generic::ArrayBuilder_1<T> {
    pub fn ToArray(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToArray", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UncheckedAdd(
        &mut self,
        item: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "UncheckedAdd",
            (item),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        capacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (capacity),
        )?;
        Ok(__cordl_ret.into())
    }
}
