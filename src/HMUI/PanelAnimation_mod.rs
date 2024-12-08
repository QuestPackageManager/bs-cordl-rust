#[cfg(feature = "HMUI+PanelAnimation")]
#[repr(C)]
#[derive(Debug)]
pub struct PanelAnimation {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
}
#[cfg(feature = "HMUI+PanelAnimation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::PanelAnimation => "HMUI"."PanelAnimation"
);
#[cfg(feature = "HMUI+PanelAnimation")]
impl std::ops::Deref for crate::HMUI::PanelAnimation {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+PanelAnimation")]
impl std::ops::DerefMut for crate::HMUI::PanelAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+PanelAnimation")]
impl crate::HMUI::PanelAnimation {
    #[cfg(feature = "HMUI+PanelAnimation+_AnimationCoroutine_d__1")]
    pub type _AnimationCoroutine_d__1 = crate::HMUI::PanelAnimation__AnimationCoroutine_d__1;
    pub fn AnimationCoroutine(
        &mut self,
        duration: f32,
        canvasGroup: *mut crate::UnityEngine::CanvasGroup,
        parentCanvasGroup: *mut crate::UnityEngine::CanvasGroup,
        scaleXAnimationCurve: *mut crate::UnityEngine::AnimationCurve,
        scaleYAnimationCurve: *mut crate::UnityEngine::AnimationCurve,
        alphaAnimationCurve: *mut crate::UnityEngine::AnimationCurve,
        parentAlphaAnimationCurve: *mut crate::UnityEngine::AnimationCurve,
        finishedCallback: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke(
                "AnimationCoroutine",
                (
                    duration,
                    canvasGroup,
                    parentCanvasGroup,
                    scaleXAnimationCurve,
                    scaleYAnimationCurve,
                    alphaAnimationCurve,
                    parentAlphaAnimationCurve,
                    finishedCallback,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn StartAnimation(
        &mut self,
        canvasGroup: *mut crate::UnityEngine::CanvasGroup,
        parentCanvasGroup: *mut crate::UnityEngine::CanvasGroup,
        duration: f32,
        scaleXAnimationCurve: *mut crate::UnityEngine::AnimationCurve,
        scaleYAnimationCurve: *mut crate::UnityEngine::AnimationCurve,
        alphaAnimationCurve: *mut crate::UnityEngine::AnimationCurve,
        parentAlphaAnimationCurve: *mut crate::UnityEngine::AnimationCurve,
        finishedCallback: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "StartAnimation",
                (
                    canvasGroup,
                    parentCanvasGroup,
                    duration,
                    scaleXAnimationCurve,
                    scaleYAnimationCurve,
                    alphaAnimationCurve,
                    parentAlphaAnimationCurve,
                    finishedCallback,
                ),
            )?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "HMUI+PanelAnimation")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::PanelAnimation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
