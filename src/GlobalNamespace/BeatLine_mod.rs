#[cfg(feature = "BeatLine")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatLine {
    __cordl_parent: LightWithIdMonoBehaviour,
    pub _tubeBloomPrePassLight: *mut TubeBloomPrePassLight,
    pub _arriveFadeCurve: *mut crate::UnityEngine::AnimationCurve,
    pub _jumpFadeCurve: *mut crate::UnityEngine::AnimationCurve,
    pub _alphaMul: f32,
    pub _maxAlpha: f32,
    pub _highlights: *mut crate::System::Collections::Generic::List_1<
        crate::GlobalNamespace::BeatLine_HighlightData,
    >,
    pub _color: crate::UnityEngine::Color,
    pub _rotation: f32,
}
#[cfg(feature = "BeatLine")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BeatLine => ""."BeatLine"
);
#[cfg(feature = "BeatLine")]
impl std::ops::Deref for BeatLine {
    type Target = LightWithIdMonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatLine")]
impl std::ops::DerefMut for BeatLine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatLine")]
impl BeatLine {
    #[cfg(feature = "BeatLine+Pool")]
    pub type Pool = crate::GlobalNamespace::BeatLine_Pool;
    #[cfg(feature = "BeatLine+HighlightData")]
    pub type HighlightData = crate::GlobalNamespace::BeatLine_HighlightData;
    pub fn AddHighlight(
        &mut self,
        startTime: f32,
        arriveDuration: f32,
        jumpDuration: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddHighlight", (startTime, arriveDuration, jumpDuration))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
    pub fn get_isFinished(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isFinished", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rotation(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_rotation", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatLine")]
impl quest_hook::libil2cpp::ObjectType for BeatLine {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatLine+HighlightData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BeatLine_HighlightData {
    pub startTime: f32,
    pub arriveDuration: f32,
    pub halfJumpDuration: f32,
}
#[cfg(feature = "BeatLine+HighlightData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatLine_HighlightData => ""
    ."BeatLine/HighlightData"
);
#[cfg(feature = "BeatLine+HighlightData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::BeatLine_HighlightData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatLine+HighlightData")]
impl crate::GlobalNamespace::BeatLine_HighlightData {}
#[cfg(feature = "BeatLine+Pool")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatLine_Pool {
    __cordl_parent: crate::Zenject::MonoMemoryPool_1<*mut BeatLine>,
}
#[cfg(feature = "BeatLine+Pool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatLine_Pool => ""
    ."BeatLine/Pool"
);
#[cfg(feature = "BeatLine+Pool")]
impl std::ops::Deref for crate::GlobalNamespace::BeatLine_Pool {
    type Target = crate::Zenject::MonoMemoryPool_1<*mut BeatLine>;
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
#[cfg(feature = "BeatLine+Pool")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BeatLine_Pool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}