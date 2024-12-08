#[cfg(feature = "HMUI+PanelAnimationSO")]
#[repr(C)]
#[derive(Debug)]
pub struct PanelAnimationSO {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub _duration: f32,
    pub _scaleXAnimationCurve: *mut crate::UnityEngine::AnimationCurve,
    pub _scaleYAnimationCurve: *mut crate::UnityEngine::AnimationCurve,
    pub _alphaAnimationCurve: *mut crate::UnityEngine::AnimationCurve,
    pub _parentAlphaAnimationCurve: *mut crate::UnityEngine::AnimationCurve,
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
    pub fn ExecuteAnimation_GameObject0(
        &mut self,
        go: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExecuteAnimation", (go))?;
        Ok(__cordl_ret)
    }
    pub fn ExecuteAnimation_Action1(
        &mut self,
        go: *mut crate::UnityEngine::GameObject,
        finishedCallback: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExecuteAnimation", (go, finishedCallback))?;
        Ok(__cordl_ret)
    }
    pub fn ExecuteAnimation_CanvasGroup_Action2(
        &mut self,
        go: *mut crate::UnityEngine::GameObject,
        parentCanvasGroup: *mut crate::UnityEngine::CanvasGroup,
        finishedCallback: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExecuteAnimation", (go, parentCanvasGroup, finishedCallback))?;
        Ok(__cordl_ret)
    }
    pub fn ExecuteAnimation_CanvasGroup__cordl_bool_Action3(
        &mut self,
        go: *mut crate::UnityEngine::GameObject,
        parentCanvasGroup: *mut crate::UnityEngine::CanvasGroup,
        instant: bool,
        finishedCallback: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ExecuteAnimation",
                (go, parentCanvasGroup, instant, finishedCallback),
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
#[cfg(feature = "HMUI+PanelAnimationSO")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::PanelAnimationSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
