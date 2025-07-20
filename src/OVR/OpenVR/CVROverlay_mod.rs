#[cfg(feature = "OVR+OpenVR+CVROverlay")]
#[repr(C)]
#[derive(Debug)]
pub struct CVROverlay {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub FnTable: crate::OVR::OpenVR::IVROverlay,
}
#[cfg(feature = "OVR+OpenVR+CVROverlay")]
unsafe impl quest_hook::libil2cpp::Type for crate::OVR::OpenVR::CVROverlay {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "CVROverlay";
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
#[cfg(feature = "OVR+OpenVR+CVROverlay")]
impl std::ops::Deref for crate::OVR::OpenVR::CVROverlay {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVROverlay")]
impl std::ops::DerefMut for crate::OVR::OpenVR::CVROverlay {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVROverlay")]
impl crate::OVR::OpenVR::CVROverlay {
    #[cfg(feature = "OVR+OpenVR+CVROverlay+PollNextOverlayEventUnion")]
    pub type PollNextOverlayEventUnion = crate::OVR::OpenVR::CVROverlay_PollNextOverlayEventUnion;
    #[cfg(feature = "OVR+OpenVR+CVROverlay+_PollNextOverlayEventPacked")]
    pub type _PollNextOverlayEventPacked = crate::OVR::OpenVR::CVROverlay__PollNextOverlayEventPacked;
    pub fn ClearOverlayTexture(
        &mut self,
        ulOverlayHandle: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u64),
                        crate::OVR::OpenVR::EVROverlayError,
                        1usize,
                    >("ClearOverlayTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ClearOverlayTexture", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CloseMessageOverlay(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("CloseMessageOverlay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CloseMessageOverlay", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeOverlayIntersection(
        &mut self,
        ulOverlayHandle: u64,
        pParams: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VROverlayIntersectionParams_t,
        >,
        pResults: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VROverlayIntersectionResults_t,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::VROverlayIntersectionParams_t,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::VROverlayIntersectionResults_t,
                            >,
                        ),
                        bool,
                        3usize,
                    >("ComputeOverlayIntersection")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ComputeOverlayIntersection", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle, pParams, pResults))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateDashboardOverlay(
        &mut self,
        pchOverlayKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchOverlayFriendlyName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        pMainHandle: quest_hook::libil2cpp::ByRefMut<u64>,
        pThumbnailHandle: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::ByRefMut<u64>,
                            quest_hook::libil2cpp::ByRefMut<u64>,
                        ),
                        crate::OVR::OpenVR::EVROverlayError,
                        4usize,
                    >("CreateDashboardOverlay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateDashboardOverlay", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        pchOverlayKey,
                        pchOverlayFriendlyName,
                        pMainHandle,
                        pThumbnailHandle,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateOverlay(
        &mut self,
        pchOverlayKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchOverlayName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pOverlayHandle: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::ByRefMut<u64>,
                        ),
                        crate::OVR::OpenVR::EVROverlayError,
                        3usize,
                    >("CreateOverlay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateOverlay", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method
                .invoke_unchecked(self, (pchOverlayKey, pchOverlayName, pOverlayHandle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DestroyOverlay(
        &mut self,
        ulOverlayHandle: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u64),
                        crate::OVR::OpenVR::EVROverlayError,
                        1usize,
                    >("DestroyOverlay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DestroyOverlay", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FindOverlay(
        &mut self,
        pchOverlayKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pOverlayHandle: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::ByRefMut<u64>,
                        ),
                        crate::OVR::OpenVR::EVROverlayError,
                        2usize,
                    >("FindOverlay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FindOverlay", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (pchOverlayKey, pOverlayHandle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetDashboardOverlaySceneProcess(
        &mut self,
        ulOverlayHandle: u64,
        punProcessId: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u64, quest_hook::libil2cpp::ByRefMut<u32>),
                        crate::OVR::OpenVR::EVROverlayError,
                        2usize,
                    >("GetDashboardOverlaySceneProcess")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetDashboardOverlaySceneProcess", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle, punProcessId))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetGamepadFocusOverlay(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), u64, 0usize>("GetGamepadFocusOverlay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetGamepadFocusOverlay", 0usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetHighQualityOverlay(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), u64, 0usize>("GetHighQualityOverlay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetHighQualityOverlay", 0usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetKeyboardText(
        &mut self,
        pchText: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        cchText: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Text::StringBuilder,
                            >,
                            u32,
                        ),
                        u32,
                        2usize,
                    >("GetKeyboardText")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetKeyboardText", 2usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked(self, (pchText, cchText))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetOverlayAlpha(
        &mut self,
        ulOverlayHandle: u64,
        pfAlpha: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u64, quest_hook::libil2cpp::ByRefMut<f32>),
                        crate::OVR::OpenVR::EVROverlayError,
                        2usize,
                    >("GetOverlayAlpha")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetOverlayAlpha", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle, pfAlpha))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetOverlayAutoCurveDistanceRangeInMeters(
        &mut self,
        ulOverlayHandle: u64,
        pfMinDistanceInMeters: quest_hook::libil2cpp::ByRefMut<f32>,
        pfMaxDistanceInMeters: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            quest_hook::libil2cpp::ByRefMut<f32>,
                            quest_hook::libil2cpp::ByRefMut<f32>,
                        ),
                        crate::OVR::OpenVR::EVROverlayError,
                        3usize,
                    >("GetOverlayAutoCurveDistanceRangeInMeters")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetOverlayAutoCurveDistanceRangeInMeters", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (ulOverlayHandle, pfMinDistanceInMeters, pfMaxDistanceInMeters),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetOverlayColor(
        &mut self,
        ulOverlayHandle: u64,
        pfRed: quest_hook::libil2cpp::ByRefMut<f32>,
        pfGreen: quest_hook::libil2cpp::ByRefMut<f32>,
        pfBlue: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            quest_hook::libil2cpp::ByRefMut<f32>,
                            quest_hook::libil2cpp::ByRefMut<f32>,
                            quest_hook::libil2cpp::ByRefMut<f32>,
                        ),
                        crate::OVR::OpenVR::EVROverlayError,
                        4usize,
                    >("GetOverlayColor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetOverlayColor", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle, pfRed, pfGreen, pfBlue))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetOverlayDualAnalogTransform(
        &mut self,
        ulOverlay: u64,
        eWhich: crate::OVR::OpenVR::EDualAnalogWhich,
        pvCenter: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdVector2_t>,
        pfRadius: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            crate::OVR::OpenVR::EDualAnalogWhich,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::HmdVector2_t,
                            >,
                            quest_hook::libil2cpp::ByRefMut<f32>,
                        ),
                        crate::OVR::OpenVR::EVROverlayError,
                        4usize,
                    >("GetOverlayDualAnalogTransform")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetOverlayDualAnalogTransform", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (ulOverlay, eWhich, pvCenter, pfRadius))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetOverlayErrorNameFromEnum(
        &mut self,
        error: crate::OVR::OpenVR::EVROverlayError,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::OVR::OpenVR::EVROverlayError),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        1usize,
                    >("GetOverlayErrorNameFromEnum")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetOverlayErrorNameFromEnum", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (error))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetOverlayFlag(
        &mut self,
        ulOverlayHandle: u64,
        eOverlayFlag: crate::OVR::OpenVR::VROverlayFlags,
        pbEnabled: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            crate::OVR::OpenVR::VROverlayFlags,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                        ),
                        crate::OVR::OpenVR::EVROverlayError,
                        3usize,
                    >("GetOverlayFlag")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetOverlayFlag", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle, eOverlayFlag, pbEnabled))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetOverlayFlags(
        &mut self,
        ulOverlayHandle: u64,
        pFlags: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u64, quest_hook::libil2cpp::ByRefMut<u32>),
                        crate::OVR::OpenVR::EVROverlayError,
                        2usize,
                    >("GetOverlayFlags")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetOverlayFlags", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle, pFlags))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetOverlayImageData(
        &mut self,
        ulOverlayHandle: u64,
        pvBuffer: crate::System::IntPtr,
        unBufferSize: u32,
        punWidth: quest_hook::libil2cpp::ByRefMut<u32>,
        punHeight: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            crate::System::IntPtr,
                            u32,
                            quest_hook::libil2cpp::ByRefMut<u32>,
                            quest_hook::libil2cpp::ByRefMut<u32>,
                        ),
                        crate::OVR::OpenVR::EVROverlayError,
                        5usize,
                    >("GetOverlayImageData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetOverlayImageData", 5usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (ulOverlayHandle, pvBuffer, unBufferSize, punWidth, punHeight),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetOverlayInputMethod(
        &mut self,
        ulOverlayHandle: u64,
        peInputMethod: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VROverlayInputMethod,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::VROverlayInputMethod,
                            >,
                        ),
                        crate::OVR::OpenVR::EVROverlayError,
                        2usize,
                    >("GetOverlayInputMethod")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetOverlayInputMethod", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle, peInputMethod))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetOverlayKey(
        &mut self,
        ulOverlayHandle: u64,
        pchValue: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unBufferSize: u32,
        pError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVROverlayError>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Text::StringBuilder,
                            >,
                            u32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::EVROverlayError,
                            >,
                        ),
                        u32,
                        4usize,
                    >("GetOverlayKey")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetOverlayKey", 4usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (ulOverlayHandle, pchValue, unBufferSize, pError),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetOverlayMouseScale(
        &mut self,
        ulOverlayHandle: u64,
        pvecMouseScale: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdVector2_t>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::HmdVector2_t,
                            >,
                        ),
                        crate::OVR::OpenVR::EVROverlayError,
                        2usize,
                    >("GetOverlayMouseScale")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetOverlayMouseScale", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle, pvecMouseScale))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetOverlayName(
        &mut self,
        ulOverlayHandle: u64,
        pchValue: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unBufferSize: u32,
        pError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVROverlayError>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Text::StringBuilder,
                            >,
                            u32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::EVROverlayError,
                            >,
                        ),
                        u32,
                        4usize,
                    >("GetOverlayName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetOverlayName", 4usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (ulOverlayHandle, pchValue, unBufferSize, pError),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetOverlayRenderModel(
        &mut self,
        ulOverlayHandle: u64,
        pchValue: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unBufferSize: u32,
        pColor: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdColor_t>,
        pError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVROverlayError>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Text::StringBuilder,
                            >,
                            u32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::HmdColor_t,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::EVROverlayError,
                            >,
                        ),
                        u32,
                        5usize,
                    >("GetOverlayRenderModel")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetOverlayRenderModel", 5usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (ulOverlayHandle, pchValue, unBufferSize, pColor, pError),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetOverlayRenderingPid(
        &mut self,
        ulOverlayHandle: u64,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(u64), u32, 1usize>("GetOverlayRenderingPid")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetOverlayRenderingPid", 1usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetOverlaySortOrder(
        &mut self,
        ulOverlayHandle: u64,
        punSortOrder: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u64, quest_hook::libil2cpp::ByRefMut<u32>),
                        crate::OVR::OpenVR::EVROverlayError,
                        2usize,
                    >("GetOverlaySortOrder")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetOverlaySortOrder", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle, punSortOrder))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetOverlayTexelAspect(
        &mut self,
        ulOverlayHandle: u64,
        pfTexelAspect: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u64, quest_hook::libil2cpp::ByRefMut<f32>),
                        crate::OVR::OpenVR::EVROverlayError,
                        2usize,
                    >("GetOverlayTexelAspect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetOverlayTexelAspect", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle, pfTexelAspect))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetOverlayTexture(
        &mut self,
        ulOverlayHandle: u64,
        pNativeTextureHandle: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
        pNativeTextureRef: crate::System::IntPtr,
        pWidth: quest_hook::libil2cpp::ByRefMut<u32>,
        pHeight: quest_hook::libil2cpp::ByRefMut<u32>,
        pNativeFormat: quest_hook::libil2cpp::ByRefMut<u32>,
        pAPIType: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::ETextureType>,
        pColorSpace: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EColorSpace>,
        pTextureBounds: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VRTextureBounds_t,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
                            crate::System::IntPtr,
                            quest_hook::libil2cpp::ByRefMut<u32>,
                            quest_hook::libil2cpp::ByRefMut<u32>,
                            quest_hook::libil2cpp::ByRefMut<u32>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::ETextureType,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::EColorSpace,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::VRTextureBounds_t,
                            >,
                        ),
                        crate::OVR::OpenVR::EVROverlayError,
                        9usize,
                    >("GetOverlayTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetOverlayTexture", 9usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        ulOverlayHandle,
                        pNativeTextureHandle,
                        pNativeTextureRef,
                        pWidth,
                        pHeight,
                        pNativeFormat,
                        pAPIType,
                        pColorSpace,
                        pTextureBounds,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetOverlayTextureBounds(
        &mut self,
        ulOverlayHandle: u64,
        pOverlayTextureBounds: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VRTextureBounds_t,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::VRTextureBounds_t,
                            >,
                        ),
                        crate::OVR::OpenVR::EVROverlayError,
                        2usize,
                    >("GetOverlayTextureBounds")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetOverlayTextureBounds", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle, pOverlayTextureBounds))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetOverlayTextureColorSpace(
        &mut self,
        ulOverlayHandle: u64,
        peTextureColorSpace: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::EColorSpace,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::EColorSpace,
                            >,
                        ),
                        crate::OVR::OpenVR::EVROverlayError,
                        2usize,
                    >("GetOverlayTextureColorSpace")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetOverlayTextureColorSpace", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle, peTextureColorSpace))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetOverlayTextureSize(
        &mut self,
        ulOverlayHandle: u64,
        pWidth: quest_hook::libil2cpp::ByRefMut<u32>,
        pHeight: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            quest_hook::libil2cpp::ByRefMut<u32>,
                            quest_hook::libil2cpp::ByRefMut<u32>,
                        ),
                        crate::OVR::OpenVR::EVROverlayError,
                        3usize,
                    >("GetOverlayTextureSize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetOverlayTextureSize", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle, pWidth, pHeight))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetOverlayTransformAbsolute(
        &mut self,
        ulOverlayHandle: u64,
        peTrackingOrigin: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::ETrackingUniverseOrigin,
        >,
        pmatTrackingOriginToOverlayTransform: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::ETrackingUniverseOrigin,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::HmdMatrix34_t,
                            >,
                        ),
                        crate::OVR::OpenVR::EVROverlayError,
                        3usize,
                    >("GetOverlayTransformAbsolute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetOverlayTransformAbsolute", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        ulOverlayHandle,
                        peTrackingOrigin,
                        pmatTrackingOriginToOverlayTransform,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetOverlayTransformOverlayRelative(
        &mut self,
        ulOverlayHandle: u64,
        ulOverlayHandleParent: quest_hook::libil2cpp::ByRefMut<u64>,
        pmatParentOverlayToOverlayTransform: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            quest_hook::libil2cpp::ByRefMut<u64>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::HmdMatrix34_t,
                            >,
                        ),
                        crate::OVR::OpenVR::EVROverlayError,
                        3usize,
                    >("GetOverlayTransformOverlayRelative")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetOverlayTransformOverlayRelative", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        ulOverlayHandle,
                        ulOverlayHandleParent,
                        pmatParentOverlayToOverlayTransform,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetOverlayTransformTrackedDeviceComponent(
        &mut self,
        ulOverlayHandle: u64,
        punDeviceIndex: quest_hook::libil2cpp::ByRefMut<u32>,
        pchComponentName: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unComponentNameSize: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            quest_hook::libil2cpp::ByRefMut<u32>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Text::StringBuilder,
                            >,
                            u32,
                        ),
                        crate::OVR::OpenVR::EVROverlayError,
                        4usize,
                    >("GetOverlayTransformTrackedDeviceComponent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetOverlayTransformTrackedDeviceComponent", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        ulOverlayHandle,
                        punDeviceIndex,
                        pchComponentName,
                        unComponentNameSize,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetOverlayTransformTrackedDeviceRelative(
        &mut self,
        ulOverlayHandle: u64,
        punTrackedDevice: quest_hook::libil2cpp::ByRefMut<u32>,
        pmatTrackedDeviceToOverlayTransform: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            quest_hook::libil2cpp::ByRefMut<u32>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::HmdMatrix34_t,
                            >,
                        ),
                        crate::OVR::OpenVR::EVROverlayError,
                        3usize,
                    >("GetOverlayTransformTrackedDeviceRelative")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetOverlayTransformTrackedDeviceRelative", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        ulOverlayHandle,
                        punTrackedDevice,
                        pmatTrackedDeviceToOverlayTransform,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetOverlayTransformType(
        &mut self,
        ulOverlayHandle: u64,
        peTransformType: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VROverlayTransformType,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::VROverlayTransformType,
                            >,
                        ),
                        crate::OVR::OpenVR::EVROverlayError,
                        2usize,
                    >("GetOverlayTransformType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetOverlayTransformType", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle, peTransformType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetOverlayWidthInMeters(
        &mut self,
        ulOverlayHandle: u64,
        pfWidthInMeters: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u64, quest_hook::libil2cpp::ByRefMut<f32>),
                        crate::OVR::OpenVR::EVROverlayError,
                        2usize,
                    >("GetOverlayWidthInMeters")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetOverlayWidthInMeters", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle, pfWidthInMeters))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPrimaryDashboardDevice(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), u32, 0usize>("GetPrimaryDashboardDevice")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetPrimaryDashboardDevice", 0usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetTransformForOverlayCoordinates(
        &mut self,
        ulOverlayHandle: u64,
        eTrackingOrigin: crate::OVR::OpenVR::ETrackingUniverseOrigin,
        coordinatesInOverlay: crate::OVR::OpenVR::HmdVector2_t,
        pmatTransform: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdMatrix34_t>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            crate::OVR::OpenVR::ETrackingUniverseOrigin,
                            crate::OVR::OpenVR::HmdVector2_t,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::HmdMatrix34_t,
                            >,
                        ),
                        crate::OVR::OpenVR::EVROverlayError,
                        4usize,
                    >("GetTransformForOverlayCoordinates")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTransformForOverlayCoordinates", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        ulOverlayHandle,
                        eTrackingOrigin,
                        coordinatesInOverlay,
                        pmatTransform,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HideKeyboard(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("HideKeyboard")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HideKeyboard", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HideOverlay(
        &mut self,
        ulOverlayHandle: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u64),
                        crate::OVR::OpenVR::EVROverlayError,
                        1usize,
                    >("HideOverlay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HideOverlay", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsActiveDashboardOverlay(
        &mut self,
        ulOverlayHandle: u64,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(u64), bool, 1usize>("IsActiveDashboardOverlay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsActiveDashboardOverlay", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsDashboardVisible(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("IsDashboardVisible")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsDashboardVisible", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn IsHoverTargetOverlay(
        &mut self,
        ulOverlayHandle: u64,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(u64), bool, 1usize>("IsHoverTargetOverlay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsHoverTargetOverlay", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsOverlayVisible(
        &mut self,
        ulOverlayHandle: u64,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(u64), bool, 1usize>("IsOverlayVisible")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsOverlayVisible", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MoveGamepadFocusToNeighbor(
        &mut self,
        eDirection: crate::OVR::OpenVR::EOverlayDirection,
        ulFrom: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::OVR::OpenVR::EOverlayDirection, u64),
                        crate::OVR::OpenVR::EVROverlayError,
                        2usize,
                    >("MoveGamepadFocusToNeighbor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MoveGamepadFocusToNeighbor", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (eDirection, ulFrom))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        pInterface: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pInterface))?;
        Ok(__cordl_object.into())
    }
    pub fn PollNextOverlayEvent(
        &mut self,
        ulOverlayHandle: u64,
        pEvent: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::VREvent_t>,
        uncbVREvent: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::VREvent_t,
                            >,
                            u32,
                        ),
                        bool,
                        3usize,
                    >("PollNextOverlayEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PollNextOverlayEvent", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle, pEvent, uncbVREvent))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReleaseNativeOverlayHandle(
        &mut self,
        ulOverlayHandle: u64,
        pNativeTextureHandle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u64, crate::System::IntPtr),
                        crate::OVR::OpenVR::EVROverlayError,
                        2usize,
                    >("ReleaseNativeOverlayHandle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReleaseNativeOverlayHandle", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle, pNativeTextureHandle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetDashboardOverlaySceneProcess(
        &mut self,
        ulOverlayHandle: u64,
        unProcessId: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u64, u32),
                        crate::OVR::OpenVR::EVROverlayError,
                        2usize,
                    >("SetDashboardOverlaySceneProcess")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetDashboardOverlaySceneProcess", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle, unProcessId))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGamepadFocusOverlay(
        &mut self,
        ulNewFocusOverlay: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u64),
                        crate::OVR::OpenVR::EVROverlayError,
                        1usize,
                    >("SetGamepadFocusOverlay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetGamepadFocusOverlay", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (ulNewFocusOverlay))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetHighQualityOverlay(
        &mut self,
        ulOverlayHandle: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u64),
                        crate::OVR::OpenVR::EVROverlayError,
                        1usize,
                    >("SetHighQualityOverlay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetHighQualityOverlay", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetKeyboardPositionForOverlay(
        &mut self,
        ulOverlayHandle: u64,
        avoidRect: crate::OVR::OpenVR::HmdRect2_t,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u64, crate::OVR::OpenVR::HmdRect2_t),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetKeyboardPositionForOverlay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetKeyboardPositionForOverlay", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle, avoidRect))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetKeyboardTransformAbsolute(
        &mut self,
        eTrackingOrigin: crate::OVR::OpenVR::ETrackingUniverseOrigin,
        pmatTrackingOriginToKeyboardTransform: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::OVR::OpenVR::ETrackingUniverseOrigin,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::HmdMatrix34_t,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetKeyboardTransformAbsolute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetKeyboardTransformAbsolute", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (eTrackingOrigin, pmatTrackingOriginToKeyboardTransform),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetOverlayAlpha(
        &mut self,
        ulOverlayHandle: u64,
        fAlpha: f32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u64, f32),
                        crate::OVR::OpenVR::EVROverlayError,
                        2usize,
                    >("SetOverlayAlpha")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetOverlayAlpha", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle, fAlpha))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetOverlayAutoCurveDistanceRangeInMeters(
        &mut self,
        ulOverlayHandle: u64,
        fMinDistanceInMeters: f32,
        fMaxDistanceInMeters: f32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u64, f32, f32),
                        crate::OVR::OpenVR::EVROverlayError,
                        3usize,
                    >("SetOverlayAutoCurveDistanceRangeInMeters")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetOverlayAutoCurveDistanceRangeInMeters", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (ulOverlayHandle, fMinDistanceInMeters, fMaxDistanceInMeters),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetOverlayColor(
        &mut self,
        ulOverlayHandle: u64,
        fRed: f32,
        fGreen: f32,
        fBlue: f32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u64, f32, f32, f32),
                        crate::OVR::OpenVR::EVROverlayError,
                        4usize,
                    >("SetOverlayColor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetOverlayColor", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle, fRed, fGreen, fBlue))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetOverlayDualAnalogTransform(
        &mut self,
        ulOverlay: u64,
        eWhich: crate::OVR::OpenVR::EDualAnalogWhich,
        vCenter: crate::System::IntPtr,
        fRadius: f32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            crate::OVR::OpenVR::EDualAnalogWhich,
                            crate::System::IntPtr,
                            f32,
                        ),
                        crate::OVR::OpenVR::EVROverlayError,
                        4usize,
                    >("SetOverlayDualAnalogTransform")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetOverlayDualAnalogTransform", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (ulOverlay, eWhich, vCenter, fRadius))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetOverlayFlag(
        &mut self,
        ulOverlayHandle: u64,
        eOverlayFlag: crate::OVR::OpenVR::VROverlayFlags,
        bEnabled: bool,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u64, crate::OVR::OpenVR::VROverlayFlags, bool),
                        crate::OVR::OpenVR::EVROverlayError,
                        3usize,
                    >("SetOverlayFlag")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetOverlayFlag", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle, eOverlayFlag, bEnabled))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetOverlayFromFile(
        &mut self,
        ulOverlayHandle: u64,
        pchFilePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        crate::OVR::OpenVR::EVROverlayError,
                        2usize,
                    >("SetOverlayFromFile")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetOverlayFromFile", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle, pchFilePath))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetOverlayInputMethod(
        &mut self,
        ulOverlayHandle: u64,
        eInputMethod: crate::OVR::OpenVR::VROverlayInputMethod,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u64, crate::OVR::OpenVR::VROverlayInputMethod),
                        crate::OVR::OpenVR::EVROverlayError,
                        2usize,
                    >("SetOverlayInputMethod")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetOverlayInputMethod", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle, eInputMethod))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetOverlayIntersectionMask(
        &mut self,
        ulOverlayHandle: u64,
        pMaskPrimitives: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VROverlayIntersectionMaskPrimitive_t,
        >,
        unNumMaskPrimitives: u32,
        unPrimitiveSize: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::VROverlayIntersectionMaskPrimitive_t,
                            >,
                            u32,
                            u32,
                        ),
                        crate::OVR::OpenVR::EVROverlayError,
                        4usize,
                    >("SetOverlayIntersectionMask")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetOverlayIntersectionMask", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        ulOverlayHandle,
                        pMaskPrimitives,
                        unNumMaskPrimitives,
                        unPrimitiveSize,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetOverlayMouseScale(
        &mut self,
        ulOverlayHandle: u64,
        pvecMouseScale: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdVector2_t>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::HmdVector2_t,
                            >,
                        ),
                        crate::OVR::OpenVR::EVROverlayError,
                        2usize,
                    >("SetOverlayMouseScale")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetOverlayMouseScale", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle, pvecMouseScale))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetOverlayName(
        &mut self,
        ulOverlayHandle: u64,
        pchName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        crate::OVR::OpenVR::EVROverlayError,
                        2usize,
                    >("SetOverlayName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetOverlayName", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle, pchName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetOverlayNeighbor(
        &mut self,
        eDirection: crate::OVR::OpenVR::EOverlayDirection,
        ulFrom: u64,
        ulTo: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::OVR::OpenVR::EOverlayDirection, u64, u64),
                        crate::OVR::OpenVR::EVROverlayError,
                        3usize,
                    >("SetOverlayNeighbor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetOverlayNeighbor", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (eDirection, ulFrom, ulTo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetOverlayRaw(
        &mut self,
        ulOverlayHandle: u64,
        pvBuffer: crate::System::IntPtr,
        unWidth: u32,
        unHeight: u32,
        unDepth: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u64, crate::System::IntPtr, u32, u32, u32),
                        crate::OVR::OpenVR::EVROverlayError,
                        5usize,
                    >("SetOverlayRaw")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetOverlayRaw", 5usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (ulOverlayHandle, pvBuffer, unWidth, unHeight, unDepth),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetOverlayRenderModel(
        &mut self,
        ulOverlayHandle: u64,
        pchRenderModel: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pColor: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdColor_t>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::HmdColor_t,
                            >,
                        ),
                        crate::OVR::OpenVR::EVROverlayError,
                        3usize,
                    >("SetOverlayRenderModel")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetOverlayRenderModel", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle, pchRenderModel, pColor))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetOverlayRenderingPid(
        &mut self,
        ulOverlayHandle: u64,
        unPID: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u64, u32),
                        crate::OVR::OpenVR::EVROverlayError,
                        2usize,
                    >("SetOverlayRenderingPid")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetOverlayRenderingPid", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle, unPID))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetOverlaySortOrder(
        &mut self,
        ulOverlayHandle: u64,
        unSortOrder: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u64, u32),
                        crate::OVR::OpenVR::EVROverlayError,
                        2usize,
                    >("SetOverlaySortOrder")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetOverlaySortOrder", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle, unSortOrder))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetOverlayTexelAspect(
        &mut self,
        ulOverlayHandle: u64,
        fTexelAspect: f32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u64, f32),
                        crate::OVR::OpenVR::EVROverlayError,
                        2usize,
                    >("SetOverlayTexelAspect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetOverlayTexelAspect", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle, fTexelAspect))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetOverlayTexture(
        &mut self,
        ulOverlayHandle: u64,
        pTexture: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::Texture_t>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::Texture_t,
                            >,
                        ),
                        crate::OVR::OpenVR::EVROverlayError,
                        2usize,
                    >("SetOverlayTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetOverlayTexture", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle, pTexture))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetOverlayTextureBounds(
        &mut self,
        ulOverlayHandle: u64,
        pOverlayTextureBounds: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VRTextureBounds_t,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::VRTextureBounds_t,
                            >,
                        ),
                        crate::OVR::OpenVR::EVROverlayError,
                        2usize,
                    >("SetOverlayTextureBounds")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetOverlayTextureBounds", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle, pOverlayTextureBounds))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetOverlayTextureColorSpace(
        &mut self,
        ulOverlayHandle: u64,
        eTextureColorSpace: crate::OVR::OpenVR::EColorSpace,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u64, crate::OVR::OpenVR::EColorSpace),
                        crate::OVR::OpenVR::EVROverlayError,
                        2usize,
                    >("SetOverlayTextureColorSpace")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetOverlayTextureColorSpace", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle, eTextureColorSpace))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetOverlayTransformAbsolute(
        &mut self,
        ulOverlayHandle: u64,
        eTrackingOrigin: crate::OVR::OpenVR::ETrackingUniverseOrigin,
        pmatTrackingOriginToOverlayTransform: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            crate::OVR::OpenVR::ETrackingUniverseOrigin,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::HmdMatrix34_t,
                            >,
                        ),
                        crate::OVR::OpenVR::EVROverlayError,
                        3usize,
                    >("SetOverlayTransformAbsolute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetOverlayTransformAbsolute", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        ulOverlayHandle,
                        eTrackingOrigin,
                        pmatTrackingOriginToOverlayTransform,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetOverlayTransformOverlayRelative(
        &mut self,
        ulOverlayHandle: u64,
        ulOverlayHandleParent: u64,
        pmatParentOverlayToOverlayTransform: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            u64,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::HmdMatrix34_t,
                            >,
                        ),
                        crate::OVR::OpenVR::EVROverlayError,
                        3usize,
                    >("SetOverlayTransformOverlayRelative")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetOverlayTransformOverlayRelative", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        ulOverlayHandle,
                        ulOverlayHandleParent,
                        pmatParentOverlayToOverlayTransform,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetOverlayTransformTrackedDeviceComponent(
        &mut self,
        ulOverlayHandle: u64,
        unDeviceIndex: u32,
        pchComponentName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            u32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        crate::OVR::OpenVR::EVROverlayError,
                        3usize,
                    >("SetOverlayTransformTrackedDeviceComponent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetOverlayTransformTrackedDeviceComponent", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (ulOverlayHandle, unDeviceIndex, pchComponentName),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetOverlayTransformTrackedDeviceRelative(
        &mut self,
        ulOverlayHandle: u64,
        unTrackedDevice: u32,
        pmatTrackedDeviceToOverlayTransform: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            u32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::HmdMatrix34_t,
                            >,
                        ),
                        crate::OVR::OpenVR::EVROverlayError,
                        3usize,
                    >("SetOverlayTransformTrackedDeviceRelative")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetOverlayTransformTrackedDeviceRelative", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        ulOverlayHandle,
                        unTrackedDevice,
                        pmatTrackedDeviceToOverlayTransform,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetOverlayWidthInMeters(
        &mut self,
        ulOverlayHandle: u64,
        fWidthInMeters: f32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u64, f32),
                        crate::OVR::OpenVR::EVROverlayError,
                        2usize,
                    >("SetOverlayWidthInMeters")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetOverlayWidthInMeters", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle, fWidthInMeters))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShowDashboard(
        &mut self,
        pchOverlayToShow: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ShowDashboard")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ShowDashboard", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (pchOverlayToShow))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShowKeyboard(
        &mut self,
        eInputMode: i32,
        eLineInputMode: i32,
        pchDescription: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        unCharMax: u32,
        pchExistingText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bUseMinimalMode: bool,
        uUserValue: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            u32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            bool,
                            u64,
                        ),
                        crate::OVR::OpenVR::EVROverlayError,
                        7usize,
                    >("ShowKeyboard")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ShowKeyboard", 7usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        eInputMode,
                        eLineInputMode,
                        pchDescription,
                        unCharMax,
                        pchExistingText,
                        bUseMinimalMode,
                        uUserValue,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShowKeyboardForOverlay(
        &mut self,
        ulOverlayHandle: u64,
        eInputMode: i32,
        eLineInputMode: i32,
        pchDescription: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        unCharMax: u32,
        pchExistingText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bUseMinimalMode: bool,
        uUserValue: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            u32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            bool,
                            u64,
                        ),
                        crate::OVR::OpenVR::EVROverlayError,
                        8usize,
                    >("ShowKeyboardForOverlay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ShowKeyboardForOverlay", 8usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        ulOverlayHandle,
                        eInputMode,
                        eLineInputMode,
                        pchDescription,
                        unCharMax,
                        pchExistingText,
                        bUseMinimalMode,
                        uUserValue,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShowMessageOverlay(
        &mut self,
        pchText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchCaption: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchButton0Text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchButton1Text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchButton2Text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchButton3Text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::VRMessageOverlayResponse> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        crate::OVR::OpenVR::VRMessageOverlayResponse,
                        6usize,
                    >("ShowMessageOverlay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ShowMessageOverlay", 6usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::VRMessageOverlayResponse = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        pchText,
                        pchCaption,
                        pchButton0Text,
                        pchButton1Text,
                        pchButton2Text,
                        pchButton3Text,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShowOverlay(
        &mut self,
        ulOverlayHandle: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u64),
                        crate::OVR::OpenVR::EVROverlayError,
                        1usize,
                    >("ShowOverlay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ShowOverlay", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        pInterface: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::IntPtr),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (pInterface))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+CVROverlay")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::CVROverlay {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+CVROverlay+PollNextOverlayEventUnion")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CVROverlay_PollNextOverlayEventUnion {
    padding: quest_hook::libil2cpp::ValueTypePadding<8usize>,
}
#[cfg(feature = "OVR+OpenVR+CVROverlay+PollNextOverlayEventUnion")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::CVROverlay_PollNextOverlayEventUnion {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "CVROverlay/PollNextOverlayEventUnion";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "OVR+OpenVR+CVROverlay+PollNextOverlayEventUnion")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::OVR::OpenVR::CVROverlay_PollNextOverlayEventUnion {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVR+OpenVR+CVROverlay+PollNextOverlayEventUnion")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::OVR::OpenVR::CVROverlay_PollNextOverlayEventUnion {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "OVR+OpenVR+CVROverlay+PollNextOverlayEventUnion")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::OVR::OpenVR::CVROverlay_PollNextOverlayEventUnion {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "OVR+OpenVR+CVROverlay+PollNextOverlayEventUnion")]
unsafe impl quest_hook::libil2cpp::Return
for crate::OVR::OpenVR::CVROverlay_PollNextOverlayEventUnion {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "OVR+OpenVR+CVROverlay+PollNextOverlayEventUnion")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::OVR::OpenVR::CVROverlay_PollNextOverlayEventUnion {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+CVROverlay+PollNextOverlayEventUnion")]
impl crate::OVR::OpenVR::CVROverlay_PollNextOverlayEventUnion {}
#[cfg(feature = "OVR+OpenVR+CVROverlay+_PollNextOverlayEventPacked")]
#[repr(C)]
#[derive(Debug)]
pub struct CVROverlay__PollNextOverlayEventPacked {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+CVROverlay+_PollNextOverlayEventPacked")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::CVROverlay__PollNextOverlayEventPacked {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "CVROverlay/_PollNextOverlayEventPacked";
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
#[cfg(feature = "OVR+OpenVR+CVROverlay+_PollNextOverlayEventPacked")]
impl std::ops::Deref for crate::OVR::OpenVR::CVROverlay__PollNextOverlayEventPacked {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVROverlay+_PollNextOverlayEventPacked")]
impl std::ops::DerefMut for crate::OVR::OpenVR::CVROverlay__PollNextOverlayEventPacked {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVROverlay+_PollNextOverlayEventPacked")]
impl crate::OVR::OpenVR::CVROverlay__PollNextOverlayEventPacked {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        pEvent: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::VREvent_t_Packed>,
        uncbVREvent: u32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::VREvent_t_Packed,
                            >,
                            u32,
                            quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                        5usize,
                    >("BeginInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BeginInvoke", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (ulOverlayHandle, pEvent, uncbVREvent, callback, object),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pEvent: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::VREvent_t_Packed>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::VREvent_t_Packed,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                        ),
                        bool,
                        2usize,
                    >("EndInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EndInvoke", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (pEvent, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        pEvent: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::VREvent_t_Packed>,
        uncbVREvent: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::VREvent_t_Packed,
                            >,
                            u32,
                        ),
                        bool,
                        3usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Invoke",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (ulOverlayHandle, pEvent, uncbVREvent))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+CVROverlay+_PollNextOverlayEventPacked")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::CVROverlay__PollNextOverlayEventPacked {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
