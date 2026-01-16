#[cfg(feature = "cordl_class_UnityEngine+InputForUI+InputEventPartialProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct InputEventPartialProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _ev: quest_hook::libil2cpp::Gc<crate::UnityEngine::Event>,
    pub _operatingSystemFamily: crate::UnityEngine::OperatingSystemFamily,
    pub _keyboardButtonsState: crate::UnityEngine::InputForUI::KeyEvent_ButtonsState,
    pub _eventModifiers: crate::UnityEngine::InputForUI::EventModifiers,
    pub _sendNavigationEventOnTabKey: bool,
    pub _IMGUICommandToInputForUICommandType: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IDictionary_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            crate::UnityEngine::InputForUI::CommandEvent_Command,
        >,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+InputForUI+InputEventPartialProvider")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::InputForUI::InputEventPartialProvider
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputForUI";
    const CLASS_NAME: &'static str = "InputEventPartialProvider";
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
#[cfg(feature = "UnityEngine+InputForUI+InputEventPartialProvider")]
impl std::ops::Deref for crate::UnityEngine::InputForUI::InputEventPartialProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputForUI+InputEventPartialProvider")]
impl std::ops::DerefMut for crate::UnityEngine::InputForUI::InputEventPartialProvider {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputForUI+InputEventPartialProvider")]
impl crate::UnityEngine::InputForUI::InputEventPartialProvider {
    pub fn GetTimestamp(
        &mut self,
        ev: quest_hook::libil2cpp::ByRefMut<quest_hook::libil2cpp::Gc<crate::UnityEngine::Event>>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::IntegerTime::DiscreteTime> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Event>,
                    >), crate::Unity::IntegerTime::DiscreteTime, 1usize>(
                        "GetTimestamp"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetTimestamp",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::IntegerTime::DiscreteTime =
            unsafe { cordl_method_info.invoke_unchecked(self, (ev))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Initialize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Initialize",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnFocusChanged(
        &mut self,
        focus: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("OnFocusChanged")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnFocusChanged",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (focus))? };
        Ok(__cordl_ret.into())
    }
    pub fn RequestCurrentState(
        &mut self,
        _cordl_type: crate::UnityEngine::InputForUI::Event_Type,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::InputForUI::Event_Type), bool, 1usize>(
                        "RequestCurrentState",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RequestCurrentState",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn SendNextOrPreviousNavigationEventOnTabKeyDownEvent(
        &mut self,
        ev: quest_hook::libil2cpp::ByRefMut<quest_hook::libil2cpp::Gc<crate::UnityEngine::Event>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Event>,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "SendNextOrPreviousNavigationEventOnTabKeyDownEvent",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SendNextOrPreviousNavigationEventOnTabKeyDownEvent",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (ev))? };
        Ok(__cordl_ret.into())
    }
    pub fn Shutdown(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Shutdown")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Shutdown",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ToCommandEvent(
        &mut self,
        ev: quest_hook::libil2cpp::ByRefMut<quest_hook::libil2cpp::Gc<crate::UnityEngine::Event>>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputForUI::CommandEvent> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Event>,
                    >), crate::UnityEngine::InputForUI::CommandEvent, 1usize>(
                        "ToCommandEvent"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ToCommandEvent",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputForUI::CommandEvent =
            unsafe { cordl_method_info.invoke_unchecked(self, (ev))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToKeyEvent(
        &mut self,
        ev: quest_hook::libil2cpp::ByRefMut<quest_hook::libil2cpp::Gc<crate::UnityEngine::Event>>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputForUI::KeyEvent> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Event>,
                    >), crate::UnityEngine::InputForUI::KeyEvent, 1usize>(
                        "ToKeyEvent"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ToKeyEvent",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputForUI::KeyEvent =
            unsafe { cordl_method_info.invoke_unchecked(self, (ev))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToTextInputEvent(
        &mut self,
        ev: quest_hook::libil2cpp::ByRefMut<quest_hook::libil2cpp::Gc<crate::UnityEngine::Event>>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputForUI::TextInputEvent> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Event>,
                    >), crate::UnityEngine::InputForUI::TextInputEvent, 1usize>(
                        "ToTextInputEvent"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ToTextInputEvent",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputForUI::TextInputEvent =
            unsafe { cordl_method_info.invoke_unchecked(self, (ev))? };
        Ok(__cordl_ret.into())
    }
    pub fn Update(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Update")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Update",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateEventModifiers(
        &mut self,
        ev: quest_hook::libil2cpp::ByRefMut<quest_hook::libil2cpp::Gc<crate::UnityEngine::Event>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Event>,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "UpdateEventModifiers"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdateEventModifiers",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (ev))? };
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
#[cfg(feature = "cordl_class_UnityEngine+InputForUI+InputEventPartialProvider")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::InputForUI::InputEventPartialProvider
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputForUI+InputEventPartialProvider")]
impl AsRef<crate::UnityEngine::InputForUI::IEventProviderImpl>
    for crate::UnityEngine::InputForUI::InputEventPartialProvider
{
    fn as_ref(&self) -> &crate::UnityEngine::InputForUI::IEventProviderImpl {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputForUI+InputEventPartialProvider")]
impl AsMut<crate::UnityEngine::InputForUI::IEventProviderImpl>
    for crate::UnityEngine::InputForUI::InputEventPartialProvider
{
    fn as_mut(&mut self) -> &mut crate::UnityEngine::InputForUI::IEventProviderImpl {
        unsafe { std::mem::transmute(self) }
    }
}
