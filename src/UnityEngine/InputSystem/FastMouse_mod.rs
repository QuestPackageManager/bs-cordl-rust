#[cfg(feature = "UnityEngine+InputSystem+FastMouse")]
#[repr(C)]
#[derive(Debug)]
pub struct FastMouse {
    __cordl_parent: crate::UnityEngine::InputSystem::Mouse,
}
#[cfg(feature = "UnityEngine+InputSystem+FastMouse")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::InputSystem::FastMouse {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "FastMouse";
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
#[cfg(feature = "UnityEngine+InputSystem+FastMouse")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::FastMouse {
    type Target = crate::UnityEngine::InputSystem::Mouse;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+FastMouse")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::FastMouse {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+FastMouse")]
impl crate::UnityEngine::InputSystem::FastMouse {
    pub const metadata: &'static str = "AutoWindowSpace;Vector2;Delta;Button;Axis;Digital;Integer;Mouse;Pointer";
    pub fn Initialize_ctrlMousebackButton(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
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
                    >("Initialize_ctrlMousebackButton")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Initialize_ctrlMousebackButton", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (kButtonLayout, parent))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlMouseclickCount(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::IntegerControl,
                        >,
                        2usize,
                    >("Initialize_ctrlMouseclickCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Initialize_ctrlMouseclickCount", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (kIntegerLayout, parent))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlMousedelta(
        &mut self,
        kDeltaLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::DeltaControl,
                        >,
                        2usize,
                    >("Initialize_ctrlMousedelta")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Initialize_ctrlMousedelta", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (kDeltaLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlMousedeltadown(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::AxisControl,
                        >,
                        2usize,
                    >("Initialize_ctrlMousedeltadown")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Initialize_ctrlMousedeltadown", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlMousedeltaleft(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::AxisControl,
                        >,
                        2usize,
                    >("Initialize_ctrlMousedeltaleft")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Initialize_ctrlMousedeltaleft", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlMousedeltaright(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::AxisControl,
                        >,
                        2usize,
                    >("Initialize_ctrlMousedeltaright")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Initialize_ctrlMousedeltaright", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlMousedeltaup(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::AxisControl,
                        >,
                        2usize,
                    >("Initialize_ctrlMousedeltaup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Initialize_ctrlMousedeltaup", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlMousedeltax(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::AxisControl,
                        >,
                        2usize,
                    >("Initialize_ctrlMousedeltax")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Initialize_ctrlMousedeltax", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlMousedeltay(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::AxisControl,
                        >,
                        2usize,
                    >("Initialize_ctrlMousedeltay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Initialize_ctrlMousedeltay", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlMousedisplayIndex(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::IntegerControl,
                        >,
                        2usize,
                    >("Initialize_ctrlMousedisplayIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Initialize_ctrlMousedisplayIndex", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (kIntegerLayout, parent))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlMouseforwardButton(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
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
                    >("Initialize_ctrlMouseforwardButton")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Initialize_ctrlMouseforwardButton", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (kButtonLayout, parent))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlMouseleftButton(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
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
                    >("Initialize_ctrlMouseleftButton")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Initialize_ctrlMouseleftButton", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (kButtonLayout, parent))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlMousemiddleButton(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
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
                    >("Initialize_ctrlMousemiddleButton")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Initialize_ctrlMousemiddleButton", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (kButtonLayout, parent))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlMousepointerId(
        &mut self,
        kDigitalLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::IntegerControl,
                        >,
                        2usize,
                    >("Initialize_ctrlMousepointerId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Initialize_ctrlMousepointerId", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (kDigitalLayout, parent))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlMouseposition(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::Vector2Control,
                        >,
                        2usize,
                    >("Initialize_ctrlMouseposition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Initialize_ctrlMouseposition", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (kVector2Layout, parent))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlMousepositionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::AxisControl,
                        >,
                        2usize,
                    >("Initialize_ctrlMousepositionx")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Initialize_ctrlMousepositionx", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlMousepositiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::AxisControl,
                        >,
                        2usize,
                    >("Initialize_ctrlMousepositiony")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Initialize_ctrlMousepositiony", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlMousepress(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
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
                    >("Initialize_ctrlMousepress")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Initialize_ctrlMousepress", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (kButtonLayout, parent))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlMousepressure(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::AxisControl,
                        >,
                        2usize,
                    >("Initialize_ctrlMousepressure")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Initialize_ctrlMousepressure", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlMouseradius(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::Vector2Control,
                        >,
                        2usize,
                    >("Initialize_ctrlMouseradius")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Initialize_ctrlMouseradius", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (kVector2Layout, parent))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlMouseradiusx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::AxisControl,
                        >,
                        2usize,
                    >("Initialize_ctrlMouseradiusx")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Initialize_ctrlMouseradiusx", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlMouseradiusy(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::AxisControl,
                        >,
                        2usize,
                    >("Initialize_ctrlMouseradiusy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Initialize_ctrlMouseradiusy", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlMouserightButton(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
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
                    >("Initialize_ctrlMouserightButton")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Initialize_ctrlMouserightButton", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (kButtonLayout, parent))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlMousescroll(
        &mut self,
        kDeltaLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::DeltaControl,
                        >,
                        2usize,
                    >("Initialize_ctrlMousescroll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Initialize_ctrlMousescroll", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (kDeltaLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlMousescrolldown(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::AxisControl,
                        >,
                        2usize,
                    >("Initialize_ctrlMousescrolldown")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Initialize_ctrlMousescrolldown", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlMousescrollleft(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::AxisControl,
                        >,
                        2usize,
                    >("Initialize_ctrlMousescrollleft")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Initialize_ctrlMousescrollleft", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlMousescrollright(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::AxisControl,
                        >,
                        2usize,
                    >("Initialize_ctrlMousescrollright")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Initialize_ctrlMousescrollright", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlMousescrollup(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::AxisControl,
                        >,
                        2usize,
                    >("Initialize_ctrlMousescrollup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Initialize_ctrlMousescrollup", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlMousescrollx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::AxisControl,
                        >,
                        2usize,
                    >("Initialize_ctrlMousescrollx")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Initialize_ctrlMousescrollx", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlMousescrolly(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::AxisControl,
                        >,
                        2usize,
                    >("Initialize_ctrlMousescrolly")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Initialize_ctrlMousescrolly", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn MergeForward(
        currentEventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        nextEventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                        ),
                        bool,
                        2usize,
                    >("MergeForward")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MergeForward", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (currentEventPtr, nextEventPtr))?
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
    pub fn OnNextUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("OnNextUpdate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnNextUpdate", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnStateEvent(
        &mut self,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::InputSystem::LowLevel::InputEventPtr),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OnStateEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnStateEvent", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (eventPtr))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_InputSystem_LowLevel_IEventMerger_MergeForward(
        &mut self,
        currentEventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        nextEventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                        ),
                        bool,
                        2usize,
                    >("UnityEngine.InputSystem.LowLevel.IEventMerger.MergeForward")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.InputSystem.LowLevel.IEventMerger.MergeForward",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (currentEventPtr, nextEventPtr))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_InputSystem_LowLevel_IInputStateCallbackReceiver_OnNextUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >(
                        "UnityEngine.InputSystem.LowLevel.IInputStateCallbackReceiver.OnNextUpdate",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.InputSystem.LowLevel.IInputStateCallbackReceiver.OnNextUpdate",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_InputSystem_LowLevel_IInputStateCallbackReceiver_OnStateEvent(
        &mut self,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::InputSystem::LowLevel::InputEventPtr),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(
                        "UnityEngine.InputSystem.LowLevel.IInputStateCallbackReceiver.OnStateEvent",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.InputSystem.LowLevel.IInputStateCallbackReceiver.OnStateEvent",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (eventPtr))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
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
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+FastMouse")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::InputSystem::FastMouse {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+FastMouse")]
impl AsRef<crate::UnityEngine::InputSystem::LowLevel::IEventMerger>
for crate::UnityEngine::InputSystem::FastMouse {
    fn as_ref(&self) -> &crate::UnityEngine::InputSystem::LowLevel::IEventMerger {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+FastMouse")]
impl AsMut<crate::UnityEngine::InputSystem::LowLevel::IEventMerger>
for crate::UnityEngine::InputSystem::FastMouse {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::InputSystem::LowLevel::IEventMerger {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+FastMouse")]
impl AsRef<crate::UnityEngine::InputSystem::LowLevel::IInputStateCallbackReceiver>
for crate::UnityEngine::InputSystem::FastMouse {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::InputSystem::LowLevel::IInputStateCallbackReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+FastMouse")]
impl AsMut<crate::UnityEngine::InputSystem::LowLevel::IInputStateCallbackReceiver>
for crate::UnityEngine::InputSystem::FastMouse {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::InputSystem::LowLevel::IInputStateCallbackReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
