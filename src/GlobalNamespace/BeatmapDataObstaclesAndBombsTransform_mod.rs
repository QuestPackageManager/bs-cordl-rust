#[cfg(feature = "BeatmapDataObstaclesAndBombsTransform")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataObstaclesAndBombsTransform {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatmapDataObstaclesAndBombsTransform")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatmapDataObstaclesAndBombsTransform {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapDataObstaclesAndBombsTransform";
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
#[cfg(feature = "BeatmapDataObstaclesAndBombsTransform")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapDataObstaclesAndBombsTransform {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataObstaclesAndBombsTransform")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BeatmapDataObstaclesAndBombsTransform {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataObstaclesAndBombsTransform")]
impl crate::GlobalNamespace::BeatmapDataObstaclesAndBombsTransform {
    pub fn CreateTransformedData(
        beatmapData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyBeatmapData,
        >,
        enabledObstaclesType: crate::GlobalNamespace::GameplayModifiers_EnabledObstacleType,
        noBombs: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IReadonlyBeatmapData>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IReadonlyBeatmapData,
                            >,
                            crate::GlobalNamespace::GameplayModifiers_EnabledObstacleType,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::IReadonlyBeatmapData,
                        >,
                        3usize,
                    >("CreateTransformedData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CreateTransformedData", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyBeatmapData,
        > = unsafe {
            method.invoke_unchecked((), (beatmapData, enabledObstaclesType, noBombs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShouldUseBeatmapDataItem(
        beatmapDataItem: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataItem,
        >,
        enabledObstaclesType: crate::GlobalNamespace::GameplayModifiers_EnabledObstacleType,
        noBombs: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapDataItem,
                            >,
                            crate::GlobalNamespace::GameplayModifiers_EnabledObstacleType,
                            bool,
                        ),
                        bool,
                        3usize,
                    >("ShouldUseBeatmapDataItem")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ShouldUseBeatmapDataItem", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked((), (beatmapDataItem, enabledObstaclesType, noBombs))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataObstaclesAndBombsTransform")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapDataObstaclesAndBombsTransform {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
