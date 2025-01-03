#[cfg(feature = "System+Diagnostics+ProcessWaitHandle")]
#[repr(C)]
#[derive(Debug)]
pub struct ProcessWaitHandle {
    __cordl_parent: crate::System::Threading::WaitHandle,
}
#[cfg(feature = "System+Diagnostics+ProcessWaitHandle")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Diagnostics::ProcessWaitHandle =>
    "System.Diagnostics"."ProcessWaitHandle"
);
#[cfg(feature = "System+Diagnostics+ProcessWaitHandle")]
impl std::ops::Deref for crate::System::Diagnostics::ProcessWaitHandle {
    type Target = crate::System::Threading::WaitHandle;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+ProcessWaitHandle")]
impl std::ops::DerefMut for crate::System::Diagnostics::ProcessWaitHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+ProcessWaitHandle")]
impl crate::System::Diagnostics::ProcessWaitHandle {
    pub fn New(
        processHandle: quest_hook::libil2cpp::Gc<
            crate::Microsoft::Win32::SafeHandles::SafeProcessHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (processHandle))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        processHandle: quest_hook::libil2cpp::Gc<
            crate::Microsoft::Win32::SafeHandles::SafeProcessHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (processHandle))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Diagnostics+ProcessWaitHandle")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Diagnostics::ProcessWaitHandle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
