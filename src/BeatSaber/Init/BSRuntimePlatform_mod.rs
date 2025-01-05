#[cfg(feature = "BeatSaber+Init+BSRuntimePlatform")]
#[repr(C)]
#[derive(Debug)]
pub struct BSRuntimePlatform {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatSaber+Init+BSRuntimePlatform")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::Init::BSRuntimePlatform =>
    "BeatSaber.Init"."BSRuntimePlatform"
);
#[cfg(feature = "BeatSaber+Init+BSRuntimePlatform")]
impl std::ops::Deref for crate::BeatSaber::Init::BSRuntimePlatform {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Init+BSRuntimePlatform")]
impl std::ops::DerefMut for crate::BeatSaber::Init::BSRuntimePlatform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Init+BSRuntimePlatform")]
impl crate::BeatSaber::Init::BSRuntimePlatform {
    pub fn GetPlatformType() -> quest_hook::libil2cpp::Result<
        crate::BeatSaber::Init::RuntimePlatformType,
    > {
        let __cordl_ret: crate::BeatSaber::Init::RuntimePlatformType = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPlatformType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsSteam() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsSteam", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+Init+BSRuntimePlatform")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatSaber::Init::BSRuntimePlatform {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
