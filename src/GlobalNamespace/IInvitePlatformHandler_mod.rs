#[cfg(feature = "IInvitePlatformHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct IInvitePlatformHandler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IInvitePlatformHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IInvitePlatformHandler => ""
    ."IInvitePlatformHandler"
);
#[cfg(feature = "IInvitePlatformHandler")]
impl std::ops::Deref for crate::GlobalNamespace::IInvitePlatformHandler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IInvitePlatformHandler")]
impl std::ops::DerefMut for crate::GlobalNamespace::IInvitePlatformHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IInvitePlatformHandler")]
impl crate::GlobalNamespace::IInvitePlatformHandler {
    pub fn OpenInvitePanel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OpenInvitePanel", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_isSupported(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isSupported", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "IInvitePlatformHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IInvitePlatformHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
