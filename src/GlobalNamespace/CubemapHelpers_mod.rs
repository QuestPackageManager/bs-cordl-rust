#[cfg(feature = "CubemapHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct CubemapHelpers {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "CubemapHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::CubemapHelpers => ""
    ."CubemapHelpers"
);
#[cfg(feature = "CubemapHelpers")]
impl std::ops::Deref for crate::GlobalNamespace::CubemapHelpers {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CubemapHelpers")]
impl std::ops::DerefMut for crate::GlobalNamespace::CubemapHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CubemapHelpers")]
impl crate::GlobalNamespace::CubemapHelpers {
    pub const kCubemapDownsamplePass: i32 = 0i32;
    pub const kCubemapHelpersShaderName: &'static str = "Hidden/CubemapHelpers";
    pub const kCubemapTo2DTexturePass: i32 = 1i32;
}
#[cfg(feature = "CubemapHelpers")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::CubemapHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
