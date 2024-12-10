#[cfg(feature = "UnityEngine+Timeline+INotificationOptionProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct INotificationOptionProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Timeline+INotificationOptionProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Timeline::INotificationOptionProvider => "UnityEngine.Timeline"
    ."INotificationOptionProvider"
);
#[cfg(feature = "UnityEngine+Timeline+INotificationOptionProvider")]
impl std::ops::Deref for crate::UnityEngine::Timeline::INotificationOptionProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+INotificationOptionProvider")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::INotificationOptionProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+INotificationOptionProvider")]
impl crate::UnityEngine::Timeline::INotificationOptionProvider {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_flags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Timeline::NotificationFlags> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Timeline::NotificationFlags = __cordl_object
            .invoke("get_flags", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Timeline+INotificationOptionProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Timeline::INotificationOptionProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
