#[cfg(feature = "BeatmapDataObstaclesMergingTransform")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataObstaclesMergingTransform {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatmapDataObstaclesMergingTransform")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatmapDataObstaclesMergingTransform {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapDataObstaclesMergingTransform";
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
#[cfg(feature = "BeatmapDataObstaclesMergingTransform")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapDataObstaclesMergingTransform {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataObstaclesMergingTransform")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BeatmapDataObstaclesMergingTransform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataObstaclesMergingTransform")]
impl crate::GlobalNamespace::BeatmapDataObstaclesMergingTransform {
    pub fn CanBeMerged(
        firstObstacle: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ObstacleData>,
        secondObstacle: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ObstacleData>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::BeatmapDataObstaclesMergingTransform as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ObstacleData>,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ObstacleData>,
                ),
                bool,
                2usize,
            >("CanBeMerged")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::BeatmapDataObstaclesMergingTransform as
                    quest_hook::libil2cpp::Type > ::class(), "CanBeMerged", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (firstObstacle, secondObstacle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateTransformedData(
        beatmapData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyBeatmapData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IReadonlyBeatmapData>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::BeatmapDataObstaclesMergingTransform as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::IReadonlyBeatmapData,
                >),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IReadonlyBeatmapData>,
                1usize,
            >("CreateTransformedData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::BeatmapDataObstaclesMergingTransform as
                    quest_hook::libil2cpp::Type > ::class(), "CreateTransformedData",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyBeatmapData,
        > = unsafe { method.invoke_unchecked((), (beatmapData))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataObstaclesMergingTransform")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapDataObstaclesMergingTransform {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
