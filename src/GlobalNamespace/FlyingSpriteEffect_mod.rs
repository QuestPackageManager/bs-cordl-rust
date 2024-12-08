#[cfg(feature = "FlyingSpriteEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct FlyingSpriteEffect {
    __cordl_parent: crate::GlobalNamespace::FlyingObjectEffect,
    pub _spriteRenderer: *mut crate::UnityEngine::SpriteRenderer,
    pub _fadeAnimationCurve: *mut crate::UnityEngine::AnimationCurve,
    pub _color: crate::UnityEngine::Color,
}
#[cfg(feature = "FlyingSpriteEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::FlyingSpriteEffect => ""
    ."FlyingSpriteEffect"
);
#[cfg(feature = "FlyingSpriteEffect")]
impl std::ops::Deref for crate::GlobalNamespace::FlyingSpriteEffect {
    type Target = crate::GlobalNamespace::FlyingObjectEffect;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FlyingSpriteEffect")]
impl std::ops::DerefMut for crate::GlobalNamespace::FlyingSpriteEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FlyingSpriteEffect")]
impl crate::GlobalNamespace::FlyingSpriteEffect {
    #[cfg(feature = "FlyingSpriteEffect+Pool")]
    pub type Pool = crate::GlobalNamespace::FlyingSpriteEffect_Pool;
    pub fn InitAndPresent(
        &mut self,
        duration: f32,
        targetPos: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        sprite: *mut crate::UnityEngine::Sprite,
        material: *mut crate::UnityEngine::Material,
        color: crate::UnityEngine::Color,
        shake: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "InitAndPresent",
                (duration, targetPos, rotation, sprite, material, color, shake),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ManualUpdate(
        &mut self,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ManualUpdate", (t))?;
        Ok(__cordl_ret)
    }
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
#[cfg(feature = "FlyingSpriteEffect")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::FlyingSpriteEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "FlyingSpriteEffect+Pool")]
#[repr(C)]
#[derive(Debug)]
pub struct FlyingSpriteEffect_Pool {
    __cordl_parent: crate::Zenject::MonoMemoryPool_1<
        *mut crate::GlobalNamespace::FlyingSpriteEffect,
    >,
}
#[cfg(feature = "FlyingSpriteEffect+Pool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::FlyingSpriteEffect_Pool => ""
    ."FlyingSpriteEffect/Pool"
);
#[cfg(feature = "FlyingSpriteEffect+Pool")]
impl std::ops::Deref for crate::GlobalNamespace::FlyingSpriteEffect_Pool {
    type Target = crate::Zenject::MonoMemoryPool_1<
        *mut crate::GlobalNamespace::FlyingSpriteEffect,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FlyingSpriteEffect+Pool")]
impl std::ops::DerefMut for crate::GlobalNamespace::FlyingSpriteEffect_Pool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FlyingSpriteEffect+Pool")]
impl crate::GlobalNamespace::FlyingSpriteEffect_Pool {
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
#[cfg(feature = "FlyingSpriteEffect+Pool")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::FlyingSpriteEffect_Pool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
