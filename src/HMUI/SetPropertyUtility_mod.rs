#[cfg(feature = "HMUI+SetPropertyUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct SetPropertyUtility {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "HMUI+SetPropertyUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::SetPropertyUtility => "HMUI"
    ."SetPropertyUtility"
);
#[cfg(feature = "HMUI+SetPropertyUtility")]
impl std::ops::Deref for crate::HMUI::SetPropertyUtility {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+SetPropertyUtility")]
impl std::ops::DerefMut for crate::HMUI::SetPropertyUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+SetPropertyUtility")]
impl crate::HMUI::SetPropertyUtility {}
#[cfg(feature = "HMUI+SetPropertyUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::SetPropertyUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
