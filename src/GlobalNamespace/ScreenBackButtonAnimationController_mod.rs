#[cfg(feature = "ScreenBackButtonAnimationController")]
#[repr(C)]
#[derive(Debug)]
pub struct ScreenBackButtonAnimationController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _animator: *mut crate::UnityEngine::Animator,
    pub _animationHashes: *mut crate::System::Collections::Generic::Dictionary_2<
        crate::GlobalNamespace::ScreenBackButtonAnimationController_AnimationType,
        i32,
    >,
}
#[cfg(feature = "ScreenBackButtonAnimationController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ScreenBackButtonAnimationController => ""
    ."ScreenBackButtonAnimationController"
);
#[cfg(feature = "ScreenBackButtonAnimationController")]
impl std::ops::Deref for crate::GlobalNamespace::ScreenBackButtonAnimationController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ScreenBackButtonAnimationController")]
impl std::ops::DerefMut for crate::GlobalNamespace::ScreenBackButtonAnimationController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ScreenBackButtonAnimationController")]
impl crate::GlobalNamespace::ScreenBackButtonAnimationController {
    #[cfg(feature = "ScreenBackButtonAnimationController+AnimationType")]
    pub type AnimationType = crate::GlobalNamespace::ScreenBackButtonAnimationController_AnimationType;
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn StartAnimation(
        &mut self,
        animationType: crate::GlobalNamespace::ScreenBackButtonAnimationController_AnimationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartAnimation", (animationType))?;
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
#[cfg(feature = "ScreenBackButtonAnimationController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ScreenBackButtonAnimationController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ScreenBackButtonAnimationController+AnimationType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenBackButtonAnimationController_AnimationType {
    FadeIn = 0i32,
    FadeOut = 1i32,
    MoveIn = 2i32,
    MoveIn2 = 4i32,
    MoveOut = 3i32,
    MoveOut2 = 5i32,
}
#[cfg(feature = "ScreenBackButtonAnimationController+AnimationType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ScreenBackButtonAnimationController_AnimationType => ""
    ."ScreenBackButtonAnimationController/AnimationType"
);
