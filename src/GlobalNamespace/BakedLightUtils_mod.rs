#[cfg(feature = "BakedLightUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct BakedLightUtils {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BakedLightUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BakedLightUtils => ""
    ."BakedLightUtils"
);
#[cfg(feature = "BakedLightUtils")]
impl std::ops::Deref for crate::GlobalNamespace::BakedLightUtils {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BakedLightUtils")]
impl std::ops::DerefMut for crate::GlobalNamespace::BakedLightUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BakedLightUtils")]
impl crate::GlobalNamespace::BakedLightUtils {
    pub const kDepthOnlyShaderName: &'static str = "Custom/SetDepthOnly";
    pub const kMirrorParentNameToIgnore: &'static str = "PlayersPlace";
}
#[cfg(feature = "BakedLightUtils")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BakedLightUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
