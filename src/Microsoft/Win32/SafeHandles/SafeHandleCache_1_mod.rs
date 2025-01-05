#[cfg(feature = "Microsoft+Win32+SafeHandles+SafeHandleCache_1")]
#[repr(C)]
#[derive(Debug)]
pub struct SafeHandleCache_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "Microsoft+Win32+SafeHandles+SafeHandleCache_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Microsoft::Win32::SafeHandles::SafeHandleCache_1
    < T > => "Microsoft.Win32.SafeHandles"."SafeHandleCache`1" < T >
);
#[cfg(feature = "Microsoft+Win32+SafeHandles+SafeHandleCache_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Microsoft::Win32::SafeHandles::SafeHandleCache_1<T> {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Microsoft+Win32+SafeHandles+SafeHandleCache_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Microsoft::Win32::SafeHandles::SafeHandleCache_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Microsoft+Win32+SafeHandles+SafeHandleCache_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::Microsoft::Win32::SafeHandles::SafeHandleCache_1<T> {
    pub fn IsCachedInvalidHandle(
        handle: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::InteropServices::SafeHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsCachedInvalidHandle", (handle))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Microsoft+Win32+SafeHandles+SafeHandleCache_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Microsoft::Win32::SafeHandles::SafeHandleCache_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
