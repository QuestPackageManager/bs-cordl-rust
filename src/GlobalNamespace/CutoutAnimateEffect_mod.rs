#[cfg(feature = "CutoutAnimateEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct CutoutAnimateEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _cuttoutEffects: *mut quest_hook::libil2cpp::Il2CppArray<*mut CutoutEffect>,
    pub _transitionCurve: *mut crate::UnityEngine::AnimationCurve,
    pub _animating_k__BackingField: bool,
}
#[cfg(feature = "CutoutAnimateEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for CutoutAnimateEffect => ""."CutoutAnimateEffect"
);
#[cfg(feature = "CutoutAnimateEffect")]
impl std::ops::Deref for CutoutAnimateEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CutoutAnimateEffect")]
impl std::ops::DerefMut for CutoutAnimateEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CutoutAnimateEffect")]
impl CutoutAnimateEffect {
    #[cfg(feature = "CutoutAnimateEffect+_AnimateToCutoutCoroutine_d__7")]
    pub type _AnimateToCutoutCoroutine_d__7 = crate::GlobalNamespace::CutoutAnimateEffect__AnimateToCutoutCoroutine_d__7;
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
        Ok(__cordl_ret)
    }
    pub fn AnimateToCutoutCoroutine(
        &mut self,
        cutoutStart: f32,
        cutoutEnd: f32,
        duration: f32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("AnimateToCutoutCoroutine", (cutoutStart, cutoutEnd, duration))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ResetEffect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetEffect", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetCutout(
        &mut self,
        cutout: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCutout", (cutout))?;
        Ok(__cordl_ret)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
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
    pub fn get_animating(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_animating", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_animating(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_animating", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "CutoutAnimateEffect")]
impl quest_hook::libil2cpp::ObjectType for CutoutAnimateEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
