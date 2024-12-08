#[cfg(feature = "SaberMovementData")]
#[repr(C)]
#[derive(Debug)]
pub struct SaberMovementData {
    __cordl_parent: crate::System::Object,
    pub _data: *mut quest_hook::libil2cpp::Il2CppArray<BladeMovementDataElement>,
    pub _dataProcessors: *mut LazyCopyHashSet_1<*mut ISaberMovementDataProcessor>,
    pub _nextAddIndex: i32,
    pub _validCount: i32,
    pub _bladeSpeed: f32,
}
#[cfg(feature = "SaberMovementData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SaberMovementData => ""."SaberMovementData"
);
#[cfg(feature = "SaberMovementData")]
impl std::ops::Deref for SaberMovementData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SaberMovementData")]
impl std::ops::DerefMut for SaberMovementData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SaberMovementData")]
impl SaberMovementData {
    pub const kOutOfRangeBladeSpeed: f32 = 100f32;
    pub const kSmoothDownBladeSpeedCoef: f32 = 2f32;
    pub const kSmoothUpBladeSpeedCoef: f32 = 24f32;
    pub fn AddDataProcessor(
        &mut self,
        dataProcessor: *mut ISaberMovementDataProcessor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddDataProcessor", (dataProcessor))?;
        Ok(__cordl_ret)
    }
    pub fn AddNewData(
        &mut self,
        topPos: crate::UnityEngine::Vector3,
        bottomPos: crate::UnityEngine::Vector3,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddNewData", (topPos, bottomPos, _cordl_time))?;
        Ok(__cordl_ret)
    }
    pub fn ComputeAdditionalData(
        &mut self,
        topPos: crate::UnityEngine::Vector3,
        bottomPos: crate::UnityEngine::Vector3,
        idxOffset: i32,
        segmentNormal: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        segmentAngle: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ComputeAdditionalData",
                (topPos, bottomPos, idxOffset, segmentNormal, segmentAngle),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ComputeCutPlaneNormal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("ComputeCutPlaneNormal", ())?;
        Ok(__cordl_ret)
    }
    pub fn ComputePlaneNormal(
        &mut self,
        tp0: crate::UnityEngine::Vector3,
        bp0: crate::UnityEngine::Vector3,
        tp1: crate::UnityEngine::Vector3,
        bp1: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("ComputePlaneNormal", (tp0, bp0, tp1, bp1))?;
        Ok(__cordl_ret)
    }
    pub fn ComputeSwingRating_1(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("ComputeSwingRating", ())?;
        Ok(__cordl_ret)
    }
    pub fn ComputeSwingRating__cordl_bool_f32_2(
        &mut self,
        overrideSegmenAngle: bool,
        overrideValue: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("ComputeSwingRating", (overrideSegmenAngle, overrideValue))?;
        Ok(__cordl_ret)
    }
    pub fn ComputeSwingRating_f32_0(
        &mut self,
        overrideSegmentAngle: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("ComputeSwingRating", (overrideSegmentAngle))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn RemoveDataProcessor(
        &mut self,
        dataProcessor: *mut ISaberMovementDataProcessor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveDataProcessor", (dataProcessor))?;
        Ok(__cordl_ret)
    }
    pub fn RequestLastDataProcessing(
        &mut self,
        dataProcessor: *mut ISaberMovementDataProcessor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RequestLastDataProcessing", (dataProcessor))?;
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
    pub fn get_bladeSpeed(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_bladeSpeed", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_lastAddedData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<BladeMovementDataElement> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: BladeMovementDataElement = __cordl_object
            .invoke("get_lastAddedData", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_prevAddedData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<BladeMovementDataElement> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: BladeMovementDataElement = __cordl_object
            .invoke("get_prevAddedData", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "SaberMovementData")]
impl quest_hook::libil2cpp::ObjectType for SaberMovementData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
