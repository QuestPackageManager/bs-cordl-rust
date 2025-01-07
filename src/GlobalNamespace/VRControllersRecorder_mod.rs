#[cfg(feature = "VRControllersRecorder+SavedData+KeyframeSerializable")]
#[repr(C)]
#[derive(Debug)]
pub struct SavedData_VRControllersRecorder_KeyframeSerializable {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _xPos1: f32,
    pub _yPos1: f32,
    pub _zPos1: f32,
    pub _xPos2: f32,
    pub _yPos2: f32,
    pub _zPos2: f32,
    pub _xPos3: f32,
    pub _yPos3: f32,
    pub _zPos3: f32,
    pub _xRot1: f32,
    pub _yRot1: f32,
    pub _zRot1: f32,
    pub _wRot1: f32,
    pub _xRot2: f32,
    pub _yRot2: f32,
    pub _zRot2: f32,
    pub _wRot2: f32,
    pub _xRot3: f32,
    pub _yRot3: f32,
    pub _zRot3: f32,
    pub _wRot3: f32,
    pub _time: f32,
}
#[cfg(feature = "VRControllersRecorder+SavedData+KeyframeSerializable")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::SavedData_VRControllersRecorder_KeyframeSerializable {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "KeyframeSerializable";
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
#[cfg(feature = "VRControllersRecorder+SavedData+KeyframeSerializable")]
impl std::ops::Deref
for crate::GlobalNamespace::SavedData_VRControllersRecorder_KeyframeSerializable {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "VRControllersRecorder+SavedData+KeyframeSerializable")]
impl std::ops::DerefMut
for crate::GlobalNamespace::SavedData_VRControllersRecorder_KeyframeSerializable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "VRControllersRecorder+SavedData+KeyframeSerializable")]
impl crate::GlobalNamespace::SavedData_VRControllersRecorder_KeyframeSerializable {
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "VRControllersRecorder+SavedData+KeyframeSerializable")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SavedData_VRControllersRecorder_KeyframeSerializable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "VRControllersRecorder")]
#[repr(C)]
#[derive(Debug)]
pub struct VRControllersRecorder {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _recordingTextAsset: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset>,
    pub _recordingFileName: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _mode: crate::GlobalNamespace::VRControllersRecorder_Mode,
    pub _dontMoveHead: bool,
    pub _changeToNonVRCamera: bool,
    pub _adjustSabersPositionBasedOnHeadPosition: bool,
    pub _headRotationOffset: crate::UnityEngine::Vector3,
    pub _headPositionOffset: crate::UnityEngine::Vector3,
    pub _headSmooth: f32,
    pub _cameraFOV: f32,
    pub _controllersTimeOffset: f32,
    pub _controllersSmooth: f32,
    pub _controller0: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VRController>,
    pub _controller1: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VRController>,
    pub _headTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    pub _recorderCamera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    pub _spawnRotationTransform: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Transform,
    >,
    pub _originTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _audioTimeSyncController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AudioTimeSyncController,
    >,
    pub _controller0Transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _controller1Transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _hasHead: bool,
    pub _hasController0: bool,
    pub _hasController1: bool,
    pub _state: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::VRControllersRecorder_State,
    >,
    pub _keyframeIndex: i32,
}
#[cfg(feature = "VRControllersRecorder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::VRControllersRecorder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "VRControllersRecorder";
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
#[cfg(feature = "VRControllersRecorder")]
impl std::ops::Deref for crate::GlobalNamespace::VRControllersRecorder {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "VRControllersRecorder")]
impl std::ops::DerefMut for crate::GlobalNamespace::VRControllersRecorder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "VRControllersRecorder")]
impl crate::GlobalNamespace::VRControllersRecorder {
    pub const kHeaderChar: char = 'v';
    pub const kRecorderVersion: u32 = 30209u32;
    pub const kSavedDataRecorderVersion: u32 = 1979777280u32;
    pub const kSavedDataWithOffsetRecorderVersion: u32 = 7733505u32;
    #[cfg(feature = "VRControllersRecorder+Keyframe")]
    pub type Keyframe = crate::GlobalNamespace::VRControllersRecorder_Keyframe;
    #[cfg(feature = "VRControllersRecorder+Mode")]
    pub type Mode = crate::GlobalNamespace::VRControllersRecorder_Mode;
    #[cfg(feature = "VRControllersRecorder+SavedData")]
    pub type SavedData = crate::GlobalNamespace::VRControllersRecorder_SavedData;
    #[cfg(feature = "VRControllersRecorder+State")]
    pub type State = crate::GlobalNamespace::VRControllersRecorder_State;
    #[cfg(feature = "VRControllersRecorder+TypeSerializationBinder")]
    pub type TypeSerializationBinder = crate::GlobalNamespace::VRControllersRecorder_TypeSerializationBinder;
    pub fn CheckNodes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckNodes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateAnimationClipFromRecording(
        recordingFilePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationClip>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationClip> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateAnimationClipFromRecording", (recordingFilePath))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBinaryFormatter() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::BinaryFormatter,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::BinaryFormatter,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBinaryFormatter", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSavedData(
        keyframes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::VRControllersRecorder_Keyframe,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::VRControllersRecorder_SavedData,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::VRControllersRecorder_SavedData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSavedData", (keyframes))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleControllerAnchorUpdate(
        &mut self,
        controller: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VRController>,
        poseOffset: crate::UnityEngine::Pose,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleControllerAnchorUpdate", (controller, poseOffset))?;
        Ok(__cordl_ret.into())
    }
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Load(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Load", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadSavedData(
        dataStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::VRControllersRecorder_SavedData,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::VRControllersRecorder_SavedData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadSavedData", (dataStream))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadStateWithDefaultOffset(
        stream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VRControllersRecorder_State>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::VRControllersRecorder_State,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadStateWithDefaultOffset", (stream))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadState_Pose_Pose1(
        stream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        leftControllerOffset: crate::UnityEngine::Pose,
        rightControllerOffset: crate::UnityEngine::Pose,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VRControllersRecorder_State>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::VRControllersRecorder_State,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadState", (stream, leftControllerOffset, rightControllerOffset))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadState_Stream0(
        &mut self,
        stream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VRControllersRecorder_State>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::VRControllersRecorder_State,
        > = __cordl_object.invoke("LoadState", (stream))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PlaybackTick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PlaybackTick", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadHeader(
        reader: quest_hook::libil2cpp::Gc<crate::System::IO::BinaryReader>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadHeader", (reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn RecordTick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecordTick", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Save(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Save", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SerializeSavedData(
        dataStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        savedData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::VRControllersRecorder_SavedData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SerializeSavedData", (dataStream, savedData))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDefaultSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDefaultSettings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetInGamePlaybackDefaultSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetInGamePlaybackDefaultSettings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPositionAndRotation(
        &mut self,
        transf: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        targetPos: crate::UnityEngine::Vector3,
        targetRot: crate::UnityEngine::Quaternion,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPositionAndRotation", (transf, targetPos, targetRot, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteHeader(
        writer: quest_hook::libil2cpp::Gc<crate::System::IO::BinaryWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteHeader", (writer))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteTo(
        writer: quest_hook::libil2cpp::Gc<crate::System::IO::BinaryWriter>,
        state: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::VRControllersRecorder_State,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteTo", (writer, state))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_changeToNonVRCamera(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_changeToNonVRCamera", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_currentPoses(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ValueTuple_3<
            crate::UnityEngine::Pose,
            crate::UnityEngine::Pose,
            crate::UnityEngine::Pose,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::ValueTuple_3<
            crate::UnityEngine::Pose,
            crate::UnityEngine::Pose,
            crate::UnityEngine::Pose,
        > = __cordl_object.invoke("get_currentPoses", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_mode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::VRControllersRecorder_Mode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::VRControllersRecorder_Mode = __cordl_object
            .invoke("get_mode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_path(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_path", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_recordingFileName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_recordingFileName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_recordingTextAsset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset> = __cordl_object
            .invoke("get_recordingTextAsset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_changeToNonVRCamera(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_changeToNonVRCamera", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_mode(
        &mut self,
        value: crate::GlobalNamespace::VRControllersRecorder_Mode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_mode", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_recordingFileName(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_recordingFileName", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_recordingTextAsset(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_recordingTextAsset", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "VRControllersRecorder")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::VRControllersRecorder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "VRControllersRecorder+Keyframe")]
#[repr(C)]
#[derive(Debug)]
pub struct VRControllersRecorder_Keyframe {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub pos1: crate::UnityEngine::Vector3,
    pub pos2: crate::UnityEngine::Vector3,
    pub pos3: crate::UnityEngine::Vector3,
    pub rot1: crate::UnityEngine::Quaternion,
    pub rot2: crate::UnityEngine::Quaternion,
    pub rot3: crate::UnityEngine::Quaternion,
    pub _cordl_time: f32,
}
#[cfg(feature = "VRControllersRecorder+Keyframe")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::VRControllersRecorder_Keyframe {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "Keyframe";
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
#[cfg(feature = "VRControllersRecorder+Keyframe")]
impl std::ops::Deref for crate::GlobalNamespace::VRControllersRecorder_Keyframe {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "VRControllersRecorder+Keyframe")]
impl std::ops::DerefMut for crate::GlobalNamespace::VRControllersRecorder_Keyframe {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "VRControllersRecorder+Keyframe")]
impl crate::GlobalNamespace::VRControllersRecorder_Keyframe {
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_VRControllersRecorder_Keyframe0(
        &mut self,
        other: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::VRControllersRecorder_Keyframe,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromSerializable(
        ks: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SavedData_VRControllersRecorder_KeyframeSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VRControllersRecorder_Keyframe>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::VRControllersRecorder_Keyframe,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromSerializable", (ks))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Pose_Pose_Pose_f32_0(
        leftController: crate::UnityEngine::Pose,
        rightController: crate::UnityEngine::Pose,
        head: crate::UnityEngine::Pose,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (leftController, rightController, head, _cordl_time))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Vector3_Vector3_Vector3_Quaternion_Quaternion_Quaternion_f32_1(
        pos1: crate::UnityEngine::Vector3,
        pos2: crate::UnityEngine::Vector3,
        pos3: crate::UnityEngine::Vector3,
        rot1: crate::UnityEngine::Quaternion,
        rot2: crate::UnityEngine::Quaternion,
        rot3: crate::UnityEngine::Quaternion,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pos1, pos2, pos3, rot1, rot2, rot3, _cordl_time))?;
        Ok(__cordl_object.into())
    }
    pub fn ReadFrom(
        reader: quest_hook::libil2cpp::Gc<crate::System::IO::BinaryReader>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VRControllersRecorder_Keyframe>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::VRControllersRecorder_Keyframe,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ReadFrom", (reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToConstructorString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToConstructorString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSerializable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SavedData_VRControllersRecorder_KeyframeSerializable,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SavedData_VRControllersRecorder_KeyframeSerializable,
        > = __cordl_object.invoke("ToSerializable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteTo(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::System::IO::BinaryWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteTo", (writer))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Pose_Pose_Pose_f32_0(
        &mut self,
        leftController: crate::UnityEngine::Pose,
        rightController: crate::UnityEngine::Pose,
        head: crate::UnityEngine::Pose,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (leftController, rightController, head, _cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Vector3_Vector3_Vector3_Quaternion_Quaternion_Quaternion_f32_1(
        &mut self,
        pos1: crate::UnityEngine::Vector3,
        pos2: crate::UnityEngine::Vector3,
        pos3: crate::UnityEngine::Vector3,
        rot1: crate::UnityEngine::Quaternion,
        rot2: crate::UnityEngine::Quaternion,
        rot3: crate::UnityEngine::Quaternion,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (pos1, pos2, pos3, rot1, rot2, rot3, _cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_headPose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Pose> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Pose = __cordl_object
            .invoke("get_headPose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_leftControllerPose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Pose> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Pose = __cordl_object
            .invoke("get_leftControllerPose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rightControllerPose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Pose> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Pose = __cordl_object
            .invoke("get_rightControllerPose", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "VRControllersRecorder+Keyframe")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::VRControllersRecorder_Keyframe {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "VRControllersRecorder+Keyframe")]
impl AsRef<crate::GlobalNamespace::IConstructorStringGenerator>
for crate::GlobalNamespace::VRControllersRecorder_Keyframe {
    fn as_ref(&self) -> &crate::GlobalNamespace::IConstructorStringGenerator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "VRControllersRecorder+Keyframe")]
impl AsMut<crate::GlobalNamespace::IConstructorStringGenerator>
for crate::GlobalNamespace::VRControllersRecorder_Keyframe {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IConstructorStringGenerator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "VRControllersRecorder+Keyframe")]
impl AsRef<
    crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VRControllersRecorder_Keyframe>,
    >,
> for crate::GlobalNamespace::VRControllersRecorder_Keyframe {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VRControllersRecorder_Keyframe>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "VRControllersRecorder+Keyframe")]
impl AsMut<
    crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VRControllersRecorder_Keyframe>,
    >,
> for crate::GlobalNamespace::VRControllersRecorder_Keyframe {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VRControllersRecorder_Keyframe>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "VRControllersRecorder+Mode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VRControllersRecorder_Mode {
    #[default]
    Off = 2i32,
    Playback = 1i32,
    Record = 0i32,
}
#[cfg(feature = "VRControllersRecorder+Mode")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::VRControllersRecorder_Mode {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "Mode";
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
for crate::GlobalNamespace::VRControllersRecorder_Mode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::VRControllersRecorder_Mode {
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
for crate::GlobalNamespace::VRControllersRecorder_Mode {
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
for crate::GlobalNamespace::VRControllersRecorder_Mode {
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
#[cfg(feature = "VRControllersRecorder+SavedData")]
#[repr(C)]
#[derive(Debug)]
pub struct VRControllersRecorder_SavedData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _keyframes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::SavedData_VRControllersRecorder_KeyframeSerializable,
            >,
        >,
    >,
}
#[cfg(feature = "VRControllersRecorder+SavedData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::VRControllersRecorder_SavedData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "SavedData";
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
#[cfg(feature = "VRControllersRecorder+SavedData")]
impl std::ops::Deref for crate::GlobalNamespace::VRControllersRecorder_SavedData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "VRControllersRecorder+SavedData")]
impl std::ops::DerefMut for crate::GlobalNamespace::VRControllersRecorder_SavedData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "VRControllersRecorder+SavedData")]
impl crate::GlobalNamespace::VRControllersRecorder_SavedData {
    #[cfg(feature = "VRControllersRecorder+SavedData+KeyframeSerializable")]
    pub type KeyframeSerializable = crate::GlobalNamespace::SavedData_VRControllersRecorder_KeyframeSerializable;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "VRControllersRecorder+SavedData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::VRControllersRecorder_SavedData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "VRControllersRecorder+State")]
#[repr(C)]
#[derive(Debug)]
pub struct VRControllersRecorder_State {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _keyframes: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::VRControllersRecorder_Keyframe,
            >,
        >,
    >,
    pub leftControllerOffset: crate::UnityEngine::Pose,
    pub rightControllerOffset: crate::UnityEngine::Pose,
    pub _version: u32,
}
#[cfg(feature = "VRControllersRecorder+State")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::VRControllersRecorder_State {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "State";
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
#[cfg(feature = "VRControllersRecorder+State")]
impl std::ops::Deref for crate::GlobalNamespace::VRControllersRecorder_State {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "VRControllersRecorder+State")]
impl std::ops::DerefMut for crate::GlobalNamespace::VRControllersRecorder_State {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "VRControllersRecorder+State")]
impl crate::GlobalNamespace::VRControllersRecorder_State {
    pub fn AddKeyframe(
        &mut self,
        keyframe: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::VRControllersRecorder_Keyframe,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddKeyframe", (keyframe))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromSavedData(
        data: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::VRControllersRecorder_SavedData,
        >,
        leftControllerOffset: crate::UnityEngine::Pose,
        rightControllerOffset: crate::UnityEngine::Pose,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VRControllersRecorder_State>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::VRControllersRecorder_State,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "FromSavedData",
                (data, leftControllerOffset, rightControllerOffset),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New_3() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_IEnumerable_1_Pose_Pose0(
        keyframes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::VRControllersRecorder_Keyframe,
                >,
            >,
        >,
        leftControllerOffset: crate::UnityEngine::Pose,
        rightControllerOffset: crate::UnityEngine::Pose,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (keyframes, leftControllerOffset, rightControllerOffset),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_List_1_Pose_Pose1(
        keyframes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::VRControllersRecorder_Keyframe,
                >,
            >,
        >,
        leftControllerOffset: crate::UnityEngine::Pose,
        rightControllerOffset: crate::UnityEngine::Pose,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (keyframes, leftControllerOffset, rightControllerOffset),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_List_1_Pose_Pose_u32_2(
        keyframes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::VRControllersRecorder_Keyframe,
                >,
            >,
        >,
        leftControllerOffset: crate::UnityEngine::Pose,
        rightControllerOffset: crate::UnityEngine::Pose,
        version: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (keyframes, leftControllerOffset, rightControllerOffset, version),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn ReadFrom(
        reader: quest_hook::libil2cpp::Gc<crate::System::IO::BinaryReader>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VRControllersRecorder_State>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::VRControllersRecorder_State,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ReadFrom", (reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn Upgrade(
        &mut self,
        newLeftControllerOffset: crate::UnityEngine::Pose,
        newRightControllerOffset: crate::UnityEngine::Pose,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VRControllersRecorder_State>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::VRControllersRecorder_State,
        > = __cordl_object
            .invoke("Upgrade", (newLeftControllerOffset, newRightControllerOffset))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteTo(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::System::IO::BinaryWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteTo", (writer))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_3(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IEnumerable_1_Pose_Pose0(
        &mut self,
        keyframes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::VRControllersRecorder_Keyframe,
                >,
            >,
        >,
        leftControllerOffset: crate::UnityEngine::Pose,
        rightControllerOffset: crate::UnityEngine::Pose,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (keyframes, leftControllerOffset, rightControllerOffset))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_List_1_Pose_Pose1(
        &mut self,
        keyframes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::VRControllersRecorder_Keyframe,
                >,
            >,
        >,
        leftControllerOffset: crate::UnityEngine::Pose,
        rightControllerOffset: crate::UnityEngine::Pose,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (keyframes, leftControllerOffset, rightControllerOffset))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_List_1_Pose_Pose_u32_2(
        &mut self,
        keyframes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::VRControllersRecorder_Keyframe,
                >,
            >,
        >,
        leftControllerOffset: crate::UnityEngine::Pose,
        rightControllerOffset: crate::UnityEngine::Pose,
        version: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (keyframes, leftControllerOffset, rightControllerOffset, version),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_keyframes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::VRControllersRecorder_Keyframe,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::VRControllersRecorder_Keyframe,
                >,
            >,
        > = __cordl_object.invoke("get_keyframes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_version(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("get_version", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "VRControllersRecorder+State")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::VRControllersRecorder_State {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "VRControllersRecorder+TypeSerializationBinder")]
#[repr(C)]
#[derive(Debug)]
pub struct VRControllersRecorder_TypeSerializationBinder {
    __cordl_parent: crate::System::Runtime::Serialization::SerializationBinder,
}
#[cfg(feature = "VRControllersRecorder+TypeSerializationBinder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::VRControllersRecorder_TypeSerializationBinder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "TypeSerializationBinder";
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
#[cfg(feature = "VRControllersRecorder+TypeSerializationBinder")]
impl std::ops::Deref
for crate::GlobalNamespace::VRControllersRecorder_TypeSerializationBinder {
    type Target = crate::System::Runtime::Serialization::SerializationBinder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "VRControllersRecorder+TypeSerializationBinder")]
impl std::ops::DerefMut
for crate::GlobalNamespace::VRControllersRecorder_TypeSerializationBinder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "VRControllersRecorder+TypeSerializationBinder")]
impl crate::GlobalNamespace::VRControllersRecorder_TypeSerializationBinder {
    pub fn BindToType(
        &mut self,
        assemblyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        typeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("BindToType", (assemblyName, typeName))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "VRControllersRecorder+TypeSerializationBinder")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::VRControllersRecorder_TypeSerializationBinder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
