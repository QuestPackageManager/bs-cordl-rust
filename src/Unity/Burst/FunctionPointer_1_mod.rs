#[cfg(feature = "Unity+Burst+FunctionPointer_1")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FunctionPointer_1<T: quest_hook::libil2cpp::Type> {
    pub _ptr: crate::System::IntPtr,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "Unity+Burst+FunctionPointer_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::FunctionPointer_1 < T > =>
    "Unity.Burst"."FunctionPointer`1<T>" < T >
);
#[cfg(feature = "Unity+Burst+FunctionPointer_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::Unity::Burst::FunctionPointer_1<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Burst+FunctionPointer_1")]
impl<T: quest_hook::libil2cpp::Type> crate::Unity::Burst::FunctionPointer_1<T> {
    pub fn CheckIsCreated(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CheckIsCreated",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Unity_Burst_IFunctionPointer_FromIntPtr(
        &mut self,
        ptr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Unity::Burst::IFunctionPointer>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Unity::Burst::IFunctionPointer,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Unity.Burst.IFunctionPointer.FromIntPtr",
            (ptr),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        ptr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (ptr),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Invoke(&mut self) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Invoke",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsCreated(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsCreated",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Value(&mut self) -> quest_hook::libil2cpp::Result<crate::System::IntPtr>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::System::IntPtr = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Value",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+FunctionPointer_1")]
impl<T: quest_hook::libil2cpp::Type> AsRef<crate::Unity::Burst::IFunctionPointer>
for crate::Unity::Burst::FunctionPointer_1<T> {
    fn as_ref(&self) -> &crate::Unity::Burst::IFunctionPointer {
        todo!()
    }
}
#[cfg(feature = "Unity+Burst+FunctionPointer_1")]
impl<T: quest_hook::libil2cpp::Type> AsMut<crate::Unity::Burst::IFunctionPointer>
for crate::Unity::Burst::FunctionPointer_1<T> {
    fn as_mut(&mut self) -> &mut crate::Unity::Burst::IFunctionPointer {
        todo!()
    }
}
