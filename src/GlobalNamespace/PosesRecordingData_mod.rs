#[cfg(feature = "PosesRecordingData")]
#[repr(C)]
#[derive(Debug)]
pub struct PosesRecordingData {
    __cordl_parent: crate::System::Object,
    pub objectIds: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    pub keyframes: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::PosesRecordingData_TransformsKeyframe,
    >,
    pub externalCameraCalibration: *mut crate::GlobalNamespace::PosesRecordingData_ExternalCameraCalibration,
}
#[cfg(feature = "PosesRecordingData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PosesRecordingData => ""
    ."PosesRecordingData"
);
#[cfg(feature = "PosesRecordingData")]
impl std::ops::Deref for crate::GlobalNamespace::PosesRecordingData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PosesRecordingData")]
impl std::ops::DerefMut for crate::GlobalNamespace::PosesRecordingData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PosesRecordingData")]
impl crate::GlobalNamespace::PosesRecordingData {
    #[cfg(feature = "PosesRecordingData+ExternalCameraCalibration")]
    pub type ExternalCameraCalibration = crate::GlobalNamespace::PosesRecordingData_ExternalCameraCalibration;
    #[cfg(feature = "PosesRecordingData+TransformsKeyframe")]
    pub type TransformsKeyframe = crate::GlobalNamespace::PosesRecordingData_TransformsKeyframe;
    pub fn AddKeyframe(
        &mut self,
        keyframe: *mut crate::GlobalNamespace::PosesRecordingData_TransformsKeyframe,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddKeyframe", (keyframe))?;
        Ok(__cordl_ret)
    }
    pub fn Contains(
        &mut self,
        objectId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Contains", (objectId))?;
        Ok(__cordl_ret)
    }
    pub fn New_List_1_PosesRecordingData_ExternalCameraCalibration1(
        objectIds: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        keyframes: *mut crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::PosesRecordingData_TransformsKeyframe,
        >,
        externalCameraCalibration: *mut crate::GlobalNamespace::PosesRecordingData_ExternalCameraCalibration,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (objectIds, keyframes, externalCameraCalibration))?;
        Ok(__cordl_object)
    }
    pub fn New_PosesRecordingData_ExternalCameraCalibration0(
        objectIds: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        externalCameraCalibration: *mut crate::GlobalNamespace::PosesRecordingData_ExternalCameraCalibration,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (objectIds, externalCameraCalibration))?;
        Ok(__cordl_object)
    }
    pub fn ObjectIndex(
        &mut self,
        objectId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ObjectIndex", (objectId))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_List_1_PosesRecordingData_ExternalCameraCalibration1(
        &mut self,
        objectIds: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        keyframes: *mut crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::PosesRecordingData_TransformsKeyframe,
        >,
        externalCameraCalibration: *mut crate::GlobalNamespace::PosesRecordingData_ExternalCameraCalibration,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (objectIds, keyframes, externalCameraCalibration))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_PosesRecordingData_ExternalCameraCalibration0(
        &mut self,
        objectIds: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        externalCameraCalibration: *mut crate::GlobalNamespace::PosesRecordingData_ExternalCameraCalibration,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (objectIds, externalCameraCalibration))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "PosesRecordingData")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::PosesRecordingData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PosesRecordingData+ExternalCameraCalibration")]
#[repr(C)]
#[derive(Debug)]
pub struct PosesRecordingData_ExternalCameraCalibration {
    __cordl_parent: crate::System::Object,
    pub fieldOfVision: f32,
    pub nearClip: f32,
    pub farClip: f32,
    pub hmdOffset: f32,
    pub nearOffset: f32,
}
#[cfg(feature = "PosesRecordingData+ExternalCameraCalibration")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PosesRecordingData_ExternalCameraCalibration => ""
    ."PosesRecordingData/ExternalCameraCalibration"
);
#[cfg(feature = "PosesRecordingData+ExternalCameraCalibration")]
impl std::ops::Deref
for crate::GlobalNamespace::PosesRecordingData_ExternalCameraCalibration {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PosesRecordingData+ExternalCameraCalibration")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PosesRecordingData_ExternalCameraCalibration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PosesRecordingData+ExternalCameraCalibration")]
impl crate::GlobalNamespace::PosesRecordingData_ExternalCameraCalibration {
    pub fn New_Camera1(
        camera: *mut crate::UnityEngine::Camera,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (camera))?;
        Ok(__cordl_object)
    }
    pub fn New_f32_f32_f32_f32_f32_0(
        fieldOfVision: f32,
        nearClip: f32,
        farClip: f32,
        hmdOffset: f32,
        nearOffset: f32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (fieldOfVision, nearClip, farClip, hmdOffset, nearOffset),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor_Camera1(
        &mut self,
        camera: *mut crate::UnityEngine::Camera,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (camera))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_f32_f32_f32_f32_f32_0(
        &mut self,
        fieldOfVision: f32,
        nearClip: f32,
        farClip: f32,
        hmdOffset: f32,
        nearOffset: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (fieldOfVision, nearClip, farClip, hmdOffset, nearOffset))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "PosesRecordingData+ExternalCameraCalibration")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PosesRecordingData_ExternalCameraCalibration {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PosesRecordingData+TransformsKeyframe")]
#[repr(C)]
#[derive(Debug)]
pub struct PosesRecordingData_TransformsKeyframe {
    __cordl_parent: crate::System::Object,
    pub poses: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Pose>,
    pub _cordl_time: f32,
}
#[cfg(feature = "PosesRecordingData+TransformsKeyframe")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PosesRecordingData_TransformsKeyframe => ""
    ."PosesRecordingData/TransformsKeyframe"
);
#[cfg(feature = "PosesRecordingData+TransformsKeyframe")]
impl std::ops::Deref for crate::GlobalNamespace::PosesRecordingData_TransformsKeyframe {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PosesRecordingData+TransformsKeyframe")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PosesRecordingData_TransformsKeyframe {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PosesRecordingData+TransformsKeyframe")]
impl crate::GlobalNamespace::PosesRecordingData_TransformsKeyframe {
    pub fn New(
        poses: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Pose>,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (poses, _cordl_time))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        poses: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Pose>,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (poses, _cordl_time))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "PosesRecordingData+TransformsKeyframe")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PosesRecordingData_TransformsKeyframe {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
