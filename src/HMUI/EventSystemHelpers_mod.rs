#[cfg(feature = "HMUI+EventSystemHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct EventSystemHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "HMUI+EventSystemHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::EventSystemHelpers => "HMUI"
    ."EventSystemHelpers"
);
#[cfg(feature = "HMUI+EventSystemHelpers")]
impl std::ops::Deref for crate::HMUI::EventSystemHelpers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+EventSystemHelpers")]
impl std::ops::DerefMut for crate::HMUI::EventSystemHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+EventSystemHelpers")]
impl crate::HMUI::EventSystemHelpers {
    pub fn IsInputFieldSelected() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsInputFieldSelected", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HMUI+EventSystemHelpers")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::EventSystemHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
