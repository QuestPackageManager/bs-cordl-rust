#[cfg(feature = "System+Net+Sockets+SafeSocketHandle")]
#[repr(C)]
#[derive(Debug)]
pub struct SafeSocketHandle {
    __cordl_parent: crate::Microsoft::Win32::SafeHandles::SafeHandleMinusOneIsInvalid,
    pub blocking_threads: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::System::Threading::Thread>,
        >,
    >,
    pub threads_stacktraces: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<crate::System::Threading::Thread>,
            quest_hook::libil2cpp::Gc<crate::System::Diagnostics::StackTrace>,
        >,
    >,
    pub in_cleanup: bool,
}
#[cfg(feature = "System+Net+Sockets+SafeSocketHandle")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Sockets::SafeSocketHandle =>
    "System.Net.Sockets"."SafeSocketHandle"
);
#[cfg(feature = "System+Net+Sockets+SafeSocketHandle")]
impl std::ops::Deref for crate::System::Net::Sockets::SafeSocketHandle {
    type Target = crate::Microsoft::Win32::SafeHandles::SafeHandleMinusOneIsInvalid;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Sockets+SafeSocketHandle")]
impl std::ops::DerefMut for crate::System::Net::Sockets::SafeSocketHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Sockets+SafeSocketHandle")]
impl crate::System::Net::Sockets::SafeSocketHandle {
    pub const ABORT_RETRIES: i32 = 10i32;
    pub const SOCKET_CLOSED: i32 = 10004i32;
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
    pub fn RegisterForBlockingSyscall(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterForBlockingSyscall", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReleaseHandle(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ReleaseHandle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnRegisterForBlockingSyscall(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnRegisterForBlockingSyscall", ())?;
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
#[cfg(feature = "System+Net+Sockets+SafeSocketHandle")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Sockets::SafeSocketHandle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
