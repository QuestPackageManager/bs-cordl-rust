#[cfg(feature = "SaberSwingRatingCounter")]
#[repr(C)]
#[derive(Debug)]
pub struct SaberSwingRatingCounter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _saberMovementData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ISaberMovementData,
    >,
    pub _cutPlaneNormal: crate::UnityEngine::Vector3,
    pub _cutTime: f32,
    pub _afterCutRating: f32,
    pub _beforeCutRating: f32,
    pub _notePlane: crate::UnityEngine::Plane,
    pub _notePlaneWasCut: bool,
    pub _noteForward: crate::UnityEngine::Vector3,
    pub _rateBeforeCut: bool,
    pub _rateAfterCut: bool,
    pub _didChangeReceivers: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LazyCopyHashSet_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::ISaberSwingRatingCounterDidChangeReceiver,
            >,
        >,
    >,
    pub _didFinishReceivers: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LazyCopyHashSet_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::ISaberSwingRatingCounterDidFinishReceiver,
            >,
        >,
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
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SaberSwingRatingCounter => ""
    ."SaberSwingRatingCounter"
);
#[cfg(feature = "SaberSwingRatingCounter")]
impl std::ops::Deref for crate::GlobalNamespace::SaberSwingRatingCounter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SaberSwingRatingCounter")]
impl std::ops::DerefMut for crate::GlobalNamespace::SaberSwingRatingCounter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SaberSwingRatingCounter")]
impl crate::GlobalNamespace::SaberSwingRatingCounter {
    pub fn DrawGizmos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DrawGizmos", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Finish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finish", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        saberMovementData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ISaberMovementData,
        >,
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
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessNewData(
        &mut self,
        newData: crate::GlobalNamespace::BladeMovementDataElement,
        prevData: crate::GlobalNamespace::BladeMovementDataElement,
        prevDataAreValid: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessNewData", (newData, prevData, prevDataAreValid))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterDidChangeReceiver(
        &mut self,
        receiver: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ISaberSwingRatingCounterDidChangeReceiver,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterDidChangeReceiver", (receiver))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterDidFinishReceiver(
        &mut self,
        receiver: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ISaberSwingRatingCounterDidFinishReceiver,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterDidFinishReceiver", (receiver))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterDidChangeReceiver(
        &mut self,
        receiver: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ISaberSwingRatingCounterDidChangeReceiver,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterDidChangeReceiver", (receiver))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterDidFinishReceiver(
        &mut self,
        receiver: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ISaberSwingRatingCounterDidFinishReceiver,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterDidFinishReceiver", (receiver))?;
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
    pub fn get_afterCutRating(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_afterCutRating", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_beforeCutRating(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_beforeCutRating", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SaberSwingRatingCounter")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SaberSwingRatingCounter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SaberSwingRatingCounter")]
impl AsRef<crate::GlobalNamespace::ISaberMovementDataProcessor>
for crate::GlobalNamespace::SaberSwingRatingCounter {
    fn as_ref(&self) -> &crate::GlobalNamespace::ISaberMovementDataProcessor {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "SaberSwingRatingCounter")]
impl AsMut<crate::GlobalNamespace::ISaberMovementDataProcessor>
for crate::GlobalNamespace::SaberSwingRatingCounter {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::ISaberMovementDataProcessor {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "SaberSwingRatingCounter")]
impl AsRef<crate::GlobalNamespace::ISaberSwingRatingCounter>
for crate::GlobalNamespace::SaberSwingRatingCounter {
    fn as_ref(&self) -> &crate::GlobalNamespace::ISaberSwingRatingCounter {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "SaberSwingRatingCounter")]
impl AsMut<crate::GlobalNamespace::ISaberSwingRatingCounter>
for crate::GlobalNamespace::SaberSwingRatingCounter {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::ISaberSwingRatingCounter {
        unsafe { std::mem::transmute(self) }
    }
}
