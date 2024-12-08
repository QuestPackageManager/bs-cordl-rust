#[cfg(feature = "PosesRecordingSaveData+ExternalCameraCalibrationSaveData")]
#[repr(C)]
#[derive(Debug)]
pub struct PosesRecordingSaveData_ExternalCameraCalibrationSaveData {
    __cordl_parent: crate::System::Object,
    pub fieldOfVision: f32,
    pub nearClip: f32,
    pub farClip: f32,
    pub hmdOffset: f32,
    pub nearOffset: f32,
}
#[cfg(feature = "PosesRecordingSaveData+ExternalCameraCalibrationSaveData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PosesRecordingSaveData_ExternalCameraCalibrationSaveData => ""
    ."PosesRecordingSaveData/ExternalCameraCalibrationSaveData"
);
#[cfg(feature = "PosesRecordingSaveData+ExternalCameraCalibrationSaveData")]
impl std::ops::Deref
for crate::GlobalNamespace::PosesRecordingSaveData_ExternalCameraCalibrationSaveData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PosesRecordingSaveData+ExternalCameraCalibrationSaveData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PosesRecordingSaveData_ExternalCameraCalibrationSaveData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PosesRecordingSaveData+ExternalCameraCalibrationSaveData")]
impl crate::GlobalNamespace::PosesRecordingSaveData_ExternalCameraCalibrationSaveData {
    pub fn New(
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
    pub fn _ctor(
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
#[cfg(feature = "PosesRecordingSaveData+ExternalCameraCalibrationSaveData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PosesRecordingSaveData_ExternalCameraCalibrationSaveData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PosesRecordingSaveData+PoseSaveData")]
#[repr(C)]
#[derive(Debug)]
pub struct PosesRecordingSaveData_PoseSaveData {
    __cordl_parent: crate::System::Object,
    pub posX: f32,
    pub posY: f32,
    pub posZ: f32,
    pub rotX: f32,
    pub rotY: f32,
    pub rotZ: f32,
    pub rotW: f32,
}
#[cfg(feature = "PosesRecordingSaveData+PoseSaveData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PosesRecordingSaveData_PoseSaveData => ""
    ."PosesRecordingSaveData/PoseSaveData"
);
#[cfg(feature = "PosesRecordingSaveData+PoseSaveData")]
impl std::ops::Deref for crate::GlobalNamespace::PosesRecordingSaveData_PoseSaveData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PosesRecordingSaveData+PoseSaveData")]
impl std::ops::DerefMut for crate::GlobalNamespace::PosesRecordingSaveData_PoseSaveData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PosesRecordingSaveData+PoseSaveData")]
impl crate::GlobalNamespace::PosesRecordingSaveData_PoseSaveData {
    pub fn New(
        posX: f32,
        posY: f32,
        posZ: f32,
        rotX: f32,
        rotY: f32,
        rotZ: f32,
        rotW: f32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (posX, posY, posZ, rotX, rotY, rotZ, rotW))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        posX: f32,
        posY: f32,
        posZ: f32,
        rotX: f32,
        rotY: f32,
        rotZ: f32,
        rotW: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (posX, posY, posZ, rotX, rotY, rotZ, rotW))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "PosesRecordingSaveData+PoseSaveData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PosesRecordingSaveData_PoseSaveData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PosesRecordingSaveData")]
#[repr(C)]
#[derive(Debug)]
pub struct PosesRecordingSaveData {
    __cordl_parent: crate::System::Object,
    pub objectIds: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    pub keyframes: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::PosesRecordingSaveData_TransformsSaveKeyframe,
    >,
    pub externalCameraCalibration: *mut crate::GlobalNamespace::PosesRecordingSaveData_ExternalCameraCalibrationSaveData,
}
#[cfg(feature = "PosesRecordingSaveData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for PosesRecordingSaveData => ""."PosesRecordingSaveData"
);
#[cfg(feature = "PosesRecordingSaveData")]
impl std::ops::Deref for PosesRecordingSaveData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PosesRecordingSaveData")]
impl std::ops::DerefMut for PosesRecordingSaveData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PosesRecordingSaveData")]
impl PosesRecordingSaveData {
    #[cfg(feature = "PosesRecordingSaveData+PoseSaveData")]
    pub type PoseSaveData = crate::GlobalNamespace::PosesRecordingSaveData_PoseSaveData;
    #[cfg(feature = "PosesRecordingSaveData+TransformsSaveKeyframe")]
    pub type TransformsSaveKeyframe = crate::GlobalNamespace::PosesRecordingSaveData_TransformsSaveKeyframe;
    #[cfg(feature = "PosesRecordingSaveData+ExternalCameraCalibrationSaveData")]
    pub type ExternalCameraCalibrationSaveData = crate::GlobalNamespace::PosesRecordingSaveData_ExternalCameraCalibrationSaveData;
    pub fn New(
        objectIds: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        keyframes: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::PosesRecordingSaveData_TransformsSaveKeyframe,
        >,
        externalCameraCalibration: *mut crate::GlobalNamespace::PosesRecordingSaveData_ExternalCameraCalibrationSaveData,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (objectIds, keyframes, externalCameraCalibration))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        objectIds: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        keyframes: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::PosesRecordingSaveData_TransformsSaveKeyframe,
        >,
        externalCameraCalibration: *mut crate::GlobalNamespace::PosesRecordingSaveData_ExternalCameraCalibrationSaveData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (objectIds, keyframes, externalCameraCalibration))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "PosesRecordingSaveData")]
impl quest_hook::libil2cpp::ObjectType for PosesRecordingSaveData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PosesRecordingSaveData+TransformsSaveKeyframe")]
#[repr(C)]
#[derive(Debug)]
pub struct PosesRecordingSaveData_TransformsSaveKeyframe {
    __cordl_parent: crate::System::Object,
    pub poses: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::PosesRecordingSaveData_PoseSaveData,
    >,
    pub _cordl_time: f32,
}
#[cfg(feature = "PosesRecordingSaveData+TransformsSaveKeyframe")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PosesRecordingSaveData_TransformsSaveKeyframe => ""
    ."PosesRecordingSaveData/TransformsSaveKeyframe"
);
#[cfg(feature = "PosesRecordingSaveData+TransformsSaveKeyframe")]
impl std::ops::Deref
for crate::GlobalNamespace::PosesRecordingSaveData_TransformsSaveKeyframe {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PosesRecordingSaveData+TransformsSaveKeyframe")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PosesRecordingSaveData_TransformsSaveKeyframe {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PosesRecordingSaveData+TransformsSaveKeyframe")]
impl crate::GlobalNamespace::PosesRecordingSaveData_TransformsSaveKeyframe {
    pub fn New(
        poses: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::PosesRecordingSaveData_PoseSaveData,
        >,
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
        poses: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::PosesRecordingSaveData_PoseSaveData,
        >,
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
#[cfg(feature = "PosesRecordingSaveData+TransformsSaveKeyframe")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PosesRecordingSaveData_TransformsSaveKeyframe {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
