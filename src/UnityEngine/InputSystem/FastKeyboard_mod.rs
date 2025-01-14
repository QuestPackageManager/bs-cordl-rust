#[cfg(feature = "UnityEngine+InputSystem+FastKeyboard")]
#[repr(C)]
#[derive(Debug)]
pub struct FastKeyboard {
    __cordl_parent: crate::UnityEngine::InputSystem::Keyboard,
}
#[cfg(feature = "UnityEngine+InputSystem+FastKeyboard")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::FastKeyboard {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "FastKeyboard";
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
#[cfg(feature = "UnityEngine+InputSystem+FastKeyboard")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::FastKeyboard {
    type Target = crate::UnityEngine::InputSystem::Keyboard;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+FastKeyboard")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::FastKeyboard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+FastKeyboard")]
impl crate::UnityEngine::InputSystem::FastKeyboard {
    pub const metadata: &'static str = ";AnyKey;Button;Axis;Key;DiscreteButton;Keyboard";
    pub fn Initialize_ctrlKeyboard0(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboard0")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboard0", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboard1(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboard1")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboard1", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboard2(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboard2")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboard2", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboard3(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboard3")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboard3", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboard4(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboard4")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboard4", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboard5(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboard5")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboard5", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboard6(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboard6")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboard6", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboard7(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboard7")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboard7", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboard8(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboard8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboard8", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboard9(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboard9")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboard9", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardIMESelected(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::ButtonControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardIMESelected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardIMESelected", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = unsafe { method.invoke_unchecked(self, (kButtonLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardOEM1(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardOEM1")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardOEM1", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardOEM2(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardOEM2")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardOEM2", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardOEM3(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardOEM3")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardOEM3", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardOEM4(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardOEM4")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardOEM4", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardOEM5(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardOEM5")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardOEM5", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboarda(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboarda")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboarda", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardalt(
        &mut self,
        kDiscreteButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DiscreteButtonControl,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::DiscreteButtonControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardalt")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardalt", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DiscreteButtonControl,
        > = unsafe { method.invoke_unchecked(self, (kDiscreteButtonLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardanyKey(
        &mut self,
        kAnyKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AnyKeyControl,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::AnyKeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardanyKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardanyKey", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AnyKeyControl,
        > = unsafe { method.invoke_unchecked(self, (kAnyKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardb(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardb")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardb", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardbackquote(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardbackquote")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardbackquote", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardbackslash(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardbackslash")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardbackslash", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardbackspace(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardbackspace")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardbackspace", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardc(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardc")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardc", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardcapsLock(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardcapsLock")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardcapsLock", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardcomma(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardcomma")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardcomma", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardcontextMenu(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardcontextMenu")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardcontextMenu", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardctrl(
        &mut self,
        kDiscreteButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DiscreteButtonControl,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::DiscreteButtonControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardctrl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardctrl", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DiscreteButtonControl,
        > = unsafe { method.invoke_unchecked(self, (kDiscreteButtonLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardd(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardd", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboarddelete(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboarddelete")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboarddelete", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboarddownArrow(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboarddownArrow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboarddownArrow", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboarde(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboarde")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboarde", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardend(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardend")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardend", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardenter(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardenter")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardenter", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardequals(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardequals")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardequals", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardescape(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardescape")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardescape", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardf(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardf")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardf", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardf1(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardf1")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardf1", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardf10(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardf10")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardf10", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardf11(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardf11")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardf11", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardf12(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardf12")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardf12", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardf2(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardf2")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardf2", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardf3(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardf3")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardf3", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardf4(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardf4")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardf4", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardf5(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardf5")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardf5", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardf6(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardf6")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardf6", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardf7(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardf7")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardf7", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardf8(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardf8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardf8", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardf9(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardf9")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardf9", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardg(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardg")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardg", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardh(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardh")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardh", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardhome(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardhome")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardhome", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardi(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardi")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardi", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardinsert(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardinsert")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardinsert", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardj(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardj")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardj", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardk(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardk")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardk", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardl(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardl", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardleftAlt(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardleftAlt")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardleftAlt", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardleftArrow(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardleftArrow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardleftArrow", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardleftBracket(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardleftBracket")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardleftBracket", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardleftCtrl(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardleftCtrl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardleftCtrl", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardleftMeta(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardleftMeta")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardleftMeta", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardleftShift(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardleftShift")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardleftShift", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardm(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardm")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardm", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardminus(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardminus")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardminus", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardn(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardn")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardn", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardnumLock(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardnumLock")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardnumLock", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardnumpad0(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardnumpad0")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardnumpad0", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardnumpad1(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardnumpad1")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardnumpad1", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardnumpad2(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardnumpad2")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardnumpad2", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardnumpad3(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardnumpad3")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardnumpad3", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardnumpad4(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardnumpad4")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardnumpad4", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardnumpad5(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardnumpad5")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardnumpad5", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardnumpad6(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardnumpad6")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardnumpad6", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardnumpad7(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardnumpad7")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardnumpad7", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardnumpad8(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardnumpad8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardnumpad8", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardnumpad9(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardnumpad9")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardnumpad9", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardnumpadDivide(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardnumpadDivide")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardnumpadDivide", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardnumpadEnter(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardnumpadEnter")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardnumpadEnter", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardnumpadEquals(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardnumpadEquals")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardnumpadEquals", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardnumpadMinus(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardnumpadMinus")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardnumpadMinus", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardnumpadMultiply(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardnumpadMultiply")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardnumpadMultiply", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardnumpadPeriod(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardnumpadPeriod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardnumpadPeriod", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardnumpadPlus(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardnumpadPlus")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardnumpadPlus", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardo(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardo", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardp(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardp")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardp", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardpageDown(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardpageDown")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardpageDown", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardpageUp(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardpageUp")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardpageUp", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardpause(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardpause")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardpause", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardperiod(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardperiod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardperiod", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardprintScreen(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardprintScreen")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardprintScreen", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardq(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardq")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardq", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardquote(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardquote")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardquote", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardr(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardr")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardr", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardrightAlt(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardrightAlt")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardrightAlt", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardrightArrow(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardrightArrow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardrightArrow", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardrightBracket(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardrightBracket")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardrightBracket", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardrightCtrl(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardrightCtrl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardrightCtrl", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardrightMeta(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardrightMeta")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardrightMeta", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardrightShift(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardrightShift")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardrightShift", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboards(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboards")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboards", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardscrollLock(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardscrollLock")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardscrollLock", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardsemicolon(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardsemicolon")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardsemicolon", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardshift(
        &mut self,
        kDiscreteButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DiscreteButtonControl,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::DiscreteButtonControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardshift")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardshift", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DiscreteButtonControl,
        > = unsafe { method.invoke_unchecked(self, (kDiscreteButtonLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardslash(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardslash")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardslash", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardspace(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardspace")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardspace", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardt(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardt")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardt", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardtab(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardtab")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardtab", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardu(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardu")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardu", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardupArrow(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardupArrow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardupArrow", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardv(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardv")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardv", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardw(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardw")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardw", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardx(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardx")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardx", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardy(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardy", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlKeyboardz(
        &mut self,
        kKeyLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::KeyControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::Utilities::InternedString,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::KeyControl,
                >,
                2usize,
            >("Initialize_ctrlKeyboardz")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Initialize_ctrlKeyboardz", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::KeyControl,
        > = unsafe { method.invoke_unchecked(self, (kKeyLayout, parent)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+FastKeyboard")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::FastKeyboard {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
