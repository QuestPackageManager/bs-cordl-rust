#[cfg(feature = "BeatmapObjectSpawnController")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapObjectSpawnController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _beatmapObjectSpawnMovementData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapObjectSpawnMovementData,
    >,
    pub _beatmapCallbacksController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapCallbacksController,
    >,
    pub _beatmapObjectSpawner: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IBeatmapObjectSpawner,
    >,
    pub _jumpOffsetYProvider: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IJumpOffsetYProvider,
    >,
    pub _variableMovementDataProvider: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IVariableMovementDataProvider,
    >,
    pub _beatmapData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IReadonlyBeatmapData,
    >,
    pub _initData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapObjectSpawnController_InitData,
    >,
    pub didInitEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub _disableSpawning: bool,
    pub _isInitialized: bool,
    pub _obstacleDataCallbackWrapper: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapDataCallbackWrapper,
    >,
    pub _noteDataCallbackWrapper: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapDataCallbackWrapper,
    >,
    pub _sliderDataCallbackWrapper: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapDataCallbackWrapper,
    >,
}
#[cfg(feature = "BeatmapObjectSpawnController")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatmapObjectSpawnController {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapObjectSpawnController";
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
#[cfg(feature = "BeatmapObjectSpawnController")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapObjectSpawnController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectSpawnController")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapObjectSpawnController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectSpawnController")]
impl crate::GlobalNamespace::BeatmapObjectSpawnController {
    #[cfg(feature = "BeatmapObjectSpawnController+InitData")]
    pub type InitData = crate::GlobalNamespace::BeatmapObjectSpawnController_InitData;
    pub fn HandleNoteDataCallback(
        &mut self,
        noteData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::BeatmapObjectSpawnController as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleNoteDataCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::BeatmapObjectSpawnController as
                    quest_hook::libil2cpp::Type > ::class(), "HandleNoteDataCallback",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (noteData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleObstacleDataCallback(
        &mut self,
        obstacleData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ObstacleData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::BeatmapObjectSpawnController as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ObstacleData>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleObstacleDataCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::BeatmapObjectSpawnController as
                    quest_hook::libil2cpp::Type > ::class(),
                    "HandleObstacleDataCallback", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (obstacleData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleSliderDataCallback(
        &mut self,
        sliderNoteData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::BeatmapObjectSpawnController as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderData>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleSliderDataCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::BeatmapObjectSpawnController as
                    quest_hook::libil2cpp::Type > ::class(), "HandleSliderDataCallback",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (sliderNoteData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::BeatmapObjectSpawnController as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnDestroy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::BeatmapObjectSpawnController as
                    quest_hook::libil2cpp::Type > ::class(), "OnDestroy", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::BeatmapObjectSpawnController as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Start")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::BeatmapObjectSpawnController as
                    quest_hook::libil2cpp::Type > ::class(), "Start", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StopSpawning(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::BeatmapObjectSpawnController as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("StopSpawning")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::BeatmapObjectSpawnController as
                    quest_hook::libil2cpp::Type > ::class(), "StopSpawning", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::BeatmapObjectSpawnController as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::BeatmapObjectSpawnController as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_didInitEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::BeatmapObjectSpawnController as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_didInitEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::BeatmapObjectSpawnController as
                    quest_hook::libil2cpp::Type > ::class(), "add_didInitEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_beatmapObjectSpawnMovementData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapObjectSpawnMovementData>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::BeatmapObjectSpawnController as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BeatmapObjectSpawnMovementData,
                >,
                0usize,
            >("get_beatmapObjectSpawnMovementData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::BeatmapObjectSpawnController as
                    quest_hook::libil2cpp::Type > ::class(),
                    "get_beatmapObjectSpawnMovementData", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapObjectSpawnMovementData,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isInitialized(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::BeatmapObjectSpawnController as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isInitialized")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::BeatmapObjectSpawnController as
                    quest_hook::libil2cpp::Type > ::class(), "get_isInitialized", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_noteLinesCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::BeatmapObjectSpawnController as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_noteLinesCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::BeatmapObjectSpawnController as
                    quest_hook::libil2cpp::Type > ::class(), "get_noteLinesCount", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_noteLinesDistance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::BeatmapObjectSpawnController as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_noteLinesDistance")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::BeatmapObjectSpawnController as
                    quest_hook::libil2cpp::Type > ::class(), "get_noteLinesDistance",
                    0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_verticalLayerDistance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::BeatmapObjectSpawnController as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_verticalLayerDistance")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::BeatmapObjectSpawnController as
                    quest_hook::libil2cpp::Type > ::class(), "get_verticalLayerDistance",
                    0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn remove_didInitEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::BeatmapObjectSpawnController as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_didInitEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::BeatmapObjectSpawnController as
                    quest_hook::libil2cpp::Type > ::class(), "remove_didInitEvent",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapObjectSpawnController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapObjectSpawnController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapObjectSpawnController+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapObjectSpawnController_InitData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub beatsPerMinute: f32,
    pub noteLinesCount: i32,
    pub noteJumpMovementSpeed: f32,
    pub noteJumpValueType: crate::GlobalNamespace::BeatmapObjectSpawnMovementData_NoteJumpValueType,
    pub noteJumpValue: f32,
}
#[cfg(feature = "BeatmapObjectSpawnController+InitData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatmapObjectSpawnController_InitData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapObjectSpawnController/InitData";
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
#[cfg(feature = "BeatmapObjectSpawnController+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapObjectSpawnController_InitData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectSpawnController+InitData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BeatmapObjectSpawnController_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectSpawnController+InitData")]
impl crate::GlobalNamespace::BeatmapObjectSpawnController_InitData {
    pub fn New(
        beatsPerMinute: f32,
        noteLinesCount: i32,
        noteJumpMovementSpeed: f32,
        noteJumpValueType: crate::GlobalNamespace::BeatmapObjectSpawnMovementData_NoteJumpValueType,
        noteJumpValue: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    beatsPerMinute,
                    noteLinesCount,
                    noteJumpMovementSpeed,
                    noteJumpValueType,
                    noteJumpValue,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        beatsPerMinute: f32,
        noteLinesCount: i32,
        noteJumpMovementSpeed: f32,
        noteJumpValueType: crate::GlobalNamespace::BeatmapObjectSpawnMovementData_NoteJumpValueType,
        noteJumpValue: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::BeatmapObjectSpawnController_InitData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    f32,
                    i32,
                    f32,
                    crate::GlobalNamespace::BeatmapObjectSpawnMovementData_NoteJumpValueType,
                    f32,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::BeatmapObjectSpawnController_InitData as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        beatsPerMinute,
                        noteLinesCount,
                        noteJumpMovementSpeed,
                        noteJumpValueType,
                        noteJumpValue,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapObjectSpawnController+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapObjectSpawnController_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
