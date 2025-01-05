#[cfg(feature = "HMUI+ViewControllerTransitionHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct ViewControllerTransitionHelpers {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "HMUI+ViewControllerTransitionHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::ViewControllerTransitionHelpers => "HMUI"
    ."ViewControllerTransitionHelpers"
);
#[cfg(feature = "HMUI+ViewControllerTransitionHelpers")]
impl std::ops::Deref for crate::HMUI::ViewControllerTransitionHelpers {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
    pub fn AnimationCoroutine(
        transitionAnimation: quest_hook::libil2cpp::Gc<f32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AnimationCoroutine", (transitionAnimation))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoDismissTransition(
        toPresentViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        toDismissViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        animationDirection: crate::HMUI::ViewController_AnimationDirection,
        moveOffsetMultiplier: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "DoDismissTransition",
                (
                    toPresentViewController,
                    toDismissViewController,
                    animationDirection,
                    moveOffsetMultiplier,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn DoHorizontalTransition(
        toPresentViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        toDismissViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        moveOffsetMultiplier: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "DoHorizontalTransition",
                (toPresentViewController, toDismissViewController, moveOffsetMultiplier),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn DoPresentTransition(
        toPresentViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        toDismissViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        animationDirection: crate::HMUI::ViewController_AnimationDirection,
        moveOffsetMultiplier: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "DoPresentTransition",
                (
                    toPresentViewController,
                    toDismissViewController,
                    animationDirection,
                    moveOffsetMultiplier,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn DoVerticalTransition(
        toPresentViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        toDismissViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        moveOffsetMultiplier: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "DoVerticalTransition",
                (toPresentViewController, toDismissViewController, moveOffsetMultiplier),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ImmediateTransition(
        toPresentViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        toDismissViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ImmediateTransition",
                (toPresentViewController, toDismissViewController),
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
