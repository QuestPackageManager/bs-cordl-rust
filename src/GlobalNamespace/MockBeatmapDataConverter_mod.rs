#[cfg(feature = "MockBeatmapDataConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct MockBeatmapDataConverter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "MockBeatmapDataConverter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MockBeatmapDataConverter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MockBeatmapDataConverter";
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
#[cfg(feature = "MockBeatmapDataConverter")]
impl std::ops::Deref for crate::GlobalNamespace::MockBeatmapDataConverter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockBeatmapDataConverter")]
impl std::ops::DerefMut for crate::GlobalNamespace::MockBeatmapDataConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockBeatmapDataConverter")]
impl crate::GlobalNamespace::MockBeatmapDataConverter {
    pub fn ToMockBeatmapData(
        beatmapData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyBeatmapData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MockBeatmapData>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::IReadonlyBeatmapData,
                >),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MockBeatmapData>,
                1usize,
            >("ToMockBeatmapData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToMockBeatmapData", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MockBeatmapData,
        > = unsafe { method.invoke_unchecked((), (beatmapData)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToMockNoteData(
        noteData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MockNoteData>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MockNoteData>,
                1usize,
            >("ToMockNoteData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToMockNoteData", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MockNoteData,
        > = unsafe { method.invoke_unchecked((), (noteData)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToMockObstacleData(
        obstacleData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ObstacleData>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MockObstacleData>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ObstacleData>),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MockObstacleData>,
                1usize,
            >("ToMockObstacleData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToMockObstacleData", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MockObstacleData,
        > = unsafe { method.invoke_unchecked((), (obstacleData)) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MockBeatmapDataConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MockBeatmapDataConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
