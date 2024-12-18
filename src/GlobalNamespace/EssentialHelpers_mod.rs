#[cfg(feature = "EssentialHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct EssentialHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "EssentialHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::EssentialHelpers => ""
    ."EssentialHelpers"
);
#[cfg(feature = "EssentialHelpers")]
impl std::ops::Deref for crate::GlobalNamespace::EssentialHelpers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EssentialHelpers")]
impl std::ops::DerefMut for crate::GlobalNamespace::EssentialHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EssentialHelpers")]
impl crate::GlobalNamespace::EssentialHelpers {}
#[cfg(feature = "EssentialHelpers")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::EssentialHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
