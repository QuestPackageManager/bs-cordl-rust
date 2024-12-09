#[cfg(feature = "TMPro+ShaderUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct ShaderUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "TMPro+ShaderUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::ShaderUtilities => "TMPro"
    ."ShaderUtilities"
);
#[cfg(feature = "TMPro+ShaderUtilities")]
impl std::ops::Deref for crate::TMPro::ShaderUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+ShaderUtilities")]
impl std::ops::DerefMut for crate::TMPro::ShaderUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+ShaderUtilities")]
impl crate::TMPro::ShaderUtilities {}
#[cfg(feature = "TMPro+ShaderUtilities")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::ShaderUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
