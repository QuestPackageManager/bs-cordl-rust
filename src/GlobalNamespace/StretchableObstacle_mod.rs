#[cfg(feature = "StretchableObstacle")]
#[repr(C)]
#[derive(Debug)]
pub struct StretchableObstacle {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _edgeSize: f32,
    pub _coreOffset: crate::UnityEngine::Vector3,
    pub _addColorMultiplier: f32,
    pub _obstacleCoreLerpToWhiteFactor: f32,
    pub _fakeGlowOffset: crate::UnityEngine::Vector3,
    pub _obstacleCore: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _materialPropertyBlockControllers: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::MaterialPropertyBlockController,
        >,
    >,
    pub _obstacleFrame: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ParametricBoxFrameController,
    >,
    pub _obstacleFakeGlow: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ParametricBoxFakeGlowController,
    >,
    pub _manualUVOffsetMultiplier: f32,
    pub _bounds: crate::UnityEngine::Bounds,
}
#[cfg(feature = "StretchableObstacle")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::StretchableObstacle => ""
    ."StretchableObstacle"
);
#[cfg(feature = "StretchableObstacle")]
impl std::ops::Deref for crate::GlobalNamespace::StretchableObstacle {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StretchableObstacle")]
impl std::ops::DerefMut for crate::GlobalNamespace::StretchableObstacle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StretchableObstacle")]
impl crate::GlobalNamespace::StretchableObstacle {
    pub fn CalculateObstacleTransformProperties(
        &mut self,
        width: f32,
        height: f32,
        length: f32,
        localPosition: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        _cordl_size: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        scale: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CalculateObstacleTransformProperties",
                (width, height, length, localPosition, _cordl_size, scale),
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
    pub fn OnValidate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnValidate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetAllProperties(
        &mut self,
        width: f32,
        height: f32,
        length: f32,
        color: crate::UnityEngine::Color,
        manualUvOffset: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetAllProperties", (width, height, length, color, manualUvOffset))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetObstacleCoreTransformProperties(
        &mut self,
        localPosition: crate::UnityEngine::Vector3,
        _cordl_size: crate::UnityEngine::Vector3,
        scale: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetObstacleCoreTransformProperties",
                (localPosition, _cordl_size, scale),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSizeAndOffset(
        &mut self,
        width: f32,
        height: f32,
        length: f32,
        offset: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSizeAndOffset", (width, height, length, offset))?;
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
    pub fn get_bounds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Bounds> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Bounds = __cordl_object
            .invoke("get_bounds", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "StretchableObstacle")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::StretchableObstacle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
