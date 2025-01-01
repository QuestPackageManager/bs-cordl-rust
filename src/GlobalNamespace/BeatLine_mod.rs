#[cfg(feature = "BeatLine")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatLine {
    __cordl_parent: crate::GlobalNamespace::LightWithIdMonoBehaviour,
    pub _tubeBloomPrePassLight: *mut crate::GlobalNamespace::TubeBloomPrePassLight,
    pub _arriveFadeCurve: *mut crate::UnityEngine::AnimationCurve,
    pub _jumpFadeCurve: *mut crate::UnityEngine::AnimationCurve,
    pub _alphaMul: f32,
    pub _maxAlpha: f32,
    pub _variableMovementDataProvider: *mut crate::GlobalNamespace::VariableMovementDataProvider,
    pub _highlightTimes: *mut crate::System::Collections::Generic::List_1<f32>,
    pub _color: crate::UnityEngine::Color,
    pub _rotation: f32,
}
#[cfg(feature = "BeatLine")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatLine => ""."BeatLine"
);
#[cfg(feature = "BeatLine")]
impl std::ops::Deref for crate::GlobalNamespace::BeatLine {
    type Target = crate::GlobalNamespace::LightWithIdMonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatLine")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatLine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatLine")]
impl crate::GlobalNamespace::BeatLine {
    #[cfg(feature = "BeatLine+Pool")]
    pub type Pool = crate::GlobalNamespace::BeatLine_Pool;
    pub fn AddHighlight(
        &mut self,
        noteTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddHighlight", (noteTime))?;
        Ok(__cordl_ret.into())
    }
    pub fn ColorWasSet(
        &mut self,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ColorWasSet", (color))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        position: crate::UnityEngine::Vector3,
        rotation: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (position, rotation))?;
        Ok(__cordl_ret.into())
    }
    pub fn ManualUpdate(
        &mut self,
        songTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ManualUpdate", (songTime))?;
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
    pub fn get_isFinished(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isFinished", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rotation(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_rotation", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatLine")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BeatLine {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatLine+Pool")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatLine_Pool {
    __cordl_parent: crate::Zenject::MonoMemoryPool_1<
        *mut crate::GlobalNamespace::BeatLine,
    >,
}
#[cfg(feature = "BeatLine+Pool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatLine_Pool => ""
    ."BeatLine/Pool"
);
#[cfg(feature = "BeatLine+Pool")]
impl std::ops::Deref for crate::GlobalNamespace::BeatLine_Pool {
    type Target = crate::Zenject::MonoMemoryPool_1<
        *mut crate::GlobalNamespace::BeatLine,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatLine+Pool")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatLine_Pool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatLine+Pool")]
impl crate::GlobalNamespace::BeatLine_Pool {
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
#[cfg(feature = "BeatLine+Pool")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BeatLine_Pool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
