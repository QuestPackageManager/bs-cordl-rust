#[cfg(feature = "HMUI+ViewControllerTransitionHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct ViewControllerTransitionHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "HMUI+ViewControllerTransitionHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::ViewControllerTransitionHelpers => "HMUI"
    ."ViewControllerTransitionHelpers"
);
#[cfg(feature = "HMUI+ViewControllerTransitionHelpers")]
impl std::ops::Deref for crate::HMUI::ViewControllerTransitionHelpers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ViewControllerTransitionHelpers")]
impl std::ops::DerefMut for crate::HMUI::ViewControllerTransitionHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ViewControllerTransitionHelpers")]
impl crate::HMUI::ViewControllerTransitionHelpers {
    pub const kHorizontalTransitionMoveOffset: f32 = 2f32;
    pub const kTransitionDuration: f32 = 0.4f32;
    pub const kVerticalTransitionMoveOffset: f32 = 0.5f32;
    #[cfg(feature = "HMUI+ViewControllerTransitionHelpers+_AnimationCoroutine_d__8")]
    pub type _AnimationCoroutine_d__8 = crate::HMUI::ViewControllerTransitionHelpers__AnimationCoroutine_d__8;
    #[cfg(feature = "HMUI+ViewControllerTransitionHelpers+_DoDismissTransition_d__4")]
    pub type _DoDismissTransition_d__4 = crate::HMUI::ViewControllerTransitionHelpers__DoDismissTransition_d__4;
    #[cfg(feature = "HMUI+ViewControllerTransitionHelpers+_DoHorizontalTransition_d__5")]
    pub type _DoHorizontalTransition_d__5 = crate::HMUI::ViewControllerTransitionHelpers__DoHorizontalTransition_d__5;
    #[cfg(feature = "HMUI+ViewControllerTransitionHelpers+_DoPresentTransition_d__3")]
    pub type _DoPresentTransition_d__3 = crate::HMUI::ViewControllerTransitionHelpers__DoPresentTransition_d__3;
    #[cfg(feature = "HMUI+ViewControllerTransitionHelpers+_DoVerticalTransition_d__6")]
    pub type _DoVerticalTransition_d__6 = crate::HMUI::ViewControllerTransitionHelpers__DoVerticalTransition_d__6;
    #[cfg(feature = "HMUI+ViewControllerTransitionHelpers+__c__DisplayClass5_0")]
    pub type __c__DisplayClass5_0 = crate::HMUI::ViewControllerTransitionHelpers___c__DisplayClass5_0;
    #[cfg(feature = "HMUI+ViewControllerTransitionHelpers+__c__DisplayClass6_0")]
    pub type __c__DisplayClass6_0 = crate::HMUI::ViewControllerTransitionHelpers___c__DisplayClass6_0;
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
#[cfg(feature = "HMUI+ViewControllerTransitionHelpers")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::ViewControllerTransitionHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
