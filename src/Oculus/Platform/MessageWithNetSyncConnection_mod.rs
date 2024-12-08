#[cfg(feature = "Oculus+Platform+MessageWithNetSyncConnection")]
#[repr(C)]
#[derive(Debug)]
pub struct MessageWithNetSyncConnection {
    __cordl_parent: crate::Oculus::Platform::Message_1<
        *mut crate::Oculus::Platform::Models::NetSyncConnection,
    >,
}
#[cfg(feature = "Oculus+Platform+MessageWithNetSyncConnection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::MessageWithNetSyncConnection
    => "Oculus.Platform"."MessageWithNetSyncConnection"
);
#[cfg(feature = "Oculus+Platform+MessageWithNetSyncConnection")]
impl std::ops::Deref for crate::Oculus::Platform::MessageWithNetSyncConnection {
    type Target = crate::Oculus::Platform::Message_1<
        *mut crate::Oculus::Platform::Models::NetSyncConnection,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+MessageWithNetSyncConnection")]
impl std::ops::DerefMut for crate::Oculus::Platform::MessageWithNetSyncConnection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+MessageWithNetSyncConnection")]
impl crate::Oculus::Platform::MessageWithNetSyncConnection {
    pub fn GetDataFromMessage(
        &mut self,
        c_message: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::NetSyncConnection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::NetSyncConnection = __cordl_object
            .invoke("GetDataFromMessage", (c_message))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        c_message: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (c_message))?;
        Ok(__cordl_ret)
    }
    pub fn GetNetSyncConnection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::NetSyncConnection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::NetSyncConnection = __cordl_object
            .invoke("GetNetSyncConnection", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        c_message: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (c_message))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Oculus+Platform+MessageWithNetSyncConnection")]
impl quest_hook::libil2cpp::ObjectType
for crate::Oculus::Platform::MessageWithNetSyncConnection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
