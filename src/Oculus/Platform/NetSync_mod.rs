#[cfg(feature = "Oculus+Platform+NetSync")]
#[repr(C)]
#[derive(Debug)]
pub struct NetSync {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Oculus+Platform+NetSync")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::NetSync => "Oculus.Platform"
    ."NetSync"
);
#[cfg(feature = "Oculus+Platform+NetSync")]
impl std::ops::Deref for crate::Oculus::Platform::NetSync {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+NetSync")]
impl std::ops::DerefMut for crate::Oculus::Platform::NetSync {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+NetSync")]
impl crate::Oculus::Platform::NetSync {
    pub fn SetConnectionStatusChangedNotificationCallback(
        callback: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Message_1_Callback<
                *mut crate::Oculus::Platform::Models::NetSyncConnection,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetConnectionStatusChangedNotificationCallback", (callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSessionsChangedNotificationCallback(
        callback: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Message_1_Callback<
                *mut crate::Oculus::Platform::Models::NetSyncSessionsChangedNotification,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetSessionsChangedNotificationCallback", (callback))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+NetSync")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::NetSync {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
