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
    pub fn AnimationCoroutine(
        &mut self,
        duration: f32,
        canvasGroup: quest_hook::libil2cpp::Gc<crate::UnityEngine::CanvasGroup>,
        parentCanvasGroup: quest_hook::libil2cpp::Gc<crate::UnityEngine::CanvasGroup>,
        scaleXAnimationCurve: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AnimationCurve,
        >,
        scaleYAnimationCurve: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AnimationCurve,
        >,
        alphaAnimationCurve: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AnimationCurve,
        >,
        parentAlphaAnimationCurve: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AnimationCurve,
        >,
        finishedCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object
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
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn StartAnimation(
        &mut self,
        canvasGroup: quest_hook::libil2cpp::Gc<crate::UnityEngine::CanvasGroup>,
        parentCanvasGroup: quest_hook::libil2cpp::Gc<crate::UnityEngine::CanvasGroup>,
        duration: f32,
        scaleXAnimationCurve: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AnimationCurve,
        >,
        scaleYAnimationCurve: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AnimationCurve,
        >,
        alphaAnimationCurve: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AnimationCurve,
        >,
        parentAlphaAnimationCurve: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AnimationCurve,
        >,
        finishedCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
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
#[cfg(feature = "HMUI+PanelAnimation")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::PanelAnimation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
