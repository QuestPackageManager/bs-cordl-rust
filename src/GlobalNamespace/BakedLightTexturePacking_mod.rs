#[cfg(feature = "BakedLightTexturePacking")]
#[repr(C)]
#[derive(Debug)]
pub struct BakedLightTexturePacking {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BakedLightTexturePacking")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BakedLightTexturePacking => ""
    ."BakedLightTexturePacking"
);
#[cfg(feature = "BakedLightTexturePacking")]
impl std::ops::Deref for BakedLightTexturePacking {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BakedLightTexturePacking")]
impl std::ops::DerefMut for BakedLightTexturePacking {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BakedLightTexturePacking")]
impl BakedLightTexturePacking {
    pub const kBakedLightTexturePackingShaderName: &'static str = "Hidden/BakedLightTexturePacking";
}
#[cfg(feature = "BakedLightTexturePacking")]
impl quest_hook::libil2cpp::ObjectType for BakedLightTexturePacking {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
