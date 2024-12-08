#[cfg(feature = "VRControllersRecorder+Keyframe")]
#[repr(C)]
#[derive(Debug)]
pub struct VRControllersRecorder_Keyframe {
    __cordl_parent: crate::System::Object,
    pub pos1: crate::UnityEngine::Vector3,
    pub pos2: crate::UnityEngine::Vector3,
    pub pos3: crate::UnityEngine::Vector3,
    pub rot1: crate::UnityEngine::Quaternion,
    pub rot2: crate::UnityEngine::Quaternion,
    pub rot3: crate::UnityEngine::Quaternion,
    pub _cordl_time: f32,
}
#[cfg(feature = "VRControllersRecorder+Keyframe")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::VRControllersRecorder_Keyframe
    => ""."VRControllersRecorder/Keyframe"
);
#[cfg(feature = "VRControllersRecorder+Keyframe")]
impl std::ops::Deref for crate::GlobalNamespace::VRControllersRecorder_Keyframe {
    type Target = crate::System::Object;
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
    pub fn Equals_Object1(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn Equals_VRControllersRecorder_Keyframe0(
        &mut self,
        other: *mut crate::GlobalNamespace::VRControllersRecorder_Keyframe,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (other))?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_Pose_Pose_Pose_f32_0(
        leftController: crate::UnityEngine::Pose,
        rightController: crate::UnityEngine::Pose,
        head: crate::UnityEngine::Pose,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (leftController, rightController, head, _cordl_time))?;
        Ok(__cordl_object)
    }
    pub fn New_Vector3_Vector3_Vector3_Quaternion_Quaternion_Quaternion_f32_1(
        pos1: crate::UnityEngine::Vector3,
        pos2: crate::UnityEngine::Vector3,
        pos3: crate::UnityEngine::Vector3,
        rot1: crate::UnityEngine::Quaternion,
        rot2: crate::UnityEngine::Quaternion,
        rot3: crate::UnityEngine::Quaternion,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pos1, pos2, pos3, rot1, rot2, rot3, _cordl_time))?;
        Ok(__cordl_object)
    }
    pub fn ToConstructorString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToConstructorString", ())?;
        Ok(__cordl_ret)
    }
    pub fn ToSerializable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::SavedData_KeyframeSerializable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::SavedData_KeyframeSerializable = __cordl_object
            .invoke("ToSerializable", ())?;
        Ok(__cordl_ret)
    }
    pub fn WriteTo(
        &mut self,
        writer: *mut crate::System::IO::BinaryWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteTo", (writer))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn get_headPose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Pose> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Pose = __cordl_object
            .invoke("get_headPose", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_leftControllerPose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Pose> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Pose = __cordl_object
            .invoke("get_leftControllerPose", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rightControllerPose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Pose> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Pose = __cordl_object
            .invoke("get_rightControllerPose", ())?;
        Ok(__cordl_ret)
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
#[cfg(feature = "VRControllersRecorder+SavedData+KeyframeSerializable")]
#[repr(C)]
#[derive(Debug)]
pub struct SavedData_KeyframeSerializable {
    __cordl_parent: crate::System::Object,
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SavedData_KeyframeSerializable
    => ""."VRControllersRecorder/SavedData/KeyframeSerializable"
);
#[cfg(feature = "VRControllersRecorder+SavedData+KeyframeSerializable")]
impl std::ops::Deref for crate::GlobalNamespace::SavedData_KeyframeSerializable {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "VRControllersRecorder+SavedData+KeyframeSerializable")]
impl std::ops::DerefMut for crate::GlobalNamespace::SavedData_KeyframeSerializable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "VRControllersRecorder+SavedData+KeyframeSerializable")]
impl crate::GlobalNamespace::SavedData_KeyframeSerializable {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "VRControllersRecorder+SavedData+KeyframeSerializable")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SavedData_KeyframeSerializable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "VRControllersRecorder+Mode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VRControllersRecorder_Mode {
    Off = 2i32,
    Playback = 1i32,
    Record = 0i32,
}
#[cfg(feature = "VRControllersRecorder+Mode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::VRControllersRecorder_Mode =>
    ""."VRControllersRecorder/Mode"
);
#[cfg(feature = "VRControllersRecorder+SavedData")]
#[repr(C)]
#[derive(Debug)]
pub struct VRControllersRecorder_SavedData {
    __cordl_parent: crate::System::Object,
    pub _keyframes: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::SavedData_KeyframeSerializable,
    >,
}
#[cfg(feature = "VRControllersRecorder+SavedData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::VRControllersRecorder_SavedData
    => ""."VRControllersRecorder/SavedData"
);
#[cfg(feature = "VRControllersRecorder+SavedData")]
impl std::ops::Deref for crate::GlobalNamespace::VRControllersRecorder_SavedData {
    type Target = crate::System::Object;
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
    pub type KeyframeSerializable = crate::GlobalNamespace::SavedData_KeyframeSerializable;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
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
    __cordl_parent: crate::System::Object,
    pub _keyframes: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::VRControllersRecorder_Keyframe,
    >,
    pub leftControllerOffset: crate::UnityEngine::Pose,
    pub rightControllerOffset: crate::UnityEngine::Pose,
    pub _version: u32,
}
#[cfg(feature = "VRControllersRecorder+State")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::VRControllersRecorder_State =>
    ""."VRControllersRecorder/State"
);
#[cfg(feature = "VRControllersRecorder+State")]
impl std::ops::Deref for crate::GlobalNamespace::VRControllersRecorder_State {
    type Target = crate::System::Object;
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
    #[cfg(feature = "VRControllersRecorder+State+__c")]
    pub type __c = crate::GlobalNamespace::State___c;
    pub fn AddKeyframe(
        &mut self,
        keyframe: *mut crate::GlobalNamespace::VRControllersRecorder_Keyframe,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddKeyframe", (keyframe))?;
        Ok(__cordl_ret)
    }
    pub fn New_3() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_IEnumerable_1_Pose_Pose0(
        keyframes: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::GlobalNamespace::VRControllersRecorder_Keyframe,
        >,
        leftControllerOffset: crate::UnityEngine::Pose,
        rightControllerOffset: crate::UnityEngine::Pose,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (keyframes, leftControllerOffset, rightControllerOffset),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_List_1_Pose_Pose1(
        keyframes: *mut crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::VRControllersRecorder_Keyframe,
        >,
        leftControllerOffset: crate::UnityEngine::Pose,
        rightControllerOffset: crate::UnityEngine::Pose,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (keyframes, leftControllerOffset, rightControllerOffset),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_List_1_Pose_Pose_u32_2(
        keyframes: *mut crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::VRControllersRecorder_Keyframe,
        >,
        leftControllerOffset: crate::UnityEngine::Pose,
        rightControllerOffset: crate::UnityEngine::Pose,
        version: u32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (keyframes, leftControllerOffset, rightControllerOffset, version),
            )?;
        Ok(__cordl_object)
    }
    pub fn Upgrade(
        &mut self,
        newLeftControllerOffset: crate::UnityEngine::Pose,
        newRightControllerOffset: crate::UnityEngine::Pose,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::VRControllersRecorder_State,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::VRControllersRecorder_State = __cordl_object
            .invoke("Upgrade", (newLeftControllerOffset, newRightControllerOffset))?;
        Ok(__cordl_ret)
    }
    pub fn WriteTo(
        &mut self,
        writer: *mut crate::System::IO::BinaryWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteTo", (writer))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_3(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IEnumerable_1_Pose_Pose0(
        &mut self,
        keyframes: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::GlobalNamespace::VRControllersRecorder_Keyframe,
        >,
        leftControllerOffset: crate::UnityEngine::Pose,
        rightControllerOffset: crate::UnityEngine::Pose,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (keyframes, leftControllerOffset, rightControllerOffset))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_List_1_Pose_Pose1(
        &mut self,
        keyframes: *mut crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::VRControllersRecorder_Keyframe,
        >,
        leftControllerOffset: crate::UnityEngine::Pose,
        rightControllerOffset: crate::UnityEngine::Pose,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (keyframes, leftControllerOffset, rightControllerOffset))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_List_1_Pose_Pose_u32_2(
        &mut self,
        keyframes: *mut crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::VRControllersRecorder_Keyframe,
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
        Ok(__cordl_ret)
    }
    pub fn get_keyframes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::GlobalNamespace::VRControllersRecorder_Keyframe,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::GlobalNamespace::VRControllersRecorder_Keyframe,
        > = __cordl_object.invoke("get_keyframes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_version(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("get_version", ())?;
        Ok(__cordl_ret)
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::VRControllersRecorder_TypeSerializationBinder => ""
    ."VRControllersRecorder/TypeSerializationBinder"
);
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
        assemblyName: *mut crate::System::String,
        typeName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("BindToType", (assemblyName, typeName))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
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
#[cfg(feature = "VRControllersRecorder")]
#[repr(C)]
#[derive(Debug)]
pub struct VRControllersRecorder {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _recordingTextAsset: *mut crate::UnityEngine::TextAsset,
    pub _recordingFileName: *mut crate::System::String,
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
    pub _controller0: *mut VRController,
    pub _controller1: *mut VRController,
    pub _headTransform: *mut crate::UnityEngine::Transform,
    pub _camera: *mut crate::UnityEngine::Camera,
    pub _recorderCamera: *mut crate::UnityEngine::Camera,
    pub _spawnRotationTransform: *mut crate::UnityEngine::Transform,
    pub _originTransform: *mut crate::UnityEngine::Transform,
    pub _audioTimeSyncController: *mut AudioTimeSyncController,
    pub _controller0Transform: *mut crate::UnityEngine::Transform,
    pub _controller1Transform: *mut crate::UnityEngine::Transform,
    pub _hasHead: bool,
    pub _hasController0: bool,
    pub _hasController1: bool,
    pub _state: *mut crate::GlobalNamespace::VRControllersRecorder_State,
    pub _keyframeIndex: i32,
}
#[cfg(feature = "VRControllersRecorder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for VRControllersRecorder => ""."VRControllersRecorder"
);
#[cfg(feature = "VRControllersRecorder")]
impl std::ops::Deref for VRControllersRecorder {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "VRControllersRecorder")]
impl std::ops::DerefMut for VRControllersRecorder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "VRControllersRecorder")]
impl VRControllersRecorder {
    pub const kHeaderChar: char = 'v';
    pub const kRecorderVersion: u32 = 30209u32;
    pub const kSavedDataRecorderVersion: u32 = 1979777280u32;
    pub const kSavedDataWithOffsetRecorderVersion: u32 = 7733505u32;
    #[cfg(feature = "VRControllersRecorder+TypeSerializationBinder")]
    pub type TypeSerializationBinder = crate::GlobalNamespace::VRControllersRecorder_TypeSerializationBinder;
    #[cfg(feature = "VRControllersRecorder+Keyframe")]
    pub type Keyframe = crate::GlobalNamespace::VRControllersRecorder_Keyframe;
    #[cfg(feature = "VRControllersRecorder+SavedData")]
    pub type SavedData = crate::GlobalNamespace::VRControllersRecorder_SavedData;
    #[cfg(feature = "VRControllersRecorder+State")]
    pub type State = crate::GlobalNamespace::VRControllersRecorder_State;
    #[cfg(feature = "VRControllersRecorder+Mode")]
    pub type Mode = crate::GlobalNamespace::VRControllersRecorder_Mode;
    pub fn CheckNodes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckNodes", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleControllerAnchorUpdate(
        &mut self,
        controller: *mut VRController,
        poseOffset: crate::UnityEngine::Pose,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleControllerAnchorUpdate", (controller, poseOffset))?;
        Ok(__cordl_ret)
    }
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn Load(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Load", ())?;
        Ok(__cordl_ret)
    }
    pub fn LoadState(
        &mut self,
        stream: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::VRControllersRecorder_State,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::VRControllersRecorder_State = __cordl_object
            .invoke("LoadState", (stream))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn PlaybackTick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PlaybackTick", ())?;
        Ok(__cordl_ret)
    }
    pub fn RecordTick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecordTick", ())?;
        Ok(__cordl_ret)
    }
    pub fn Save(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Save", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetDefaultSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDefaultSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetInGamePlaybackDefaultSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetInGamePlaybackDefaultSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetPositionAndRotation(
        &mut self,
        transf: *mut crate::UnityEngine::Transform,
        targetPos: crate::UnityEngine::Vector3,
        targetRot: crate::UnityEngine::Quaternion,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPositionAndRotation", (transf, targetPos, targetRot, t))?;
        Ok(__cordl_ret)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_changeToNonVRCamera(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_changeToNonVRCamera", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn get_path(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_path", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_recordingFileName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_recordingFileName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_recordingTextAsset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::TextAsset> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::TextAsset = __cordl_object
            .invoke("get_recordingTextAsset", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn set_recordingFileName(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_recordingFileName", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_recordingTextAsset(
        &mut self,
        value: *mut crate::UnityEngine::TextAsset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_recordingTextAsset", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "VRControllersRecorder")]
impl quest_hook::libil2cpp::ObjectType for VRControllersRecorder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
