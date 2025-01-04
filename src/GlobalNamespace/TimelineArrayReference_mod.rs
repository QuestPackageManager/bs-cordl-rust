#[cfg(feature = "TimelineArrayReference")]
#[repr(C)]
#[derive(Debug)]
pub struct TimelineArrayReference {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub arrayType: crate::GlobalNamespace::TimelineArrayReference_ArrayTypes,
    pub _tubeLightArray: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::TubeBloomPrePassLight,
        >,
    >,
    pub _canvasGroupArray: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::CanvasGroup>,
    >,
    pub _tmproArray: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::TMPro::TextMeshPro>,
    >,
    pub _transformArray: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Transform>,
    >,
    pub _directionalLights: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::GlobalNamespace::DirectionalLight>,
    >,
}
#[cfg(feature = "TimelineArrayReference")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::TimelineArrayReference => ""
    ."TimelineArrayReference"
);
#[cfg(feature = "TimelineArrayReference")]
impl std::ops::Deref for crate::GlobalNamespace::TimelineArrayReference {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TimelineArrayReference")]
impl std::ops::DerefMut for crate::GlobalNamespace::TimelineArrayReference {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TimelineArrayReference")]
impl crate::GlobalNamespace::TimelineArrayReference {
    #[cfg(feature = "TimelineArrayReference+ArrayTypes")]
    pub type ArrayTypes = crate::GlobalNamespace::TimelineArrayReference_ArrayTypes;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
}
#[cfg(feature = "TimelineArrayReference")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::TimelineArrayReference {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TimelineArrayReference+ArrayTypes")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TimelineArrayReference_ArrayTypes {
    #[default]
    Canvas = 2i32,
    DirectionalLight = 4i32,
    TextMeshPro = 3i32,
    Transform = 1i32,
    TubeLight = 0i32,
}
#[cfg(feature = "TimelineArrayReference+ArrayTypes")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::TimelineArrayReference_ArrayTypes => ""
    ."TimelineArrayReference/ArrayTypes"
);
