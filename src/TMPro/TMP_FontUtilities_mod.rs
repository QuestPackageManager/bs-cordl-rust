#[cfg(feature = "TMPro+TMP_FontUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_FontUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "TMPro+TMP_FontUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_FontUtilities => "TMPro"
    ."TMP_FontUtilities"
);
#[cfg(feature = "TMPro+TMP_FontUtilities")]
impl std::ops::Deref for crate::TMPro::TMP_FontUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_FontUtilities")]
impl std::ops::DerefMut for crate::TMPro::TMP_FontUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_FontUtilities")]
impl crate::TMPro::TMP_FontUtilities {}
#[cfg(feature = "TMPro+TMP_FontUtilities")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_FontUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
