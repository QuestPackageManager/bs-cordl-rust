#[cfg(feature = "DroneHover")]
#[repr(C)]
#[derive(Debug)]
pub struct DroneHover {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _hoverAreaPerAxis: crate::UnityEngine::Vector3,
    pub _speed: f32,
    pub _compoundSins: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::DroneHover_SineLayer,
    >,
    pub _tiltTransforms: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::Transform,
    >,
    pub _maxTiltAmount: f32,
    pub _tiltSpeed: f32,
    pub _tiltAheadOfTime: f32,
    pub _tiltToTarget: bool,
    pub _startPos: crate::UnityEngine::Vector3,
    pub _cloth: *mut crate::UnityEngine::Cloth,
}
#[cfg(feature = "DroneHover")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for DroneHover => ""."DroneHover"
);
#[cfg(feature = "DroneHover")]
impl std::ops::Deref for DroneHover {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DroneHover")]
impl std::ops::DerefMut for DroneHover {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DroneHover")]
impl DroneHover {
    #[cfg(feature = "DroneHover+SineLayer")]
    pub type SineLayer = crate::GlobalNamespace::DroneHover_SineLayer;
    pub fn GetNoise(
        &mut self,
        _cordl_time: f32,
        offset: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetNoise", (_cordl_time, offset))?;
        Ok(__cordl_ret)
    }
    pub fn GetNoiseVec3(
        &mut self,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("GetNoiseVec3", (_cordl_time))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn UpdateTiltTransform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateTiltTransform", ())?;
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
}
#[cfg(feature = "DroneHover")]
impl quest_hook::libil2cpp::ObjectType for DroneHover {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "DroneHover+SineLayer")]
#[repr(C)]
#[derive(Debug)]
pub struct DroneHover_SineLayer {
    __cordl_parent: crate::System::Object,
    pub multiplier: f32,
    pub offset: f32,
}
#[cfg(feature = "DroneHover+SineLayer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::DroneHover_SineLayer => ""
    ."DroneHover/SineLayer"
);
#[cfg(feature = "DroneHover+SineLayer")]
impl std::ops::Deref for crate::GlobalNamespace::DroneHover_SineLayer {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DroneHover+SineLayer")]
impl std::ops::DerefMut for crate::GlobalNamespace::DroneHover_SineLayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DroneHover+SineLayer")]
impl crate::GlobalNamespace::DroneHover_SineLayer {
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
#[cfg(feature = "DroneHover+SineLayer")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::DroneHover_SineLayer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}