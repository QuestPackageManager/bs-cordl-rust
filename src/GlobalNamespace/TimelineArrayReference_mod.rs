#[cfg(feature = "TimelineArrayReference+ArrayTypes")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimelineArrayReference_ArrayTypes {
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
#[cfg(feature = "TimelineArrayReference")]
#[repr(C)]
#[derive(Debug)]
pub struct TimelineArrayReference {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub arrayType: crate::GlobalNamespace::TimelineArrayReference_ArrayTypes,
    pub _tubeLightArray: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut TubeBloomPrePassLight,
    >,
    pub _canvasGroupArray: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::CanvasGroup,
    >,
    pub _tmproArray: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::TMPro::TextMeshPro,
    >,
    pub _transformArray: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::Transform,
    >,
    pub _directionalLights: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut DirectionalLight,
    >,
}
#[cfg(feature = "TimelineArrayReference")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for TimelineArrayReference => ""."TimelineArrayReference"
);
#[cfg(feature = "TimelineArrayReference")]
impl std::ops::Deref for TimelineArrayReference {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TimelineArrayReference")]
impl std::ops::DerefMut for TimelineArrayReference {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TimelineArrayReference")]
impl TimelineArrayReference {
    #[cfg(feature = "TimelineArrayReference+ArrayTypes")]
    pub type ArrayTypes = crate::GlobalNamespace::TimelineArrayReference_ArrayTypes;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "TimelineArrayReference")]
impl quest_hook::libil2cpp::ObjectType for TimelineArrayReference {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
