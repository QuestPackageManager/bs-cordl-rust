#[cfg(feature = "Ice+FloorLightTile")]
#[repr(C)]
#[derive(Debug)]
pub struct FloorLightTile {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _colorSetter: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MaterialPropertyBlockColorSetter,
    >,
    pub _tubeBloomPrePassLight: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::TubeBloomPrePassLight,
    >,
    pub _songTimeTweeningManager: quest_hook::libil2cpp::Gc<
        crate::Tweening::SongTimeTweeningManager,
    >,
    pub didFinish: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<*mut crate::Ice::FloorLightTile>,
    >,
    pub _fadeInTween: quest_hook::libil2cpp::Gc<crate::Tweening::ColorTween>,
    pub _fadeOutTween: quest_hook::libil2cpp::Gc<crate::Tweening::ColorTween>,
}
#[cfg(feature = "Ice+FloorLightTile")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Ice::FloorLightTile => "Ice"."FloorLightTile"
);
#[cfg(feature = "Ice+FloorLightTile")]
impl std::ops::Deref for crate::Ice::FloorLightTile {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Ice+FloorLightTile")]
impl std::ops::DerefMut for crate::Ice::FloorLightTile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Ice+FloorLightTile")]
impl crate::Ice::FloorLightTile {
    #[cfg(feature = "Ice+FloorLightTile+Pool")]
    pub type Pool = crate::Ice::FloorLightTile_Pool;
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleFadeInTweenOnCompleted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleFadeInTweenOnCompleted", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleFadeOutTweenOnCompleted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleFadeOutTweenOnCompleted", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HighlightWithColor(
        &mut self,
        color: crate::UnityEngine::Color,
        fadeInDuration: f32,
        fadeOutDuration: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HighlightWithColor", (color, fadeInDuration, fadeOutDuration))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetColor(
        &mut self,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColor", (color))?;
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
#[cfg(feature = "Ice+FloorLightTile")]
impl quest_hook::libil2cpp::ObjectType for crate::Ice::FloorLightTile {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Ice+FloorLightTile+Pool")]
#[repr(C)]
#[derive(Debug)]
pub struct FloorLightTile_Pool {
    __cordl_parent: crate::Zenject::MonoMemoryPool_1<*mut crate::Ice::FloorLightTile>,
}
#[cfg(feature = "Ice+FloorLightTile+Pool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Ice::FloorLightTile_Pool => "Ice"
    ."FloorLightTile/Pool"
);
#[cfg(feature = "Ice+FloorLightTile+Pool")]
impl std::ops::Deref for crate::Ice::FloorLightTile_Pool {
    type Target = crate::Zenject::MonoMemoryPool_1<*mut crate::Ice::FloorLightTile>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Ice+FloorLightTile+Pool")]
impl std::ops::DerefMut for crate::Ice::FloorLightTile_Pool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Ice+FloorLightTile+Pool")]
impl crate::Ice::FloorLightTile_Pool {
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
#[cfg(feature = "Ice+FloorLightTile+Pool")]
impl quest_hook::libil2cpp::ObjectType for crate::Ice::FloorLightTile_Pool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
