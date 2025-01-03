#[cfg(feature = "FlyingTextEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct FlyingTextEffect {
    __cordl_parent: crate::GlobalNamespace::FlyingObjectEffect,
    pub _text: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshPro>,
    pub _fadeAnimationCurve: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AnimationCurve,
    >,
    pub _color: crate::UnityEngine::Color,
}
#[cfg(feature = "FlyingTextEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::FlyingTextEffect => ""
    ."FlyingTextEffect"
);
#[cfg(feature = "FlyingTextEffect")]
impl std::ops::Deref for crate::GlobalNamespace::FlyingTextEffect {
    type Target = crate::GlobalNamespace::FlyingObjectEffect;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FlyingTextEffect")]
impl std::ops::DerefMut for crate::GlobalNamespace::FlyingTextEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FlyingTextEffect")]
impl crate::GlobalNamespace::FlyingTextEffect {
    #[cfg(feature = "FlyingTextEffect+Pool")]
    pub type Pool = crate::GlobalNamespace::FlyingTextEffect_Pool;
    pub fn InitAndPresent(
        &mut self,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        duration: f32,
        targetPos: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        color: crate::UnityEngine::Color,
        fontSize: f32,
        shake: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "InitAndPresent",
                (text, duration, targetPos, rotation, color, fontSize, shake),
            )?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "FlyingTextEffect")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::FlyingTextEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "FlyingTextEffect+Pool")]
#[repr(C)]
#[derive(Debug)]
pub struct FlyingTextEffect_Pool {
    __cordl_parent: crate::Zenject::MonoMemoryPool_1<
        *mut crate::GlobalNamespace::FlyingTextEffect,
    >,
}
#[cfg(feature = "FlyingTextEffect+Pool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::FlyingTextEffect_Pool => ""
    ."FlyingTextEffect/Pool"
);
#[cfg(feature = "FlyingTextEffect+Pool")]
impl std::ops::Deref for crate::GlobalNamespace::FlyingTextEffect_Pool {
    type Target = crate::Zenject::MonoMemoryPool_1<
        *mut crate::GlobalNamespace::FlyingTextEffect,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FlyingTextEffect+Pool")]
impl std::ops::DerefMut for crate::GlobalNamespace::FlyingTextEffect_Pool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FlyingTextEffect+Pool")]
impl crate::GlobalNamespace::FlyingTextEffect_Pool {
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
#[cfg(feature = "FlyingTextEffect+Pool")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::FlyingTextEffect_Pool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
