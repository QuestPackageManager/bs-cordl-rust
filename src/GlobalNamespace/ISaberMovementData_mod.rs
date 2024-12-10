#[cfg(feature = "ISaberMovementData")]
#[repr(C)]
#[derive(Debug)]
pub struct ISaberMovementData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ISaberMovementData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ISaberMovementData => ""
    ."ISaberMovementData"
);
#[cfg(feature = "ISaberMovementData")]
impl std::ops::Deref for crate::GlobalNamespace::ISaberMovementData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ISaberMovementData")]
impl std::ops::DerefMut for crate::GlobalNamespace::ISaberMovementData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ISaberMovementData")]
impl crate::GlobalNamespace::ISaberMovementData {
    pub fn AddDataProcessor(
        &mut self,
        dataProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ISaberMovementDataProcessor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddDataProcessor", (dataProcessor))?;
        Ok(__cordl_ret.into())
    }
    pub fn ComputeSwingRating_1(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("ComputeSwingRating", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn RemoveDataProcessor(
        &mut self,
        dataProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ISaberMovementDataProcessor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveDataProcessor", (dataProcessor))?;
        Ok(__cordl_ret.into())
    }
    pub fn RequestLastDataProcessing(
        &mut self,
        dataProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ISaberMovementDataProcessor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RequestLastDataProcessing", (dataProcessor))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_lastAddedData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::BladeMovementDataElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::BladeMovementDataElement = __cordl_object
            .invoke("get_lastAddedData", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ISaberMovementData")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ISaberMovementData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
