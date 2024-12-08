#[cfg(feature = "TMPro+TMPro_ExtensionMethods")]
#[repr(C)]
#[derive(Debug)]
pub struct TMPro_ExtensionMethods {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "TMPro+TMPro_ExtensionMethods")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMPro_ExtensionMethods => "TMPro"
    ."TMPro_ExtensionMethods"
);
#[cfg(feature = "TMPro+TMPro_ExtensionMethods")]
impl std::ops::Deref for crate::TMPro::TMPro_ExtensionMethods {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMPro_ExtensionMethods")]
impl std::ops::DerefMut for crate::TMPro::TMPro_ExtensionMethods {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMPro_ExtensionMethods")]
impl crate::TMPro::TMPro_ExtensionMethods {}
#[cfg(feature = "TMPro+TMPro_ExtensionMethods")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMPro_ExtensionMethods {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
