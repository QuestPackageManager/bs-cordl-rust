#[cfg(feature = "Microsoft+Win32+SafeHandles+SafeFileHandle")]
#[repr(C)]
#[derive(Debug)]
pub struct SafeFileHandle {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::Microsoft::Win32::SafeHandles::SafeHandleZeroOrMinusOneIsInvalid,
    >,
}
#[cfg(feature = "Microsoft+Win32+SafeHandles+SafeFileHandle")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Microsoft::Win32::SafeHandles::SafeFileHandle =>
    "Microsoft.Win32.SafeHandles"."SafeFileHandle"
);
#[cfg(feature = "Microsoft+Win32+SafeHandles+SafeFileHandle")]
impl std::ops::Deref for crate::Microsoft::Win32::SafeHandles::SafeFileHandle {
    type Target = quest_hook::libil2cpp::Gc<
        crate::Microsoft::Win32::SafeHandles::SafeHandleZeroOrMinusOneIsInvalid,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Microsoft+Win32+SafeHandles+SafeFileHandle")]
impl std::ops::DerefMut for crate::Microsoft::Win32::SafeHandles::SafeFileHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Microsoft+Win32+SafeHandles+SafeFileHandle")]
impl crate::Microsoft::Win32::SafeHandles::SafeFileHandle {
    pub fn New(
        preexistingHandle: crate::System::IntPtr,
        ownsHandle: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (preexistingHandle, ownsHandle))?;
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
        preexistingHandle: crate::System::IntPtr,
        ownsHandle: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (preexistingHandle, ownsHandle))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Microsoft+Win32+SafeHandles+SafeFileHandle")]
impl quest_hook::libil2cpp::ObjectType
for crate::Microsoft::Win32::SafeHandles::SafeFileHandle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
