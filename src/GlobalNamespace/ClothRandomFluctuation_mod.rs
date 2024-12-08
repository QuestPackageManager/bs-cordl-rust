#[cfg(feature = "ClothRandomFluctuation")]
#[repr(C)]
#[derive(Debug)]
pub struct ClothRandomFluctuation {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _cloth: *mut crate::UnityEngine::Cloth,
    pub _useLocalExternalFluctuations: bool,
    pub _externalFluctuations: crate::UnityEngine::Vector3,
    pub _useLocalRandomFluctuations: bool,
    pub _minFluctuations: crate::UnityEngine::Vector3,
    pub _maxFluctuations: crate::UnityEngine::Vector3,
    pub _compoundSins: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::ClothRandomFluctuation_SineLayer,
    >,
    pub _speed: f32,
}
#[cfg(feature = "ClothRandomFluctuation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for ClothRandomFluctuation => ""."ClothRandomFluctuation"
);
#[cfg(feature = "ClothRandomFluctuation")]
impl std::ops::Deref for ClothRandomFluctuation {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ClothRandomFluctuation")]
impl std::ops::DerefMut for ClothRandomFluctuation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ClothRandomFluctuation")]
impl ClothRandomFluctuation {
    #[cfg(feature = "ClothRandomFluctuation+SineLayer")]
    pub type SineLayer = crate::GlobalNamespace::ClothRandomFluctuation_SineLayer;
    pub fn FluctuateCloth(
        &mut self,
        cloth: *mut crate::UnityEngine::Cloth,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FluctuateCloth", (cloth))?;
        Ok(__cordl_ret)
    }
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
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
#[cfg(feature = "ClothRandomFluctuation")]
impl quest_hook::libil2cpp::ObjectType for ClothRandomFluctuation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ClothRandomFluctuation+SineLayer")]
#[repr(C)]
#[derive(Debug)]
pub struct ClothRandomFluctuation_SineLayer {
    __cordl_parent: crate::System::Object,
    pub multiplier: f32,
    pub offset: f32,
}
#[cfg(feature = "ClothRandomFluctuation+SineLayer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ClothRandomFluctuation_SineLayer => ""
    ."ClothRandomFluctuation/SineLayer"
);
#[cfg(feature = "ClothRandomFluctuation+SineLayer")]
impl std::ops::Deref for crate::GlobalNamespace::ClothRandomFluctuation_SineLayer {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ClothRandomFluctuation+SineLayer")]
impl std::ops::DerefMut for crate::GlobalNamespace::ClothRandomFluctuation_SineLayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ClothRandomFluctuation+SineLayer")]
impl crate::GlobalNamespace::ClothRandomFluctuation_SineLayer {
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
#[cfg(feature = "ClothRandomFluctuation+SineLayer")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ClothRandomFluctuation_SineLayer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
