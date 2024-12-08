#[cfg(feature = "Microsoft+Win32+SafeHandles+SafeProcessHandle")]
#[repr(C)]
#[derive(Debug)]
pub struct SafeProcessHandle {
    __cordl_parent: crate::Microsoft::Win32::SafeHandles::SafeHandleZeroOrMinusOneIsInvalid,
}
#[cfg(feature = "Microsoft+Win32+SafeHandles+SafeProcessHandle")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Microsoft::Win32::SafeHandles::SafeProcessHandle
    => "Microsoft.Win32.SafeHandles"."SafeProcessHandle"
);
#[cfg(feature = "Microsoft+Win32+SafeHandles+SafeProcessHandle")]
impl std::ops::Deref for crate::Microsoft::Win32::SafeHandles::SafeProcessHandle {
    type Target = crate::Microsoft::Win32::SafeHandles::SafeHandleZeroOrMinusOneIsInvalid;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Microsoft+Win32+SafeHandles+SafeProcessHandle")]
impl std::ops::DerefMut for crate::Microsoft::Win32::SafeHandles::SafeProcessHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Microsoft+Win32+SafeHandles+SafeProcessHandle")]
impl crate::Microsoft::Win32::SafeHandles::SafeProcessHandle {
    pub fn _ctor_IntPtr0(
        &mut self,
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (handle))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor__cordl_bool1(
        &mut self,
        existingHandle: crate::System::IntPtr,
        ownsHandle: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (existingHandle, ownsHandle))?;
        Ok(__cordl_ret)
    }
    pub fn ReleaseHandle(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ReleaseHandle", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_IntPtr0(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (handle))?;
        Ok(__cordl_object)
    }
    pub fn New__cordl_bool1(
        existingHandle: crate::System::IntPtr,
        ownsHandle: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (existingHandle, ownsHandle))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Microsoft+Win32+SafeHandles+SafeProcessHandle")]
impl quest_hook::libil2cpp::ObjectType
for crate::Microsoft::Win32::SafeHandles::SafeProcessHandle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
