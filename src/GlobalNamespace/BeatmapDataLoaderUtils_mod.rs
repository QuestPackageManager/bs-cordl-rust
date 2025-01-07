#[cfg(feature = "BeatmapDataLoaderUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoaderUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatmapDataLoaderUtils")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatmapDataLoaderUtils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapDataLoaderUtils";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "BeatmapDataLoaderUtils")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapDataLoaderUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderUtils")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapDataLoaderUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderUtils")]
impl crate::GlobalNamespace::BeatmapDataLoaderUtils {
    pub fn GetEnvironmentKeywords(
        environmentInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IEnvironmentInfo,
        >,
        beatmapLevelDataVersion: crate::GlobalNamespace::BeatmapLevelDataVersion,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::EnvironmentKeywords>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentKeywords,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetEnvironmentKeywords",
                (environmentInfo, beatmapLevelDataVersion),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEnvironmentLightGroups(
        environmentInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IEnvironmentInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IEnvironmentLightGroups>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IEnvironmentLightGroups,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEnvironmentLightGroups", (environmentInfo))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataLoaderUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapDataLoaderUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
