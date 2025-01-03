#[cfg(feature = "LightConstants")]
#[repr(C)]
#[derive(Debug)]
pub struct LightConstants {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "LightConstants")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LightConstants => ""
    ."LightConstants"
);
#[cfg(feature = "LightConstants")]
impl std::ops::Deref for crate::GlobalNamespace::LightConstants {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightConstants")]
impl std::ops::DerefMut for crate::GlobalNamespace::LightConstants {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightConstants")]
impl crate::GlobalNamespace::LightConstants {
    pub const kBaseLightId: i32 = 25i32;
    pub const kLightProbeLightBakeIdPrefix: &'static str = "_LightProbeLightBakeId";
    pub const kLightmapLightBakeIdPrefix: &'static str = "_LightmapLightBakeId";
    #[cfg(feature = "LightConstants+BakeId")]
    pub type BakeId = crate::GlobalNamespace::LightConstants_BakeId;
    pub fn GetComputeFieldPropertyId(
        fieldName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetComputeFieldPropertyId", (fieldName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLightProbeLightBakeIdPropertyId(
        bakeId: crate::GlobalNamespace::LightConstants_BakeId,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLightProbeLightBakeIdPropertyId", (bakeId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLightmapLightBakeIdPropertyId(
        bakeId: crate::GlobalNamespace::LightConstants_BakeId,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLightmapLightBakeIdPropertyId", (bakeId))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LightConstants")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::LightConstants {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
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
