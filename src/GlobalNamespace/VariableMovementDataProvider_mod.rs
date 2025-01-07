#[cfg(feature = "VariableMovementDataProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct VariableMovementDataProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _wasUpdatedThisFrame_k__BackingField: bool,
    pub _playerTransforms: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerTransforms,
    >,
    pub _beatmapCallbacksController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapCallbacksController,
    >,
    pub _njsDataCallbackWrapper: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapDataCallbackWrapper,
    >,
    pub _targetNoteJumpMovementSpeed: f32,
    pub _noteJumpMovementSpeed: f32,
    pub _prevNoteJumpMovementSpeed: f32,
    pub _jumpDistance: f32,
    pub _halfJumpDistance: f32,
    pub _jumpDuration: f32,
    pub _halfJumpDuration: f32,
    pub _waitingDuration: f32,
    pub _moveStartPosition: crate::UnityEngine::Vector3,
    pub _moveEndPosition: crate::UnityEngine::Vector3,
    pub _jumpEndPosition: crate::UnityEngine::Vector3,
    pub _noteJumpValueType: crate::GlobalNamespace::BeatmapObjectSpawnMovementData_NoteJumpValueType,
    pub _halfJumpDurationInBeats: f32,
    pub _initOneBeatDuration: f32,
    pub _initNoteJumpMovementSpeed: f32,
    pub _spawnAheadTime: f32,
    pub _centerPosition: crate::UnityEngine::Vector3,
    pub _forwardVector: crate::UnityEngine::Vector3,
    pub _relativeNoteJumpSpeedInterpolation: crate::GlobalNamespace::VariableMovementDataProvider_InterpolationData,
}
#[cfg(feature = "VariableMovementDataProvider")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::VariableMovementDataProvider {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "VariableMovementDataProvider";
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
#[cfg(feature = "VariableMovementDataProvider")]
impl std::ops::Deref for crate::GlobalNamespace::VariableMovementDataProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "VariableMovementDataProvider")]
impl std::ops::DerefMut for crate::GlobalNamespace::VariableMovementDataProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "VariableMovementDataProvider")]
impl crate::GlobalNamespace::VariableMovementDataProvider {
    pub const kInitMoveDistance: f32 = 200f32;
    pub const kMinNoteJumpMovementSpeed: f32 = 0.01f32;
    pub const kMoveDuration: f32 = 0.5f32;
    #[cfg(feature = "VariableMovementDataProvider+InterpolationData")]
    pub type InterpolationData = crate::GlobalNamespace::VariableMovementDataProvider_InterpolationData;
    pub fn CalculateCurrentNoteJumpGravity(
        &mut self,
        gravityBase: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("CalculateCurrentNoteJumpGravity", (gravityBase))?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleNoteJumpMovementSpeedEvent(
        &mut self,
        currentEventData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteJumpSpeedEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteJumpMovementSpeedEvent", (currentEventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        startHalfJumpDurationInBeats: f32,
        maxHalfJumpDistance: f32,
        noteJumpMovementSpeed: f32,
        minRelativeNoteJumpSpeed: f32,
        bpm: f32,
        noteJumpValueType: crate::GlobalNamespace::BeatmapObjectSpawnMovementData_NoteJumpValueType,
        noteJumpValue: f32,
        centerPosition: crate::UnityEngine::Vector3,
        forwardVector: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
                (
                    startHalfJumpDurationInBeats,
                    maxHalfJumpDistance,
                    noteJumpMovementSpeed,
                    minRelativeNoteJumpSpeed,
                    bpm,
                    noteJumpValueType,
                    noteJumpValue,
                    centerPosition,
                    forwardVector,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn JumpPosYForLineLayerAtDistanceFromPlayerWithoutJumpOffset(
        &mut self,
        highestJumpPosY: f32,
        distanceFromPlayer: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke(
                "JumpPosYForLineLayerAtDistanceFromPlayerWithoutJumpOffset",
                (highestJumpPosY, distanceFromPlayer),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ManualUpdate(
        &mut self,
        songTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ManualUpdate", (songTime))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        playerTransforms: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerTransforms,
        >,
        beatmapCallbacksController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCallbacksController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (playerTransforms, beatmapCallbacksController))?;
        Ok(__cordl_object.into())
    }
    pub fn NoteJumpGravityForLineLayerWithoutJumpOffset(
        &mut self,
        highestJumpPosY: f32,
        beforeJumpLineLayer: crate::GlobalNamespace::NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke(
                "NoteJumpGravityForLineLayerWithoutJumpOffset",
                (highestJumpPosY, beforeJumpLineLayer),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        playerTransforms: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerTransforms,
        >,
        beatmapCallbacksController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCallbacksController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (playerTransforms, beatmapCallbacksController))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_halfJumpDuration(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_halfJumpDuration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_jumpDistance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_jumpDistance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_jumpDuration(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_jumpDuration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_jumpEndPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_jumpEndPosition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_moveDuration(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_moveDuration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_moveEndPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_moveEndPosition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_moveStartPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_moveStartPosition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_noteJumpSpeed(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_noteJumpSpeed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_spawnAheadTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_spawnAheadTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_targetNoteJumpSpeed(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_targetNoteJumpSpeed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_waitingDuration(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_waitingDuration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wasUpdatedThisFrame(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_wasUpdatedThisFrame", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_wasUpdatedThisFrame(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_wasUpdatedThisFrame", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "VariableMovementDataProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::VariableMovementDataProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "VariableMovementDataProvider")]
impl AsRef<crate::GlobalNamespace::IVariableMovementDataProvider>
for crate::GlobalNamespace::VariableMovementDataProvider {
    fn as_ref(&self) -> &crate::GlobalNamespace::IVariableMovementDataProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "VariableMovementDataProvider")]
impl AsMut<crate::GlobalNamespace::IVariableMovementDataProvider>
for crate::GlobalNamespace::VariableMovementDataProvider {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IVariableMovementDataProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "VariableMovementDataProvider")]
impl AsRef<crate::System::IDisposable>
for crate::GlobalNamespace::VariableMovementDataProvider {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "VariableMovementDataProvider")]
impl AsMut<crate::System::IDisposable>
for crate::GlobalNamespace::VariableMovementDataProvider {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "VariableMovementDataProvider+InterpolationData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct VariableMovementDataProvider_InterpolationData {
    pub _fromValue: f32,
    pub _toValue: f32,
    pub _fromTime: f32,
    pub _duration: f32,
    pub _easeType: crate::GlobalNamespace::EaseType,
}
#[cfg(feature = "VariableMovementDataProvider+InterpolationData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::VariableMovementDataProvider_InterpolationData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "InterpolationData";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::VariableMovementDataProvider_InterpolationData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::VariableMovementDataProvider_InterpolationData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::VariableMovementDataProvider_InterpolationData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::VariableMovementDataProvider_InterpolationData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "VariableMovementDataProvider+InterpolationData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::VariableMovementDataProvider_InterpolationData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "VariableMovementDataProvider+InterpolationData")]
impl crate::GlobalNamespace::VariableMovementDataProvider_InterpolationData {
    pub fn GetValue(&mut self, _cordl_time: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetValue",
            (_cordl_time),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetValues(
        &mut self,
        fromValue: f32,
        toValue: f32,
        fromTime: f32,
        toTime: f32,
        easeType: crate::GlobalNamespace::EaseType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetValues",
            (fromValue, toValue, fromTime, toTime, easeType),
        )?;
        Ok(__cordl_ret.into())
    }
}
