#[cfg(feature = "HMUI+PanelAnimationSO")]
#[repr(C)]
#[derive(Debug)]
pub struct PanelAnimationSO {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub _duration: f32,
    pub _scaleXAnimationCurve: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AnimationCurve,
    >,
    pub _scaleYAnimationCurve: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AnimationCurve,
    >,
    pub _alphaAnimationCurve: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AnimationCurve,
    >,
    pub _parentAlphaAnimationCurve: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AnimationCurve,
    >,
}
#[cfg(feature = "HMUI+PanelAnimationSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::PanelAnimationSO => "HMUI"
    ."PanelAnimationSO"
);
#[cfg(feature = "HMUI+PanelAnimationSO")]
impl std::ops::Deref for crate::HMUI::PanelAnimationSO {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+PanelAnimationSO")]
impl std::ops::DerefMut for crate::HMUI::PanelAnimationSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+PanelAnimationSO")]
impl crate::HMUI::PanelAnimationSO {
    pub fn ExecuteAnimation_Action1(
        &mut self,
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        finishedCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExecuteAnimation", (go, finishedCallback))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteAnimation_CanvasGroup_Action2(
        &mut self,
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        parentCanvasGroup: quest_hook::libil2cpp::Gc<crate::UnityEngine::CanvasGroup>,
        finishedCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExecuteAnimation", (go, parentCanvasGroup, finishedCallback))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteAnimation_CanvasGroup__cordl_bool_Action3(
        &mut self,
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        parentCanvasGroup: quest_hook::libil2cpp::Gc<crate::UnityEngine::CanvasGroup>,
        instant: bool,
        finishedCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ExecuteAnimation",
                (go, parentCanvasGroup, instant, finishedCallback),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteAnimation_GameObject0(
        &mut self,
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExecuteAnimation", (go))?;
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
#[cfg(feature = "HMUI+PanelAnimationSO")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::PanelAnimationSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
