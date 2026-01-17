#[cfg(feature = "cordl_class_OVRMicrogesturesSample")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct OVRMicrogesturesSample {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub leftGestureSource:
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRMicrogestureEventSource>,
    pub rightGestureSource:
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRMicrogestureEventSource>,
    pub leftGestureLabel: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Text>,
    pub rightGestureLabel: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Text>,
    pub gestureShowDuration: f32,
    pub leftArrowL: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
    pub rightArrowL: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
    pub upArrowL: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
    pub downArrowL: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
    pub selectIconL: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
    pub leftArrowR: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
    pub rightArrowR: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
    pub upArrowR: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
    pub downArrowR: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
    pub selectIconR: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
    pub initialColor: crate::UnityEngine::Color,
    pub highlightColor: crate::UnityEngine::Color,
    pub highlightDuration: f32,
    pub highlightCoroutines: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Coroutine>,
        >,
    >,
}
#[cfg(feature = "cordl_class_OVRMicrogesturesSample")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRMicrogesturesSample {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRMicrogesturesSample";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "OVRMicrogesturesSample")]
impl std::ops::Deref for crate::GlobalNamespace::OVRMicrogesturesSample {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRMicrogesturesSample")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRMicrogesturesSample {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRMicrogesturesSample")]
impl crate::GlobalNamespace::OVRMicrogesturesSample {
    pub fn HighlightGesture(
        &mut self,
        hand: crate::GlobalNamespace::OVRPlugin_Hand,
        gesture: crate::GlobalNamespace::OVRHand_MicrogestureType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::GlobalNamespace::OVRPlugin_Hand,
                        crate::GlobalNamespace::OVRHand_MicrogestureType,
                    ), quest_hook::libil2cpp::Void, 2usize>("HighlightGesture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "HighlightGesture",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (hand, gesture))? };
        Ok(__cordl_ret.into())
    }
    pub fn HighlightIconCoroutine(
        &mut self,
        navIcon: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::IEnumerator,
                        >,
                        1usize,
                    >("HighlightIconCoroutine")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HighlightIconCoroutine", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator> =
            unsafe { cordl_method_info.invoke_unchecked(self, (navIcon))? };
        Ok(__cordl_ret.into())
    }
    pub fn HighlightIcon_Image0(
        &mut self,
        icon: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("HighlightIcon")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HighlightIcon", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (icon))? };
        Ok(__cordl_ret.into())
    }
    pub fn HighlightIcon__cordl_bool1(
        &mut self,
        navIcon: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
        state: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
                        bool,
                    ), quest_hook::libil2cpp::Void, 2usize>("HighlightIcon")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "HighlightIcon",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (navIcon, state))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnGestureRecognized(
        &mut self,
        hand: crate::GlobalNamespace::OVRPlugin_Hand,
        gesture: crate::GlobalNamespace::OVRHand_MicrogestureType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::GlobalNamespace::OVRPlugin_Hand,
                        crate::GlobalNamespace::OVRHand_MicrogestureType,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "OnGestureRecognized"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnGestureRecognized",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (hand, gesture))? };
        Ok(__cordl_ret.into())
    }
    pub fn ShowGestureLabel(
        &mut self,
        gestureLabel: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Text>,
        label: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Text>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>, 2usize>(
                        "ShowGestureLabel",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ShowGestureLabel",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator> =
            unsafe { cordl_method_info.invoke_unchecked(self, (gestureLabel, label))? };
        Ok(__cordl_ret.into())
    }
    pub fn ShowRecognizedGestureLabel(
        &mut self,
        gestureLabel: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Text>,
        label: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Text>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "ShowRecognizedGestureLabel"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ShowRecognizedGestureLabel",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (gestureLabel, label))? };
        Ok(__cordl_ret.into())
    }
    pub fn Start(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Start")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Start",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _Start_b__19_0(
        &mut self,
        gesture: crate::GlobalNamespace::OVRHand_MicrogestureType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::OVRHand_MicrogestureType),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("<Start>b__19_0")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "<Start>b__19_0", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (gesture))? };
        Ok(__cordl_ret.into())
    }
    pub fn _Start_b__19_1(
        &mut self,
        gesture: crate::GlobalNamespace::OVRHand_MicrogestureType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::OVRHand_MicrogestureType),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("<Start>b__19_1")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "<Start>b__19_1", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (gesture))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OVRMicrogesturesSample")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRMicrogesturesSample {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
