#[cfg(feature = "PseudoHDREncoding")]
#[repr(C)]
#[derive(Debug)]
pub struct PseudoHDREncoding {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "PseudoHDREncoding")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for PseudoHDREncoding => ""."PseudoHDREncoding"
);
#[cfg(feature = "PseudoHDREncoding")]
impl std::ops::Deref for PseudoHDREncoding {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PseudoHDREncoding")]
impl std::ops::DerefMut for PseudoHDREncoding {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PseudoHDREncoding")]
impl PseudoHDREncoding {
    pub const kPseudoHDREncodingShaderName: &'static str = "Hidden/PseudoHDREncoding";
}
#[cfg(feature = "PseudoHDREncoding")]
impl quest_hook::libil2cpp::ObjectType for PseudoHDREncoding {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
