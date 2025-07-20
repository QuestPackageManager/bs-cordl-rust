#[cfg(feature = "HMUI+PanelAnimation")]
#[repr(C)]
#[derive(Debug)]
pub struct PanelAnimation {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
}
#[cfg(feature = "HMUI+PanelAnimation")]
unsafe impl quest_hook::libil2cpp::Type for crate::HMUI::PanelAnimation {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HMUI";
    const CLASS_NAME: &'static str = "PanelAnimation";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "HMUI+PanelAnimation")]
impl std::ops::Deref for crate::HMUI::PanelAnimation {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+PanelAnimation")]
impl std::ops::DerefMut for crate::HMUI::PanelAnimation {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            f32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::CanvasGroup>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::CanvasGroup>,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::AnimationCurve,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::AnimationCurve,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::AnimationCurve,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::AnimationCurve,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Action>,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::IEnumerator,
                        >,
                        8usize,
                    >("AnimationCoroutine")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AnimationCoroutine", 8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = unsafe {
            method
                .invoke_unchecked(
                    self,
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
                )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::CanvasGroup>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::CanvasGroup>,
                            f32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::AnimationCurve,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::AnimationCurve,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::AnimationCurve,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::AnimationCurve,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Action>,
                        ),
                        quest_hook::libil2cpp::Void,
                        8usize,
                    >("StartAnimation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "StartAnimation", 8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
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
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
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
