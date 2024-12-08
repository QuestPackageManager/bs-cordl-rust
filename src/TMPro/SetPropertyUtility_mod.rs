#[cfg(feature = "TMPro+SetPropertyUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct SetPropertyUtility {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "TMPro+SetPropertyUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::SetPropertyUtility => "TMPro"
    ."SetPropertyUtility"
);
#[cfg(feature = "TMPro+SetPropertyUtility")]
impl std::ops::Deref for crate::TMPro::SetPropertyUtility {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+SetPropertyUtility")]
impl std::ops::DerefMut for crate::TMPro::SetPropertyUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+SetPropertyUtility")]
impl crate::TMPro::SetPropertyUtility {}
#[cfg(feature = "TMPro+SetPropertyUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::SetPropertyUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
