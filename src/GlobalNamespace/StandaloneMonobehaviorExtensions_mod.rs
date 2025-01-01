#[cfg(feature = "StandaloneMonobehaviorExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct StandaloneMonobehaviorExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "StandaloneMonobehaviorExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::StandaloneMonobehaviorExtensions => ""
    ."StandaloneMonobehaviorExtensions"
);
#[cfg(feature = "StandaloneMonobehaviorExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::StandaloneMonobehaviorExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StandaloneMonobehaviorExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::StandaloneMonobehaviorExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StandaloneMonobehaviorExtensions")]
impl crate::GlobalNamespace::StandaloneMonobehaviorExtensions {}
#[cfg(feature = "StandaloneMonobehaviorExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::StandaloneMonobehaviorExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}