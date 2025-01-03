#[cfg(feature = "FlyingSpriteSpawner")]
#[repr(C)]
#[derive(Debug)]
pub struct FlyingSpriteSpawner {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _sprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub _material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub _duration: f32,
    pub _xSpread: f32,
    pub _targetYPos: f32,
    pub _targetZPos: f32,
    pub _color: crate::UnityEngine::Color,
    pub _shake: bool,
    pub _flyingSpriteEffectPool: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::FlyingSpriteEffect_Pool,
    >,
}
#[cfg(feature = "FlyingSpriteSpawner")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::FlyingSpriteSpawner => ""
    ."FlyingSpriteSpawner"
);
#[cfg(feature = "FlyingSpriteSpawner")]
impl std::ops::Deref for crate::GlobalNamespace::FlyingSpriteSpawner {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FlyingSpriteSpawner")]
impl std::ops::DerefMut for crate::GlobalNamespace::FlyingSpriteSpawner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FlyingSpriteSpawner")]
impl crate::GlobalNamespace::FlyingSpriteSpawner {
    pub fn HandleFlyingObjectEffectDidFinish(
        &mut self,
        flyingObjectEffect: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::FlyingObjectEffect,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleFlyingObjectEffectDidFinish", (flyingObjectEffect))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SpawnFlyingSprite(
        &mut self,
        pos: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        inverseRotation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SpawnFlyingSprite", (pos, rotation, inverseRotation))?;
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
}
#[cfg(feature = "FlyingSpriteSpawner")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::FlyingSpriteSpawner {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "FlyingSpriteSpawner")]
impl AsRef<crate::GlobalNamespace::IFlyingObjectEffectDidFinishEvent>
for crate::GlobalNamespace::FlyingSpriteSpawner {
    fn as_ref(&self) -> &crate::GlobalNamespace::IFlyingObjectEffectDidFinishEvent {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "FlyingSpriteSpawner")]
impl AsMut<crate::GlobalNamespace::IFlyingObjectEffectDidFinishEvent>
for crate::GlobalNamespace::FlyingSpriteSpawner {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::IFlyingObjectEffectDidFinishEvent {
        unsafe { std::mem::transmute(self) }
    }
}
