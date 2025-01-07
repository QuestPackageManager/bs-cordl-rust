#[cfg(feature = "IVariableMovementDataProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct IVariableMovementDataProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IVariableMovementDataProvider")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::IVariableMovementDataProvider {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "IVariableMovementDataProvider";
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
#[cfg(feature = "IVariableMovementDataProvider")]
impl std::ops::Deref for crate::GlobalNamespace::IVariableMovementDataProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IVariableMovementDataProvider")]
impl std::ops::DerefMut for crate::GlobalNamespace::IVariableMovementDataProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IVariableMovementDataProvider")]
impl crate::GlobalNamespace::IVariableMovementDataProvider {
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
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
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
}
#[cfg(feature = "IVariableMovementDataProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IVariableMovementDataProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
