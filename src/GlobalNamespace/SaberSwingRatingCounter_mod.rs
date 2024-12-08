#[cfg(feature = "SaberSwingRatingCounter")]
#[repr(C)]
#[derive(Debug)]
pub struct SaberSwingRatingCounter {
    __cordl_parent: crate::System::Object,
    pub _saberMovementData: *mut ISaberMovementData,
    pub _cutPlaneNormal: crate::UnityEngine::Vector3,
    pub _cutTime: f32,
    pub _afterCutRating: f32,
    pub _beforeCutRating: f32,
    pub _notePlane: crate::UnityEngine::Plane,
    pub _notePlaneWasCut: bool,
    pub _noteForward: crate::UnityEngine::Vector3,
    pub _rateBeforeCut: bool,
    pub _rateAfterCut: bool,
    pub _didChangeReceivers: *mut LazyCopyHashSet_1<
        *mut ISaberSwingRatingCounterDidChangeReceiver,
    >,
    pub _didFinishReceivers: *mut LazyCopyHashSet_1<
        *mut ISaberSwingRatingCounterDidFinishReceiver,
    >,
    pub _notePlaneCenter: crate::UnityEngine::Vector3,
    pub _beforeCutTopPos: crate::UnityEngine::Vector3,
    pub _beforeCutBottomPos: crate::UnityEngine::Vector3,
    pub _afterCutTopPos: crate::UnityEngine::Vector3,
    pub _afterCutBottomPos: crate::UnityEngine::Vector3,
    pub _newPlaneNormal: crate::UnityEngine::Vector3,
    pub _cutTopPos: crate::UnityEngine::Vector3,
    pub _cutBottomPos: crate::UnityEngine::Vector3,
    pub _finished: bool,
}
#[cfg(feature = "SaberSwingRatingCounter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SaberSwingRatingCounter => ""."SaberSwingRatingCounter"
);
#[cfg(feature = "SaberSwingRatingCounter")]
impl std::ops::Deref for SaberSwingRatingCounter {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SaberSwingRatingCounter")]
impl std::ops::DerefMut for SaberSwingRatingCounter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SaberSwingRatingCounter")]
impl SaberSwingRatingCounter {
    pub fn DrawGizmos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DrawGizmos", ())?;
        Ok(__cordl_ret)
    }
    pub fn Finish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finish", ())?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        saberMovementData: *mut ISaberMovementData,
        notePosition: crate::UnityEngine::Vector3,
        noteRotation: crate::UnityEngine::Quaternion,
        rateBeforeCut: bool,
        rateAfterCut: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
                (
                    saberMovementData,
                    notePosition,
                    noteRotation,
                    rateBeforeCut,
                    rateAfterCut,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ProcessNewData(
        &mut self,
        newData: BladeMovementDataElement,
        prevData: BladeMovementDataElement,
        prevDataAreValid: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessNewData", (newData, prevData, prevDataAreValid))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterDidChangeReceiver(
        &mut self,
        receiver: *mut ISaberSwingRatingCounterDidChangeReceiver,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterDidChangeReceiver", (receiver))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterDidFinishReceiver(
        &mut self,
        receiver: *mut ISaberSwingRatingCounterDidFinishReceiver,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterDidFinishReceiver", (receiver))?;
        Ok(__cordl_ret)
    }
    pub fn UnregisterDidChangeReceiver(
        &mut self,
        receiver: *mut ISaberSwingRatingCounterDidChangeReceiver,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterDidChangeReceiver", (receiver))?;
        Ok(__cordl_ret)
    }
    pub fn UnregisterDidFinishReceiver(
        &mut self,
        receiver: *mut ISaberSwingRatingCounterDidFinishReceiver,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterDidFinishReceiver", (receiver))?;
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
    pub fn get_afterCutRating(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_afterCutRating", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_beforeCutRating(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_beforeCutRating", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "SaberSwingRatingCounter")]
impl quest_hook::libil2cpp::ObjectType for SaberSwingRatingCounter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
