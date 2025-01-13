#[cfg(feature = "PosesRecordingInfoSaveData")]
#[repr(C)]
#[derive(Debug)]
pub struct PosesRecordingInfoSaveData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _version: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _objectIds: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub _externalCameraCalibration: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PosesRecordingInfoSaveData_ExternalCameraCalibrationSaveData,
    >,
    pub _dataFileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "PosesRecordingInfoSaveData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PosesRecordingInfoSaveData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PosesRecordingInfoSaveData";
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
#[cfg(feature = "PosesRecordingInfoSaveData")]
impl std::ops::Deref for crate::GlobalNamespace::PosesRecordingInfoSaveData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PosesRecordingInfoSaveData")]
impl std::ops::DerefMut for crate::GlobalNamespace::PosesRecordingInfoSaveData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PosesRecordingInfoSaveData")]
impl crate::GlobalNamespace::PosesRecordingInfoSaveData {
    pub const kCurrentVersion: &'static str = "1.0.0";
    #[cfg(feature = "PosesRecordingInfoSaveData+ExternalCameraCalibrationSaveData")]
    pub type ExternalCameraCalibrationSaveData = crate::GlobalNamespace::PosesRecordingInfoSaveData_ExternalCameraCalibrationSaveData;
    pub fn New(
        objectIds: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        externalCameraCalibration: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PosesRecordingInfoSaveData_ExternalCameraCalibrationSaveData,
        >,
        dataFileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (objectIds, externalCameraCalibration, dataFileName))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        objectIds: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        externalCameraCalibration: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PosesRecordingInfoSaveData_ExternalCameraCalibrationSaveData,
        >,
        dataFileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (objectIds, externalCameraCalibration, dataFileName))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_dataFileName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_dataFileName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_externalCameraCalibration(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PosesRecordingInfoSaveData_ExternalCameraCalibrationSaveData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PosesRecordingInfoSaveData_ExternalCameraCalibrationSaveData,
        > = __cordl_object.invoke("get_externalCameraCalibration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_objectIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = __cordl_object.invoke("get_objectIds", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_version(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_version", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PosesRecordingInfoSaveData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PosesRecordingInfoSaveData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PosesRecordingInfoSaveData+ExternalCameraCalibrationSaveData")]
#[repr(C)]
#[derive(Debug)]
pub struct PosesRecordingInfoSaveData_ExternalCameraCalibrationSaveData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _fieldOfVision: f32,
    pub _nearClip: f32,
    pub _farClip: f32,
    pub _hmdOffset: f32,
    pub _nearOffset: f32,
}
#[cfg(feature = "PosesRecordingInfoSaveData+ExternalCameraCalibrationSaveData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PosesRecordingInfoSaveData_ExternalCameraCalibrationSaveData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PosesRecordingInfoSaveData/ExternalCameraCalibrationSaveData";
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
#[cfg(feature = "PosesRecordingInfoSaveData+ExternalCameraCalibrationSaveData")]
impl std::ops::Deref
for crate::GlobalNamespace::PosesRecordingInfoSaveData_ExternalCameraCalibrationSaveData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PosesRecordingInfoSaveData+ExternalCameraCalibrationSaveData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PosesRecordingInfoSaveData_ExternalCameraCalibrationSaveData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PosesRecordingInfoSaveData+ExternalCameraCalibrationSaveData")]
impl crate::GlobalNamespace::PosesRecordingInfoSaveData_ExternalCameraCalibrationSaveData {
    pub fn New(
        fieldOfVision: f32,
        nearClip: f32,
        farClip: f32,
        hmdOffset: f32,
        nearOffset: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (fieldOfVision, nearClip, farClip, hmdOffset, nearOffset),
            )?;
        Ok(__cordl_object.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn get_farClip(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_farClip", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fieldOfVision(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_fieldOfVision", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hmdOffset(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_hmdOffset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_nearClip(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_nearClip", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_nearOffset(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_nearOffset", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PosesRecordingInfoSaveData+ExternalCameraCalibrationSaveData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PosesRecordingInfoSaveData_ExternalCameraCalibrationSaveData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
