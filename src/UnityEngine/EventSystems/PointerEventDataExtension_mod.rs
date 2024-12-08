#[cfg(feature = "UnityEngine+EventSystems+PointerEventDataExtension")]
#[repr(C)]
#[derive(Debug)]
pub struct PointerEventDataExtension {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+EventSystems+PointerEventDataExtension")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::EventSystems::PointerEventDataExtension => "UnityEngine.EventSystems"
    ."PointerEventDataExtension"
);
#[cfg(feature = "UnityEngine+EventSystems+PointerEventDataExtension")]
impl std::ops::Deref for crate::UnityEngine::EventSystems::PointerEventDataExtension {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+PointerEventDataExtension")]
impl std::ops::DerefMut for crate::UnityEngine::EventSystems::PointerEventDataExtension {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+PointerEventDataExtension")]
impl crate::UnityEngine::EventSystems::PointerEventDataExtension {}
#[cfg(feature = "UnityEngine+EventSystems+PointerEventDataExtension")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::EventSystems::PointerEventDataExtension {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
