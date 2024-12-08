#[cfg(feature = "TMPro+TMPro_EventManager")]
#[repr(C)]
#[derive(Debug)]
pub struct TMPro_EventManager {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "TMPro+TMPro_EventManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMPro_EventManager => "TMPro"
    ."TMPro_EventManager"
);
#[cfg(feature = "TMPro+TMPro_EventManager")]
impl std::ops::Deref for crate::TMPro::TMPro_EventManager {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMPro_EventManager")]
impl std::ops::DerefMut for crate::TMPro::TMPro_EventManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMPro_EventManager")]
impl crate::TMPro::TMPro_EventManager {}
#[cfg(feature = "TMPro+TMPro_EventManager")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMPro_EventManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
