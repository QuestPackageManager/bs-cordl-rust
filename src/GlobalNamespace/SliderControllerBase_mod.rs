#[cfg(feature = "SliderControllerBase")]
#[repr(C)]
#[derive(Debug)]
pub struct SliderControllerBase {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _materialPropertyBlockController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MaterialPropertyBlockController,
    >,
    pub _cutoutAnimateEffect: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::CutoutAnimateEffect,
    >,
    pub _dissolving: bool,
}
#[cfg(feature = "SliderControllerBase")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SliderControllerBase => ""
    ."SliderControllerBase"
);
#[cfg(feature = "SliderControllerBase")]
impl std::ops::Deref for crate::GlobalNamespace::SliderControllerBase {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SliderControllerBase")]
impl std::ops::DerefMut for crate::GlobalNamespace::SliderControllerBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SliderControllerBase")]
impl crate::GlobalNamespace::SliderControllerBase {
    pub fn AnimateCutout(
        &mut self,
        cutoutStart: f32,
        cutoutEnd: f32,
        duration: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AnimateCutout", (cutoutStart, cutoutEnd, duration))?;
        Ok(__cordl_ret.into())
    }
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
#[cfg(feature = "SliderControllerBase")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SliderControllerBase {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
