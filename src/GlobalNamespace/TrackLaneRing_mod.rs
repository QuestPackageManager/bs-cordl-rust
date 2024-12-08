#[cfg(feature = "TrackLaneRing")]
#[repr(C)]
#[derive(Debug)]
pub struct TrackLaneRing {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _prevRotZ: f32,
    pub _rotZ: f32,
    pub _destRotZ: f32,
    pub _rotationSpeed: f32,
    pub _prevPosZ: f32,
    pub _posZ: f32,
    pub _destPosZ: f32,
    pub _moveSpeed: f32,
    pub _positionOffset: crate::UnityEngine::Vector3,
    pub _transform: *mut crate::UnityEngine::Transform,
}
#[cfg(feature = "TrackLaneRing")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::TrackLaneRing => ""
    ."TrackLaneRing"
);
#[cfg(feature = "TrackLaneRing")]
impl std::ops::Deref for crate::GlobalNamespace::TrackLaneRing {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TrackLaneRing")]
impl std::ops::DerefMut for crate::GlobalNamespace::TrackLaneRing {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TrackLaneRing")]
impl crate::GlobalNamespace::TrackLaneRing {
    pub fn FixedUpdateRing(
        &mut self,
        fixedDeltaTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FixedUpdateRing", (fixedDeltaTime))?;
        Ok(__cordl_ret)
    }
    pub fn GetDestinationRotation(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetDestinationRotation", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetRotation(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetRotation", ())?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        position: crate::UnityEngine::Vector3,
        positionOffset: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (position, positionOffset))?;
        Ok(__cordl_ret)
    }
    pub fn LateUpdateRing(
        &mut self,
        interpolationFactor: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateUpdateRing", (interpolationFactor))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SetDestRotation(
        &mut self,
        destRotZ: f32,
        rotateSpeed: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDestRotation", (destRotZ, rotateSpeed))?;
        Ok(__cordl_ret)
    }
    pub fn SetPosition(
        &mut self,
        destPosZ: f32,
        moveSpeed: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPosition", (destPosZ, moveSpeed))?;
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
    pub fn get_destRotZ(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_destRotZ", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "TrackLaneRing")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::TrackLaneRing {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
