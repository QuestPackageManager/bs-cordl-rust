#[cfg(feature = "TransformSpectrogram")]
#[repr(C)]
#[derive(Debug)]
pub struct TransformSpectrogram {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _transforms: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::Transform,
    >,
    pub _axis: crate::GlobalNamespace::LightAxis,
    pub _minPosition: f32,
    pub _maxPosition: f32,
    pub _scaleSamples: bool,
    pub _scale: f32,
    pub _spectrogramData: *mut crate::GlobalNamespace::BasicSpectrogramData,
    pub _direction: crate::UnityEngine::Vector3,
    pub _defaultPositions: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::Vector3,
    >,
}
#[cfg(feature = "TransformSpectrogram")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::TransformSpectrogram => ""
    ."TransformSpectrogram"
);
#[cfg(feature = "TransformSpectrogram")]
impl std::ops::Deref for crate::GlobalNamespace::TransformSpectrogram {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TransformSpectrogram")]
impl std::ops::DerefMut for crate::GlobalNamespace::TransformSpectrogram {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TransformSpectrogram")]
impl crate::GlobalNamespace::TransformSpectrogram {
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
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
#[cfg(feature = "TransformSpectrogram")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::TransformSpectrogram {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
