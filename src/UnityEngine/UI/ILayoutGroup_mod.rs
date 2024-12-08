#[cfg(feature = "UnityEngine+UI+ILayoutGroup")]
#[repr(C)]
#[derive(Debug)]
pub struct ILayoutGroup {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UI+ILayoutGroup")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::ILayoutGroup =>
    "UnityEngine.UI"."ILayoutGroup"
);
#[cfg(feature = "UnityEngine+UI+ILayoutGroup")]
impl std::ops::Deref for crate::UnityEngine::UI::ILayoutGroup {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+ILayoutGroup")]
impl std::ops::DerefMut for crate::UnityEngine::UI::ILayoutGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+ILayoutGroup")]
impl crate::UnityEngine::UI::ILayoutGroup {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+UI+ILayoutGroup")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UI::ILayoutGroup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
