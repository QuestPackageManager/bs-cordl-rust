#[cfg(feature = "cordl_class_BeatmapBasicData")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapBasicData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub noteJumpMovementSpeed: f32,
    pub noteJumpStartBeatOffset: f32,
    pub environmentName: crate::GlobalNamespace::EnvironmentName,
    pub beatmapColorScheme: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ColorScheme,
    >,
    pub notesCount: i32,
    pub cuttableObjectsCount: i32,
    pub obstaclesCount: i32,
    pub bombsCount: i32,
    pub mappers: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub lighters: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
}
#[cfg(feature = "cordl_class_BeatmapBasicData")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::BeatmapBasicData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapBasicData";
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
#[cfg(feature = "BeatmapBasicData")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapBasicData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapBasicData")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapBasicData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapBasicData")]
impl crate::GlobalNamespace::BeatmapBasicData {
    pub fn New(
        noteJumpMovementSpeed: f32,
        noteJumpStartBeatOffset: f32,
        environmentName: crate::GlobalNamespace::EnvironmentName,
        beatmapColorScheme: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorScheme,
        >,
        notesCount: i32,
        cuttableObjectsCount: i32,
        obstaclesCount: i32,
        bombsCount: i32,
        mappers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        lighters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    noteJumpMovementSpeed,
                    noteJumpStartBeatOffset,
                    environmentName,
                    beatmapColorScheme,
                    notesCount,
                    cuttableObjectsCount,
                    obstaclesCount,
                    bombsCount,
                    mappers,
                    lighters,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        noteJumpMovementSpeed: f32,
        noteJumpStartBeatOffset: f32,
        environmentName: crate::GlobalNamespace::EnvironmentName,
        beatmapColorScheme: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorScheme,
        >,
        notesCount: i32,
        cuttableObjectsCount: i32,
        obstaclesCount: i32,
        bombsCount: i32,
        mappers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        lighters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            f32,
                            f32,
                            crate::GlobalNamespace::EnvironmentName,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::ColorScheme,
                            >,
                            i32,
                            i32,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppString,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppString,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        10usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            10usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        noteJumpMovementSpeed,
                        noteJumpStartBeatOffset,
                        environmentName,
                        beatmapColorScheme,
                        notesCount,
                        cuttableObjectsCount,
                        obstaclesCount,
                        bombsCount,
                        mappers,
                        lighters,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BeatmapBasicData")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BeatmapBasicData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
