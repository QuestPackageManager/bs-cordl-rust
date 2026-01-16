#[cfg(
    feature = "cordl_class_BeatmapDataLoaderVersion4+LightColorEventBoxGroupConverter"
)]
#[repr(C)]
#[derive(Debug)]
pub struct LightColorEventBoxGroupConverter {
    __cordl_parent: crate::BeatmapDataLoaderVersion4::EventBoxGroupConverter,
}
#[cfg(
    feature = "cordl_class_BeatmapDataLoaderVersion4+LightColorEventBoxGroupConverter"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapDataLoaderVersion4::LightColorEventBoxGroupConverter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapDataLoaderVersion4";
    const CLASS_NAME: &'static str = "LightColorEventBoxGroupConverter";
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
#[cfg(feature = "BeatmapDataLoaderVersion4+LightColorEventBoxGroupConverter")]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion4::LightColorEventBoxGroupConverter {
    type Target = crate::BeatmapDataLoaderVersion4::EventBoxGroupConverter;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+LightColorEventBoxGroupConverter")]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion4::LightColorEventBoxGroupConverter {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+LightColorEventBoxGroupConverter")]
impl crate::BeatmapDataLoaderVersion4::LightColorEventBoxGroupConverter {
    pub fn ConvertEvents(
        &mut self,
        eventBox: crate::BeatmapSaveDataVersion4::EventBox,
        indexFilter: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IndexFilter>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventDataBox>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::BeatmapSaveDataVersion4::EventBox,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IndexFilter,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapEventDataBox,
                        >,
                        2usize,
                    >("ConvertEvents")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConvertEvents", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapEventDataBox,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (eventBox, indexFilter))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        lightshowSaveData: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion4::LightshowSaveData,
        >,
        lightGroups: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IEnvironmentLightGroups,
        >,
        boxConverter: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapBoxConverter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (lightshowSaveData, lightGroups, boxConverter))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        lightshowSaveData: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion4::LightshowSaveData,
        >,
        lightGroups: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IEnvironmentLightGroups,
        >,
        boxConverter: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapBoxConverter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::BeatmapSaveDataVersion4::LightshowSaveData,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IEnvironmentLightGroups,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IBeatmapBoxConverter,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (lightshowSaveData, lightGroups, boxConverter))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_BeatmapDataLoaderVersion4+LightColorEventBoxGroupConverter"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion4::LightColorEventBoxGroupConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
