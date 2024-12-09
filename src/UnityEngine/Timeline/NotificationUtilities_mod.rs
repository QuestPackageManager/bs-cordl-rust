#[cfg(feature = "UnityEngine+Timeline+NotificationUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct NotificationUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Timeline+NotificationUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::NotificationUtilities =>
    "UnityEngine.Timeline"."NotificationUtilities"
);
#[cfg(feature = "UnityEngine+Timeline+NotificationUtilities")]
impl std::ops::Deref for crate::UnityEngine::Timeline::NotificationUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+NotificationUtilities")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::NotificationUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+NotificationUtilities")]
impl crate::UnityEngine::Timeline::NotificationUtilities {}
#[cfg(feature = "UnityEngine+Timeline+NotificationUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Timeline::NotificationUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
