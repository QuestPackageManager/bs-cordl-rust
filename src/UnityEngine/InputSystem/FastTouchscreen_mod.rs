#[cfg(feature = "UnityEngine+InputSystem+FastTouchscreen")]
#[repr(C)]
#[derive(Debug)]
pub struct FastTouchscreen {
    __cordl_parent: crate::UnityEngine::InputSystem::Touchscreen,
}
#[cfg(feature = "UnityEngine+InputSystem+FastTouchscreen")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::FastTouchscreen {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "FastTouchscreen";
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
#[cfg(feature = "UnityEngine+InputSystem+FastTouchscreen")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::FastTouchscreen {
    type Target = crate::UnityEngine::InputSystem::Touchscreen;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+FastTouchscreen")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::FastTouchscreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+FastTouchscreen")]
impl crate::UnityEngine::InputSystem::FastTouchscreen {
    pub const metadata: &'static str = "AutoWindowSpace;Touch;Vector2;Delta;Analog;TouchPress;Button;Axis;Integer;TouchPhase;Double;Touchscreen;Pointer";
    pub fn Initialize_ctrlTouchscreendelta(
        &mut self,
        kDeltaLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreendelta")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreendelta", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        > = unsafe { method.invoke_unchecked(self, (kDeltaLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreendeltadown(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreendeltadown")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreendeltadown", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreendeltaleft(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreendeltaleft")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreendeltaleft", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreendeltaright(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreendeltaright")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreendeltaright", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreendeltaup(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreendeltaup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreendeltaup", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreendeltax(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreendeltax")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreendeltax", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreendeltay(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreendeltay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreendeltay", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreendisplayIndex(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreendisplayIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreendisplayIndex",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = unsafe { method.invoke_unchecked(self, (kIntegerLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenposition(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreenposition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreenposition", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = unsafe { method.invoke_unchecked(self, (kVector2Layout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenpositionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreenpositionx")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreenpositionx", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenpositiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreenpositiony")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreenpositiony", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenpress(
        &mut self,
        kTouchPressLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreenpress")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreenpress", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        > = unsafe { method.invoke_unchecked(self, (kTouchPressLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenpressure(
        &mut self,
        kAnalogLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreenpressure")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreenpressure", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAnalogLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouch(
        &mut self,
        kTouchLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::TouchControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreenprimaryTouch")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreenprimaryTouch",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        > = unsafe { method.invoke_unchecked(self, (kTouchLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchdelta(
        &mut self,
        kDeltaLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreenprimaryTouchdelta")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreenprimaryTouchdelta",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        > = unsafe { method.invoke_unchecked(self, (kDeltaLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchdeltadown(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreenprimaryTouchdeltadown")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreenprimaryTouchdeltadown", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchdeltaleft(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreenprimaryTouchdeltaleft")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreenprimaryTouchdeltaleft", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchdeltaright(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreenprimaryTouchdeltaright")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreenprimaryTouchdeltaright", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchdeltaup(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreenprimaryTouchdeltaup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreenprimaryTouchdeltaup", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchdeltax(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreenprimaryTouchdeltax")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreenprimaryTouchdeltax", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchdeltay(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreenprimaryTouchdeltay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreenprimaryTouchdeltay", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchdisplayIndex(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreenprimaryTouchdisplayIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreenprimaryTouchdisplayIndex", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = unsafe { method.invoke_unchecked(self, (kIntegerLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchindirectTouch(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreenprimaryTouchindirectTouch")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreenprimaryTouchindirectTouch", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = unsafe { method.invoke_unchecked(self, (kButtonLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchphase(
        &mut self,
        kTouchPhaseLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreenprimaryTouchphase")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreenprimaryTouchphase",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        > = unsafe { method.invoke_unchecked(self, (kTouchPhaseLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchposition(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreenprimaryTouchposition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreenprimaryTouchposition", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = unsafe { method.invoke_unchecked(self, (kVector2Layout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchpositionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreenprimaryTouchpositionx")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreenprimaryTouchpositionx", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchpositiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreenprimaryTouchpositiony")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreenprimaryTouchpositiony", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchpress(
        &mut self,
        kTouchPressLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreenprimaryTouchpress")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreenprimaryTouchpress",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        > = unsafe { method.invoke_unchecked(self, (kTouchPressLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchpressure(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreenprimaryTouchpressure")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreenprimaryTouchpressure", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchradius(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreenprimaryTouchradius")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreenprimaryTouchradius", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = unsafe { method.invoke_unchecked(self, (kVector2Layout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchradiusx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreenprimaryTouchradiusx")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreenprimaryTouchradiusx", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchradiusy(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreenprimaryTouchradiusy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreenprimaryTouchradiusy", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchstartPosition(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreenprimaryTouchstartPosition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreenprimaryTouchstartPosition", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = unsafe { method.invoke_unchecked(self, (kVector2Layout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchstartPositionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreenprimaryTouchstartPositionx")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreenprimaryTouchstartPositionx",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchstartPositiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreenprimaryTouchstartPositiony")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreenprimaryTouchstartPositiony",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchstartTime(
        &mut self,
        kDoubleLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::DoubleControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreenprimaryTouchstartTime")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreenprimaryTouchstartTime", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        > = unsafe { method.invoke_unchecked(self, (kDoubleLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchtap(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreenprimaryTouchtap")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreenprimaryTouchtap",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = unsafe { method.invoke_unchecked(self, (kButtonLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchtapCount(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreenprimaryTouchtapCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreenprimaryTouchtapCount", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = unsafe { method.invoke_unchecked(self, (kIntegerLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenprimaryTouchtouchId(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreenprimaryTouchtouchId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreenprimaryTouchtouchId", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = unsafe { method.invoke_unchecked(self, (kIntegerLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenradius(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreenradius")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreenradius", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = unsafe { method.invoke_unchecked(self, (kVector2Layout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenradiusx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreenradiusx")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreenradiusx", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreenradiusy(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreenradiusy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreenradiusy", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0(
        &mut self,
        kTouchLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::TouchControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreentouch0")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch0", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        > = unsafe { method.invoke_unchecked(self, (kTouchLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0delta(
        &mut self,
        kDeltaLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch0delta")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch0delta",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        > = unsafe { method.invoke_unchecked(self, (kDeltaLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0deltadown(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch0deltadown")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch0deltadown",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0deltaleft(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch0deltaleft")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch0deltaleft",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0deltaright(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch0deltaright")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch0deltaright",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0deltaup(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch0deltaup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch0deltaup",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0deltax(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch0deltax")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch0deltax",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0deltay(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch0deltay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch0deltay",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0displayIndex(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch0displayIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch0displayIndex", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = unsafe { method.invoke_unchecked(self, (kIntegerLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0indirectTouch(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch0indirectTouch")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch0indirectTouch", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = unsafe { method.invoke_unchecked(self, (kButtonLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0phase(
        &mut self,
        kTouchPhaseLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreentouch0phase")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch0phase",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        > = unsafe { method.invoke_unchecked(self, (kTouchPhaseLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0position(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch0position")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch0position",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = unsafe { method.invoke_unchecked(self, (kVector2Layout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0positionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch0positionx")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch0positionx",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0positiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch0positiony")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch0positiony",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0press(
        &mut self,
        kTouchPressLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreentouch0press")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch0press",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        > = unsafe { method.invoke_unchecked(self, (kTouchPressLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0pressure(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch0pressure")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch0pressure",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0radius(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch0radius")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch0radius",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = unsafe { method.invoke_unchecked(self, (kVector2Layout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0radiusx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch0radiusx")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch0radiusx",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0radiusy(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch0radiusy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch0radiusy",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0startPosition(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch0startPosition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch0startPosition", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = unsafe { method.invoke_unchecked(self, (kVector2Layout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0startPositionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch0startPositionx")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch0startPositionx", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0startPositiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch0startPositiony")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch0startPositiony", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0startTime(
        &mut self,
        kDoubleLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::DoubleControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreentouch0startTime")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch0startTime",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        > = unsafe { method.invoke_unchecked(self, (kDoubleLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0tap(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch0tap")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch0tap", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = unsafe { method.invoke_unchecked(self, (kButtonLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0tapCount(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch0tapCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch0tapCount",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = unsafe { method.invoke_unchecked(self, (kIntegerLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch0touchId(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch0touchId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch0touchId",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = unsafe { method.invoke_unchecked(self, (kIntegerLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1(
        &mut self,
        kTouchLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::TouchControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreentouch1")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch1", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        > = unsafe { method.invoke_unchecked(self, (kTouchLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1delta(
        &mut self,
        kDeltaLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch1delta")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch1delta",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        > = unsafe { method.invoke_unchecked(self, (kDeltaLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1deltadown(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch1deltadown")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch1deltadown",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1deltaleft(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch1deltaleft")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch1deltaleft",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1deltaright(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch1deltaright")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch1deltaright",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1deltaup(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch1deltaup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch1deltaup",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1deltax(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch1deltax")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch1deltax",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1deltay(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch1deltay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch1deltay",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1displayIndex(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch1displayIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch1displayIndex", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = unsafe { method.invoke_unchecked(self, (kIntegerLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1indirectTouch(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch1indirectTouch")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch1indirectTouch", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = unsafe { method.invoke_unchecked(self, (kButtonLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1phase(
        &mut self,
        kTouchPhaseLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreentouch1phase")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch1phase",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        > = unsafe { method.invoke_unchecked(self, (kTouchPhaseLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1position(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch1position")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch1position",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = unsafe { method.invoke_unchecked(self, (kVector2Layout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1positionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch1positionx")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch1positionx",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1positiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch1positiony")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch1positiony",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1press(
        &mut self,
        kTouchPressLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreentouch1press")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch1press",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        > = unsafe { method.invoke_unchecked(self, (kTouchPressLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1pressure(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch1pressure")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch1pressure",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1radius(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch1radius")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch1radius",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = unsafe { method.invoke_unchecked(self, (kVector2Layout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1radiusx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch1radiusx")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch1radiusx",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1radiusy(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch1radiusy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch1radiusy",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1startPosition(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch1startPosition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch1startPosition", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = unsafe { method.invoke_unchecked(self, (kVector2Layout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1startPositionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch1startPositionx")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch1startPositionx", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1startPositiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch1startPositiony")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch1startPositiony", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1startTime(
        &mut self,
        kDoubleLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::DoubleControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreentouch1startTime")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch1startTime",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        > = unsafe { method.invoke_unchecked(self, (kDoubleLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1tap(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch1tap")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch1tap", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = unsafe { method.invoke_unchecked(self, (kButtonLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1tapCount(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch1tapCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch1tapCount",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = unsafe { method.invoke_unchecked(self, (kIntegerLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch1touchId(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch1touchId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch1touchId",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = unsafe { method.invoke_unchecked(self, (kIntegerLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2(
        &mut self,
        kTouchLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::TouchControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreentouch2")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch2", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        > = unsafe { method.invoke_unchecked(self, (kTouchLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2delta(
        &mut self,
        kDeltaLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch2delta")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch2delta",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        > = unsafe { method.invoke_unchecked(self, (kDeltaLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2deltadown(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch2deltadown")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch2deltadown",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2deltaleft(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch2deltaleft")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch2deltaleft",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2deltaright(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch2deltaright")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch2deltaright",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2deltaup(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch2deltaup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch2deltaup",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2deltax(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch2deltax")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch2deltax",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2deltay(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch2deltay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch2deltay",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2displayIndex(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch2displayIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch2displayIndex", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = unsafe { method.invoke_unchecked(self, (kIntegerLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2indirectTouch(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch2indirectTouch")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch2indirectTouch", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = unsafe { method.invoke_unchecked(self, (kButtonLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2phase(
        &mut self,
        kTouchPhaseLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreentouch2phase")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch2phase",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        > = unsafe { method.invoke_unchecked(self, (kTouchPhaseLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2position(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch2position")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch2position",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = unsafe { method.invoke_unchecked(self, (kVector2Layout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2positionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch2positionx")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch2positionx",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2positiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch2positiony")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch2positiony",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2press(
        &mut self,
        kTouchPressLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreentouch2press")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch2press",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        > = unsafe { method.invoke_unchecked(self, (kTouchPressLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2pressure(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch2pressure")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch2pressure",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2radius(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch2radius")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch2radius",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = unsafe { method.invoke_unchecked(self, (kVector2Layout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2radiusx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch2radiusx")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch2radiusx",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2radiusy(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch2radiusy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch2radiusy",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2startPosition(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch2startPosition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch2startPosition", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = unsafe { method.invoke_unchecked(self, (kVector2Layout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2startPositionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch2startPositionx")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch2startPositionx", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2startPositiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch2startPositiony")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch2startPositiony", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2startTime(
        &mut self,
        kDoubleLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::DoubleControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreentouch2startTime")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch2startTime",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        > = unsafe { method.invoke_unchecked(self, (kDoubleLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2tap(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch2tap")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch2tap", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = unsafe { method.invoke_unchecked(self, (kButtonLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2tapCount(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch2tapCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch2tapCount",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = unsafe { method.invoke_unchecked(self, (kIntegerLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch2touchId(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch2touchId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch2touchId",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = unsafe { method.invoke_unchecked(self, (kIntegerLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3(
        &mut self,
        kTouchLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::TouchControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreentouch3")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch3", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        > = unsafe { method.invoke_unchecked(self, (kTouchLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3delta(
        &mut self,
        kDeltaLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch3delta")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch3delta",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        > = unsafe { method.invoke_unchecked(self, (kDeltaLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3deltadown(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch3deltadown")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch3deltadown",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3deltaleft(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch3deltaleft")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch3deltaleft",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3deltaright(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch3deltaright")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch3deltaright",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3deltaup(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch3deltaup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch3deltaup",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3deltax(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch3deltax")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch3deltax",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3deltay(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch3deltay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch3deltay",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3displayIndex(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch3displayIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch3displayIndex", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = unsafe { method.invoke_unchecked(self, (kIntegerLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3indirectTouch(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch3indirectTouch")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch3indirectTouch", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = unsafe { method.invoke_unchecked(self, (kButtonLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3phase(
        &mut self,
        kTouchPhaseLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreentouch3phase")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch3phase",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        > = unsafe { method.invoke_unchecked(self, (kTouchPhaseLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3position(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch3position")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch3position",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = unsafe { method.invoke_unchecked(self, (kVector2Layout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3positionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch3positionx")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch3positionx",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3positiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch3positiony")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch3positiony",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3press(
        &mut self,
        kTouchPressLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreentouch3press")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch3press",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        > = unsafe { method.invoke_unchecked(self, (kTouchPressLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3pressure(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch3pressure")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch3pressure",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3radius(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch3radius")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch3radius",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = unsafe { method.invoke_unchecked(self, (kVector2Layout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3radiusx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch3radiusx")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch3radiusx",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3radiusy(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch3radiusy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch3radiusy",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3startPosition(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch3startPosition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch3startPosition", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = unsafe { method.invoke_unchecked(self, (kVector2Layout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3startPositionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch3startPositionx")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch3startPositionx", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3startPositiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch3startPositiony")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch3startPositiony", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3startTime(
        &mut self,
        kDoubleLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::DoubleControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreentouch3startTime")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch3startTime",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        > = unsafe { method.invoke_unchecked(self, (kDoubleLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3tap(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch3tap")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch3tap", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = unsafe { method.invoke_unchecked(self, (kButtonLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3tapCount(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch3tapCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch3tapCount",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = unsafe { method.invoke_unchecked(self, (kIntegerLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch3touchId(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch3touchId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch3touchId",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = unsafe { method.invoke_unchecked(self, (kIntegerLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4(
        &mut self,
        kTouchLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::TouchControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreentouch4")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch4", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        > = unsafe { method.invoke_unchecked(self, (kTouchLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4delta(
        &mut self,
        kDeltaLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch4delta")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch4delta",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        > = unsafe { method.invoke_unchecked(self, (kDeltaLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4deltadown(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch4deltadown")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch4deltadown",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4deltaleft(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch4deltaleft")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch4deltaleft",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4deltaright(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch4deltaright")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch4deltaright",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4deltaup(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch4deltaup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch4deltaup",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4deltax(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch4deltax")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch4deltax",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4deltay(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch4deltay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch4deltay",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4displayIndex(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch4displayIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch4displayIndex", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = unsafe { method.invoke_unchecked(self, (kIntegerLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4indirectTouch(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch4indirectTouch")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch4indirectTouch", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = unsafe { method.invoke_unchecked(self, (kButtonLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4phase(
        &mut self,
        kTouchPhaseLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreentouch4phase")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch4phase",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        > = unsafe { method.invoke_unchecked(self, (kTouchPhaseLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4position(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch4position")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch4position",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = unsafe { method.invoke_unchecked(self, (kVector2Layout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4positionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch4positionx")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch4positionx",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4positiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch4positiony")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch4positiony",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4press(
        &mut self,
        kTouchPressLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreentouch4press")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch4press",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        > = unsafe { method.invoke_unchecked(self, (kTouchPressLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4pressure(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch4pressure")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch4pressure",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4radius(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch4radius")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch4radius",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = unsafe { method.invoke_unchecked(self, (kVector2Layout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4radiusx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch4radiusx")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch4radiusx",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4radiusy(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch4radiusy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch4radiusy",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4startPosition(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch4startPosition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch4startPosition", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = unsafe { method.invoke_unchecked(self, (kVector2Layout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4startPositionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch4startPositionx")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch4startPositionx", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4startPositiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch4startPositiony")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch4startPositiony", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4startTime(
        &mut self,
        kDoubleLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::DoubleControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreentouch4startTime")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch4startTime",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        > = unsafe { method.invoke_unchecked(self, (kDoubleLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4tap(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch4tap")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch4tap", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = unsafe { method.invoke_unchecked(self, (kButtonLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4tapCount(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch4tapCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch4tapCount",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = unsafe { method.invoke_unchecked(self, (kIntegerLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch4touchId(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch4touchId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch4touchId",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = unsafe { method.invoke_unchecked(self, (kIntegerLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5(
        &mut self,
        kTouchLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::TouchControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreentouch5")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch5", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        > = unsafe { method.invoke_unchecked(self, (kTouchLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5delta(
        &mut self,
        kDeltaLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch5delta")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch5delta",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        > = unsafe { method.invoke_unchecked(self, (kDeltaLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5deltadown(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch5deltadown")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch5deltadown",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5deltaleft(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch5deltaleft")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch5deltaleft",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5deltaright(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch5deltaright")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch5deltaright",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5deltaup(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch5deltaup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch5deltaup",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5deltax(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch5deltax")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch5deltax",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5deltay(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch5deltay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch5deltay",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5displayIndex(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch5displayIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch5displayIndex", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = unsafe { method.invoke_unchecked(self, (kIntegerLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5indirectTouch(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch5indirectTouch")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch5indirectTouch", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = unsafe { method.invoke_unchecked(self, (kButtonLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5phase(
        &mut self,
        kTouchPhaseLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreentouch5phase")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch5phase",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        > = unsafe { method.invoke_unchecked(self, (kTouchPhaseLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5position(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch5position")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch5position",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = unsafe { method.invoke_unchecked(self, (kVector2Layout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5positionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch5positionx")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch5positionx",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5positiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch5positiony")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch5positiony",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5press(
        &mut self,
        kTouchPressLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreentouch5press")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch5press",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        > = unsafe { method.invoke_unchecked(self, (kTouchPressLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5pressure(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch5pressure")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch5pressure",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5radius(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch5radius")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch5radius",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = unsafe { method.invoke_unchecked(self, (kVector2Layout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5radiusx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch5radiusx")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch5radiusx",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5radiusy(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch5radiusy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch5radiusy",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5startPosition(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch5startPosition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch5startPosition", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = unsafe { method.invoke_unchecked(self, (kVector2Layout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5startPositionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch5startPositionx")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch5startPositionx", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5startPositiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch5startPositiony")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch5startPositiony", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5startTime(
        &mut self,
        kDoubleLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::DoubleControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreentouch5startTime")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch5startTime",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        > = unsafe { method.invoke_unchecked(self, (kDoubleLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5tap(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch5tap")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch5tap", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = unsafe { method.invoke_unchecked(self, (kButtonLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5tapCount(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch5tapCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch5tapCount",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = unsafe { method.invoke_unchecked(self, (kIntegerLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch5touchId(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch5touchId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch5touchId",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = unsafe { method.invoke_unchecked(self, (kIntegerLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6(
        &mut self,
        kTouchLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::TouchControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreentouch6")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch6", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        > = unsafe { method.invoke_unchecked(self, (kTouchLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6delta(
        &mut self,
        kDeltaLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch6delta")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch6delta",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        > = unsafe { method.invoke_unchecked(self, (kDeltaLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6deltadown(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch6deltadown")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch6deltadown",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6deltaleft(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch6deltaleft")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch6deltaleft",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6deltaright(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch6deltaright")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch6deltaright",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6deltaup(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch6deltaup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch6deltaup",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6deltax(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch6deltax")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch6deltax",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6deltay(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch6deltay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch6deltay",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6displayIndex(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch6displayIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch6displayIndex", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = unsafe { method.invoke_unchecked(self, (kIntegerLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6indirectTouch(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch6indirectTouch")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch6indirectTouch", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = unsafe { method.invoke_unchecked(self, (kButtonLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6phase(
        &mut self,
        kTouchPhaseLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreentouch6phase")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch6phase",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        > = unsafe { method.invoke_unchecked(self, (kTouchPhaseLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6position(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch6position")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch6position",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = unsafe { method.invoke_unchecked(self, (kVector2Layout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6positionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch6positionx")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch6positionx",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6positiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch6positiony")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch6positiony",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6press(
        &mut self,
        kTouchPressLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreentouch6press")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch6press",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        > = unsafe { method.invoke_unchecked(self, (kTouchPressLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6pressure(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch6pressure")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch6pressure",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6radius(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch6radius")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch6radius",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = unsafe { method.invoke_unchecked(self, (kVector2Layout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6radiusx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch6radiusx")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch6radiusx",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6radiusy(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch6radiusy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch6radiusy",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6startPosition(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch6startPosition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch6startPosition", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = unsafe { method.invoke_unchecked(self, (kVector2Layout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6startPositionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch6startPositionx")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch6startPositionx", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6startPositiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch6startPositiony")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch6startPositiony", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6startTime(
        &mut self,
        kDoubleLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::DoubleControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreentouch6startTime")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch6startTime",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        > = unsafe { method.invoke_unchecked(self, (kDoubleLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6tap(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch6tap")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch6tap", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = unsafe { method.invoke_unchecked(self, (kButtonLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6tapCount(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch6tapCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch6tapCount",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = unsafe { method.invoke_unchecked(self, (kIntegerLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch6touchId(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch6touchId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch6touchId",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = unsafe { method.invoke_unchecked(self, (kIntegerLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7(
        &mut self,
        kTouchLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::TouchControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreentouch7")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch7", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        > = unsafe { method.invoke_unchecked(self, (kTouchLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7delta(
        &mut self,
        kDeltaLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch7delta")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch7delta",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        > = unsafe { method.invoke_unchecked(self, (kDeltaLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7deltadown(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch7deltadown")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch7deltadown",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7deltaleft(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch7deltaleft")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch7deltaleft",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7deltaright(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch7deltaright")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch7deltaright",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7deltaup(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch7deltaup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch7deltaup",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7deltax(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch7deltax")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch7deltax",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7deltay(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch7deltay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch7deltay",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7displayIndex(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch7displayIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch7displayIndex", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = unsafe { method.invoke_unchecked(self, (kIntegerLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7indirectTouch(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch7indirectTouch")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch7indirectTouch", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = unsafe { method.invoke_unchecked(self, (kButtonLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7phase(
        &mut self,
        kTouchPhaseLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreentouch7phase")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch7phase",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        > = unsafe { method.invoke_unchecked(self, (kTouchPhaseLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7position(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch7position")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch7position",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = unsafe { method.invoke_unchecked(self, (kVector2Layout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7positionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch7positionx")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch7positionx",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7positiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch7positiony")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch7positiony",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7press(
        &mut self,
        kTouchPressLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreentouch7press")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch7press",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        > = unsafe { method.invoke_unchecked(self, (kTouchPressLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7pressure(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch7pressure")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch7pressure",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7radius(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch7radius")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch7radius",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = unsafe { method.invoke_unchecked(self, (kVector2Layout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7radiusx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch7radiusx")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch7radiusx",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7radiusy(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch7radiusy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch7radiusy",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7startPosition(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch7startPosition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch7startPosition", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = unsafe { method.invoke_unchecked(self, (kVector2Layout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7startPositionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch7startPositionx")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch7startPositionx", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7startPositiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch7startPositiony")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch7startPositiony", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7startTime(
        &mut self,
        kDoubleLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::DoubleControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreentouch7startTime")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch7startTime",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        > = unsafe { method.invoke_unchecked(self, (kDoubleLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7tap(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch7tap")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch7tap", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = unsafe { method.invoke_unchecked(self, (kButtonLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7tapCount(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch7tapCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch7tapCount",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = unsafe { method.invoke_unchecked(self, (kIntegerLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch7touchId(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch7touchId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch7touchId",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = unsafe { method.invoke_unchecked(self, (kIntegerLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8(
        &mut self,
        kTouchLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::TouchControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreentouch8")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch8", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        > = unsafe { method.invoke_unchecked(self, (kTouchLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8delta(
        &mut self,
        kDeltaLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch8delta")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch8delta",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        > = unsafe { method.invoke_unchecked(self, (kDeltaLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8deltadown(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch8deltadown")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch8deltadown",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8deltaleft(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch8deltaleft")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch8deltaleft",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8deltaright(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch8deltaright")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch8deltaright",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8deltaup(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch8deltaup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch8deltaup",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8deltax(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch8deltax")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch8deltax",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8deltay(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch8deltay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch8deltay",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8displayIndex(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch8displayIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch8displayIndex", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = unsafe { method.invoke_unchecked(self, (kIntegerLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8indirectTouch(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch8indirectTouch")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch8indirectTouch", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = unsafe { method.invoke_unchecked(self, (kButtonLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8phase(
        &mut self,
        kTouchPhaseLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreentouch8phase")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch8phase",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        > = unsafe { method.invoke_unchecked(self, (kTouchPhaseLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8position(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch8position")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch8position",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = unsafe { method.invoke_unchecked(self, (kVector2Layout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8positionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch8positionx")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch8positionx",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8positiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch8positiony")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch8positiony",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8press(
        &mut self,
        kTouchPressLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreentouch8press")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch8press",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        > = unsafe { method.invoke_unchecked(self, (kTouchPressLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8pressure(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch8pressure")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch8pressure",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8radius(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch8radius")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch8radius",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = unsafe { method.invoke_unchecked(self, (kVector2Layout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8radiusx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch8radiusx")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch8radiusx",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8radiusy(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch8radiusy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch8radiusy",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8startPosition(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch8startPosition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch8startPosition", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = unsafe { method.invoke_unchecked(self, (kVector2Layout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8startPositionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch8startPositionx")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch8startPositionx", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8startPositiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch8startPositiony")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch8startPositiony", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8startTime(
        &mut self,
        kDoubleLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::DoubleControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreentouch8startTime")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch8startTime",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        > = unsafe { method.invoke_unchecked(self, (kDoubleLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8tap(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch8tap")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch8tap", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = unsafe { method.invoke_unchecked(self, (kButtonLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8tapCount(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch8tapCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch8tapCount",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = unsafe { method.invoke_unchecked(self, (kIntegerLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch8touchId(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch8touchId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch8touchId",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = unsafe { method.invoke_unchecked(self, (kIntegerLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9(
        &mut self,
        kTouchLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::TouchControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreentouch9")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch9", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        > = unsafe { method.invoke_unchecked(self, (kTouchLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9delta(
        &mut self,
        kDeltaLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch9delta")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch9delta",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DeltaControl,
        > = unsafe { method.invoke_unchecked(self, (kDeltaLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9deltadown(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch9deltadown")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch9deltadown",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9deltaleft(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch9deltaleft")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch9deltaleft",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9deltaright(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch9deltaright")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch9deltaright",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9deltaup(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch9deltaup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch9deltaup",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9deltax(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch9deltax")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch9deltax",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9deltay(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch9deltay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch9deltay",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9displayIndex(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch9displayIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch9displayIndex", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = unsafe { method.invoke_unchecked(self, (kIntegerLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9indirectTouch(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch9indirectTouch")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch9indirectTouch", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = unsafe { method.invoke_unchecked(self, (kButtonLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9phase(
        &mut self,
        kTouchPhaseLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreentouch9phase")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch9phase",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPhaseControl,
        > = unsafe { method.invoke_unchecked(self, (kTouchPhaseLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9position(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch9position")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch9position",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = unsafe { method.invoke_unchecked(self, (kVector2Layout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9positionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch9positionx")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch9positionx",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9positiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch9positiony")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch9positiony",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9press(
        &mut self,
        kTouchPressLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreentouch9press")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch9press",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchPressControl,
        > = unsafe { method.invoke_unchecked(self, (kTouchPressLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9pressure(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch9pressure")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch9pressure",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9radius(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch9radius")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch9radius",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = unsafe { method.invoke_unchecked(self, (kVector2Layout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9radiusx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch9radiusx")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch9radiusx",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9radiusy(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch9radiusy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch9radiusy",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9startPosition(
        &mut self,
        kVector2Layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch9startPosition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch9startPosition", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector2Control,
        > = unsafe { method.invoke_unchecked(self, (kVector2Layout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9startPositionx(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch9startPositionx")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch9startPositionx", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9startPositiony(
        &mut self,
        kAxisLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch9startPositiony")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "Initialize_ctrlTouchscreentouch9startPositiony", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = unsafe { method.invoke_unchecked(self, (kAxisLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9startTime(
        &mut self,
        kDoubleLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::DoubleControl,
                        >,
                        2usize,
                    >("Initialize_ctrlTouchscreentouch9startTime")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch9startTime",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::DoubleControl,
        > = unsafe { method.invoke_unchecked(self, (kDoubleLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9tap(
        &mut self,
        kButtonLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch9tap")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch9tap", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = unsafe { method.invoke_unchecked(self, (kButtonLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9tapCount(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch9tapCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch9tapCount",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = unsafe { method.invoke_unchecked(self, (kIntegerLayout, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ctrlTouchscreentouch9touchId(
        &mut self,
        kIntegerLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                    >("Initialize_ctrlTouchscreentouch9touchId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Initialize_ctrlTouchscreentouch9touchId",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = unsafe { method.invoke_unchecked(self, (kIntegerLayout, parent))? };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+FastTouchscreen")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::FastTouchscreen {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
