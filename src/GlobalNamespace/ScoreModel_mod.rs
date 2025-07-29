#[cfg(feature = "cordl_class_ScoreModel")]
#[repr(C)]
#[derive(Debug)]
pub struct ScoreModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_ScoreModel")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::ScoreModel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ScoreModel";
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
#[cfg(feature = "ScoreModel")]
impl std::ops::Deref for crate::GlobalNamespace::ScoreModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ScoreModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::ScoreModel {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ScoreModel")]
impl crate::GlobalNamespace::ScoreModel {
    pub const kMaxAfterCutScore: i32 = 30i32;
    pub const kMaxBeforeCutScore: i32 = 70i32;
    pub const kMaxCenterDistanceCutScore: i32 = 15i32;
    pub const kMaxDistanceForDistanceToCenterScore: f32 = 0.3f32;
    pub const kMaxMultiplier: i32 = 8i32;
    #[cfg(feature = "ScoreModel+MaxScoreCounterElement")]
    pub type MaxScoreCounterElement = crate::GlobalNamespace::ScoreModel_MaxScoreCounterElement;
    #[cfg(feature = "ScoreModel+NoteScoreDefinition")]
    pub type NoteScoreDefinition = crate::GlobalNamespace::ScoreModel_NoteScoreDefinition;
    pub fn ComputeMaxMultipliedScoreForBeatmap(
        beatmapData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyBeatmapData,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::IReadonlyBeatmapData,
                        >),
                        i32,
                        1usize,
                    >("ComputeMaxMultipliedScoreForBeatmap")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ComputeMaxMultipliedScoreForBeatmap", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (beatmapData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeQuickInaccurateMaxMultipliedScoreForBeatmap(
        beatmapBasicData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapBasicData,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapBasicData,
                        >),
                        i32,
                        1usize,
                    >("ComputeQuickInaccurateMaxMultipliedScoreForBeatmap")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ComputeQuickInaccurateMaxMultipliedScoreForBeatmap", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (beatmapBasicData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetModifiedScoreForGameplayModifiersScoreMultiplier(
        multipliedScore: i32,
        gameplayModifiersScoreMultiplier: f32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32, f32),
                        i32,
                        2usize,
                    >("GetModifiedScoreForGameplayModifiersScoreMultiplier")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetModifiedScoreForGameplayModifiersScoreMultiplier", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (multipliedScore, gameplayModifiersScoreMultiplier),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetNoteScoreDefinition(
        scoringType: crate::GlobalNamespace::NoteData_ScoringType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ScoreModel_NoteScoreDefinition>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::NoteData_ScoringType),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::ScoreModel_NoteScoreDefinition,
                        >,
                        1usize,
                    >("GetNoteScoreDefinition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetNoteScoreDefinition", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ScoreModel_NoteScoreDefinition,
        > = unsafe { cordl_method_info.invoke_unchecked((), (scoringType))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_ScoreModel")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ScoreModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_ScoreModel+MaxScoreCounterElement")]
#[repr(C)]
#[derive(Debug)]
pub struct ScoreModel_MaxScoreCounterElement {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub noteScoreDefinition: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ScoreModel_NoteScoreDefinition,
    >,
    pub _cordl_time: f32,
}
#[cfg(feature = "cordl_class_ScoreModel+MaxScoreCounterElement")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ScoreModel_MaxScoreCounterElement {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ScoreModel/MaxScoreCounterElement";
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
#[cfg(feature = "ScoreModel+MaxScoreCounterElement")]
impl std::ops::Deref for crate::GlobalNamespace::ScoreModel_MaxScoreCounterElement {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ScoreModel+MaxScoreCounterElement")]
impl std::ops::DerefMut for crate::GlobalNamespace::ScoreModel_MaxScoreCounterElement {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ScoreModel+MaxScoreCounterElement")]
impl crate::GlobalNamespace::ScoreModel_MaxScoreCounterElement {
    pub fn CompareTo(
        &mut self,
        other: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ScoreModel_MaxScoreCounterElement,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::ScoreModel_MaxScoreCounterElement,
                        >),
                        i32,
                        1usize,
                    >("CompareTo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CompareTo", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (other))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        scoringType: crate::GlobalNamespace::NoteData_ScoringType,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (scoringType, _cordl_time))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        scoringType: crate::GlobalNamespace::NoteData_ScoringType,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::NoteData_ScoringType, f32),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (scoringType, _cordl_time))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_ScoreModel+MaxScoreCounterElement")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ScoreModel_MaxScoreCounterElement {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ScoreModel+MaxScoreCounterElement")]
impl AsRef<
    crate::System::IComparable_1<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ScoreModel_MaxScoreCounterElement,
        >,
    >,
> for crate::GlobalNamespace::ScoreModel_MaxScoreCounterElement {
    fn as_ref(
        &self,
    ) -> &crate::System::IComparable_1<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ScoreModel_MaxScoreCounterElement,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ScoreModel+MaxScoreCounterElement")]
impl AsMut<
    crate::System::IComparable_1<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ScoreModel_MaxScoreCounterElement,
        >,
    >,
> for crate::GlobalNamespace::ScoreModel_MaxScoreCounterElement {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ScoreModel_MaxScoreCounterElement,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "cordl_class_ScoreModel+NoteScoreDefinition")]
#[repr(C)]
#[derive(Debug)]
pub struct ScoreModel_NoteScoreDefinition {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub maxCenterDistanceCutScore: i32,
    pub minBeforeCutScore: i32,
    pub maxBeforeCutScore: i32,
    pub minAfterCutScore: i32,
    pub maxAfterCutScore: i32,
    pub fixedCutScore: i32,
}
#[cfg(feature = "cordl_class_ScoreModel+NoteScoreDefinition")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ScoreModel_NoteScoreDefinition {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ScoreModel/NoteScoreDefinition";
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
#[cfg(feature = "ScoreModel+NoteScoreDefinition")]
impl std::ops::Deref for crate::GlobalNamespace::ScoreModel_NoteScoreDefinition {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ScoreModel+NoteScoreDefinition")]
impl std::ops::DerefMut for crate::GlobalNamespace::ScoreModel_NoteScoreDefinition {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ScoreModel+NoteScoreDefinition")]
impl crate::GlobalNamespace::ScoreModel_NoteScoreDefinition {
    pub fn New(
        maxCenterDistanceCutScore: i32,
        minBeforeCutScore: i32,
        maxBeforeCutScore: i32,
        minAfterCutScore: i32,
        maxAfterCutScore: i32,
        fixedCutScore: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    maxCenterDistanceCutScore,
                    minBeforeCutScore,
                    maxBeforeCutScore,
                    minAfterCutScore,
                    maxAfterCutScore,
                    fixedCutScore,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        maxCenterDistanceCutScore: i32,
        minBeforeCutScore: i32,
        maxBeforeCutScore: i32,
        minAfterCutScore: i32,
        maxAfterCutScore: i32,
        fixedCutScore: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, i32, i32, i32, i32, i32),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        maxCenterDistanceCutScore,
                        minBeforeCutScore,
                        maxBeforeCutScore,
                        minAfterCutScore,
                        maxAfterCutScore,
                        fixedCutScore,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_executionOrder(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_executionOrder")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_executionOrder", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_maxCutScore(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_maxCutScore")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_maxCutScore", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_ScoreModel+NoteScoreDefinition")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ScoreModel_NoteScoreDefinition {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
