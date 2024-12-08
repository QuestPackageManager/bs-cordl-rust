#[cfg(feature = "Oculus+Platform+MessageWithNetSyncSessionsChangedNotification")]
#[repr(C)]
#[derive(Debug)]
pub struct MessageWithNetSyncSessionsChangedNotification {
    __cordl_parent: crate::Oculus::Platform::Message_1<
        *mut crate::Oculus::Platform::Models::NetSyncSessionsChangedNotification,
    >,
}
#[cfg(feature = "Oculus+Platform+MessageWithNetSyncSessionsChangedNotification")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Oculus::Platform::MessageWithNetSyncSessionsChangedNotification =>
    "Oculus.Platform"."MessageWithNetSyncSessionsChangedNotification"
);
#[cfg(feature = "Oculus+Platform+MessageWithNetSyncSessionsChangedNotification")]
impl std::ops::Deref
for crate::Oculus::Platform::MessageWithNetSyncSessionsChangedNotification {
    type Target = crate::Oculus::Platform::Message_1<
        *mut crate::Oculus::Platform::Models::NetSyncSessionsChangedNotification,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+MessageWithNetSyncSessionsChangedNotification")]
impl std::ops::DerefMut
for crate::Oculus::Platform::MessageWithNetSyncSessionsChangedNotification {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+MessageWithNetSyncSessionsChangedNotification")]
impl crate::Oculus::Platform::MessageWithNetSyncSessionsChangedNotification {
    pub fn GetDataFromMessage(
        &mut self,
        c_message: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::NetSyncSessionsChangedNotification,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::NetSyncSessionsChangedNotification = __cordl_object
            .invoke("GetDataFromMessage", (c_message))?;
        Ok(__cordl_ret)
    }
    pub fn GetNetSyncSessionsChangedNotification(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::NetSyncSessionsChangedNotification,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::NetSyncSessionsChangedNotification = __cordl_object
            .invoke("GetNetSyncSessionsChangedNotification", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        c_message: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (c_message))?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "Oculus+Platform+MessageWithNetSyncSessionsChangedNotification")]
impl quest_hook::libil2cpp::ObjectType
for crate::Oculus::Platform::MessageWithNetSyncSessionsChangedNotification {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}