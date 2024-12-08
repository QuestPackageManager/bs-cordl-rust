#[cfg(feature = "FlyingTextSpawner")]
#[repr(C)]
#[derive(Debug)]
pub struct FlyingTextSpawner {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _duration: f32,
    pub _xSpread: f32,
    pub _targetYPos: f32,
    pub _targetZPos: f32,
    pub _color: crate::UnityEngine::Color,
    pub _fontSize: f32,
    pub _shake: bool,
    pub _flyingTextEffectPool: *mut crate::GlobalNamespace::FlyingTextEffect_Pool,
}
#[cfg(feature = "FlyingTextSpawner")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::FlyingTextSpawner => ""
    ."FlyingTextSpawner"
);
#[cfg(feature = "FlyingTextSpawner")]
impl std::ops::Deref for crate::GlobalNamespace::FlyingTextSpawner {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FlyingTextSpawner")]
impl std::ops::DerefMut for crate::GlobalNamespace::FlyingTextSpawner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FlyingTextSpawner")]
impl crate::GlobalNamespace::FlyingTextSpawner {
    pub fn HandleFlyingObjectEffectDidFinish(
        &mut self,
        flyingObjectEffect: *mut crate::GlobalNamespace::FlyingObjectEffect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleFlyingObjectEffectDidFinish", (flyingObjectEffect))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SpawnText(
        &mut self,
        pos: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        inverseRotation: crate::UnityEngine::Quaternion,
        text: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SpawnText", (pos, rotation, inverseRotation, text))?;
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
#[cfg(feature = "FlyingTextSpawner")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::FlyingTextSpawner {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
