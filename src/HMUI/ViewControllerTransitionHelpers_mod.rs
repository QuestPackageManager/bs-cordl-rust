#[cfg(feature = "HMUI+ViewControllerTransitionHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct ViewControllerTransitionHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "HMUI+ViewControllerTransitionHelpers")]
unsafe impl quest_hook::libil2cpp::Type
for crate::HMUI::ViewControllerTransitionHelpers {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HMUI";
    const CLASS_NAME: &'static str = "ViewControllerTransitionHelpers";
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
    pub fn AnimationCoroutine(
        transitionAnimation: quest_hook::libil2cpp::Gc<crate::System::Action_1<f32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::ViewControllerTransitionHelpers as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action_1<f32>>),
                quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
                1usize,
            >("AnimationCoroutine")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::ViewControllerTransitionHelpers as
                    quest_hook::libil2cpp::Type > ::class(), "AnimationCoroutine", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = unsafe { method.invoke_unchecked((), (transitionAnimation))? };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::ViewControllerTransitionHelpers as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
                    quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
                    crate::HMUI::ViewController_AnimationDirection,
                    f32,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
                4usize,
            >("DoDismissTransition")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::ViewControllerTransitionHelpers as
                    quest_hook::libil2cpp::Type > ::class(), "DoDismissTransition",
                    4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        toPresentViewController,
                        toDismissViewController,
                        animationDirection,
                        moveOffsetMultiplier,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DoHorizontalTransition(
        toPresentViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        toDismissViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        moveOffsetMultiplier: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::ViewControllerTransitionHelpers as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
                    quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
                    f32,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
                3usize,
            >("DoHorizontalTransition")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::ViewControllerTransitionHelpers as
                    quest_hook::libil2cpp::Type > ::class(), "DoHorizontalTransition",
                    3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        toPresentViewController,
                        toDismissViewController,
                        moveOffsetMultiplier,
                    ),
                )?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::ViewControllerTransitionHelpers as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
                    quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
                    crate::HMUI::ViewController_AnimationDirection,
                    f32,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
                4usize,
            >("DoPresentTransition")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::ViewControllerTransitionHelpers as
                    quest_hook::libil2cpp::Type > ::class(), "DoPresentTransition",
                    4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        toPresentViewController,
                        toDismissViewController,
                        animationDirection,
                        moveOffsetMultiplier,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DoVerticalTransition(
        toPresentViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        toDismissViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        moveOffsetMultiplier: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::ViewControllerTransitionHelpers as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
                    quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
                    f32,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
                3usize,
            >("DoVerticalTransition")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::ViewControllerTransitionHelpers as
                    quest_hook::libil2cpp::Type > ::class(), "DoVerticalTransition",
                    3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        toPresentViewController,
                        toDismissViewController,
                        moveOffsetMultiplier,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ImmediateTransition(
        toPresentViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        toDismissViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::ViewControllerTransitionHelpers as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
                    quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ImmediateTransition")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::ViewControllerTransitionHelpers as
                    quest_hook::libil2cpp::Type > ::class(), "ImmediateTransition",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (toPresentViewController, toDismissViewController),
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
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::ViewControllerTransitionHelpers as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::ViewControllerTransitionHelpers as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
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
