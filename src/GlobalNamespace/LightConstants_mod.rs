#[cfg(feature = "LightConstants+BakeId")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightConstants_BakeId {
    A = 1i32,
    B = 2i32,
    C = 3i32,
    D = 4i32,
    E = 5i32,
    F = 6i32,
}
#[cfg(feature = "LightConstants+BakeId")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LightConstants_BakeId => ""
    ."LightConstants/BakeId"
);
#[cfg(feature = "LightConstants")]
#[repr(C)]
#[derive(Debug)]
pub struct LightConstants {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "LightConstants")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for LightConstants => ""."LightConstants"
);
#[cfg(feature = "LightConstants")]
impl std::ops::Deref for LightConstants {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightConstants")]
impl std::ops::DerefMut for LightConstants {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightConstants")]
impl LightConstants {
    pub const kBaseLightId: i32 = 25i32;
    pub const kLightProbeLightBakeIdPrefix: &'static str = "_LightProbeLightBakeId";
    #[cfg(feature = "LightConstants+BakeId")]
    pub type BakeId = crate::GlobalNamespace::LightConstants_BakeId;
}
#[cfg(feature = "LightConstants")]
impl quest_hook::libil2cpp::ObjectType for LightConstants {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
