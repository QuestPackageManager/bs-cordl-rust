#[cfg(feature = "cordl_class_BeatmapDataLoaderVersion4+ObstacleItemConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct ObstacleItemConverter {
    __cordl_parent: crate::GlobalNamespace::BeatToTimeConverter,
    pub _obstacles: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::BeatmapSaveDataVersion4::Obstacle>,
    >,
}
#[cfg(feature = "cordl_class_BeatmapDataLoaderVersion4+ObstacleItemConverter")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::BeatmapDataLoaderVersion4::ObstacleItemConverter
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapDataLoaderVersion4";
    const CLASS_NAME: &'static str = "ObstacleItemConverter";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "BeatmapDataLoaderVersion4+ObstacleItemConverter")]
impl std::ops::Deref for crate::BeatmapDataLoaderVersion4::ObstacleItemConverter {
    type Target = crate::GlobalNamespace::BeatToTimeConverter;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+ObstacleItemConverter")]
impl std::ops::DerefMut for crate::BeatmapDataLoaderVersion4::ObstacleItemConverter {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+ObstacleItemConverter")]
impl crate::BeatmapDataLoaderVersion4::ObstacleItemConverter {
    pub fn Convert(
        &mut self,
        index: quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataVersion4::BeatmapBeatIndex>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapObjectData>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::BeatmapSaveDataVersion4::BeatmapBeatIndex,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapObjectData,
                        >,
                        1usize,
                    >("Convert")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Convert",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapObjectData> =
            unsafe { cordl_method_info.invoke_unchecked(self, (index))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        obstacles: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::BeatmapSaveDataVersion4::Obstacle>,
        >,
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BpmTimeProcessor>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (obstacles, bpmTimeProcessor))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        obstacles: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::BeatmapSaveDataVersion4::Obstacle>,
        >,
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BpmTimeProcessor>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::BeatmapSaveDataVersion4::Obstacle,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BpmTimeProcessor>,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (obstacles, bpmTimeProcessor))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BeatmapDataLoaderVersion4+ObstacleItemConverter")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatmapDataLoaderVersion4::ObstacleItemConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
