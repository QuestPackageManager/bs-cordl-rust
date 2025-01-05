#[cfg(feature = "Microsoft+Win32+SafeHandles+SafeWaitHandle")]
#[repr(C)]
#[derive(Debug)]
pub struct SafeWaitHandle {
    __cordl_parent: crate::Microsoft::Win32::SafeHandles::SafeHandleZeroOrMinusOneIsInvalid,
}
#[cfg(feature = "Microsoft+Win32+SafeHandles+SafeWaitHandle")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Microsoft::Win32::SafeHandles::SafeWaitHandle =>
    "Microsoft.Win32.SafeHandles"."SafeWaitHandle"
);
#[cfg(feature = "Microsoft+Win32+SafeHandles+SafeWaitHandle")]
impl std::ops::Deref for crate::Microsoft::Win32::SafeHandles::SafeWaitHandle {
    type Target = crate::Microsoft::Win32::SafeHandles::SafeHandleZeroOrMinusOneIsInvalid;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Microsoft+Win32+SafeHandles+SafeWaitHandle")]
impl std::ops::DerefMut for crate::Microsoft::Win32::SafeHandles::SafeWaitHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Microsoft+Win32+SafeHandles+SafeWaitHandle")]
impl crate::Microsoft::Win32::SafeHandles::SafeWaitHandle {
    pub fn New(
        existingHandle: crate::System::IntPtr,
        ownsHandle: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (existingHandle, ownsHandle))?;
        Ok(__cordl_object.into())
    }
    pub fn ReleaseHandle(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ReleaseHandle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        existingHandle: crate::System::IntPtr,
        ownsHandle: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (existingHandle, ownsHandle))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Microsoft+Win32+SafeHandles+SafeWaitHandle")]
impl quest_hook::libil2cpp::ObjectType
for crate::Microsoft::Win32::SafeHandles::SafeWaitHandle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
