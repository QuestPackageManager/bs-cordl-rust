#[cfg(feature = "cordl_class_OVR+OpenVR+CVRRenderModels")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct CVRRenderModels {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub FnTable: crate::OVR::OpenVR::IVRRenderModels,
}
#[cfg(feature = "cordl_class_OVR+OpenVR+CVRRenderModels")]
unsafe impl quest_hook::libil2cpp::Type for crate::OVR::OpenVR::CVRRenderModels {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "CVRRenderModels";
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
#[cfg(feature = "OVR+OpenVR+CVRRenderModels")]
impl std::ops::Deref for crate::OVR::OpenVR::CVRRenderModels {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRRenderModels")]
impl std::ops::DerefMut for crate::OVR::OpenVR::CVRRenderModels {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRRenderModels")]
impl crate::OVR::OpenVR::CVRRenderModels {
    #[cfg(feature = "OVR+OpenVR+CVRRenderModels+GetComponentStateUnion")]
    pub type GetComponentStateUnion = crate::OVR::OpenVR::CVRRenderModels_GetComponentStateUnion;
    #[cfg(feature = "OVR+OpenVR+CVRRenderModels+_GetComponentStatePacked")]
    pub type _GetComponentStatePacked =
        crate::OVR::OpenVR::CVRRenderModels__GetComponentStatePacked;
    pub fn FreeRenderModel(
        &mut self,
        pRenderModel: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::System::IntPtr), quest_hook::libil2cpp::Void, 1usize>(
                        "FreeRenderModel",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FreeRenderModel",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (pRenderModel))? };
        Ok(__cordl_ret.into())
    }
    pub fn FreeTexture(
        &mut self,
        pTexture: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::System::IntPtr), quest_hook::libil2cpp::Void, 1usize>(
                        "FreeTexture",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FreeTexture",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (pTexture))? };
        Ok(__cordl_ret.into())
    }
    pub fn FreeTextureD3D11(
        &mut self,
        pD3D11Texture2D: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::System::IntPtr), quest_hook::libil2cpp::Void, 1usize>(
                        "FreeTextureD3D11",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FreeTextureD3D11",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (pD3D11Texture2D))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetComponentButtonMask(
        &mut self,
        pchRenderModelName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchComponentName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), u64, 2usize>("GetComponentButtonMask")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetComponentButtonMask",
                            2usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe {
            cordl_method_info.invoke_unchecked(self, (pchRenderModelName, pchComponentName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetComponentCount(
        &mut self,
        pchRenderModelName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        u32,
                        1usize,
                    >("GetComponentCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetComponentCount", 1usize
                        )
                    })
            });
        let __cordl_ret: u32 =
            unsafe { cordl_method_info.invoke_unchecked(self, (pchRenderModelName))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetComponentName(
        &mut self,
        pchRenderModelName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        unComponentIndex: u32,
        pchComponentName: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unComponentNameLen: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        u32,
                        quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
                        u32,
                    ), u32, 4usize>("GetComponentName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetComponentName",
                            4usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    pchRenderModelName,
                    unComponentIndex,
                    pchComponentName,
                    unComponentNameLen,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetComponentRenderModelName(
        &mut self,
        pchRenderModelName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchComponentName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchComponentRenderModelName: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unComponentRenderModelNameLen: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
                        u32,
                    ), u32, 4usize>("GetComponentRenderModelName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetComponentRenderModelName",
                            4usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    pchRenderModelName,
                    pchComponentName,
                    pchComponentRenderModelName,
                    unComponentRenderModelNameLen,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetComponentState(
        &mut self,
        pchRenderModelName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchComponentName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pControllerState: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::VRControllerState_t>,
        pState: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::RenderModel_ControllerMode_State_t,
        >,
        pComponentState: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::RenderModel_ComponentState_t,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::VRControllerState_t>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::OVR::OpenVR::RenderModel_ControllerMode_State_t,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::OVR::OpenVR::RenderModel_ComponentState_t,
                        >,
                    ), bool, 5usize>("GetComponentState")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetComponentState",
                            5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    pchRenderModelName,
                    pchComponentName,
                    pControllerState,
                    pState,
                    pComponentState,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetComponentStateForDevicePath(
        &mut self,
        pchRenderModelName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchComponentName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        devicePath: u64,
        pState: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::RenderModel_ControllerMode_State_t,
        >,
        pComponentState: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::RenderModel_ComponentState_t,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        u64,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::OVR::OpenVR::RenderModel_ControllerMode_State_t,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::OVR::OpenVR::RenderModel_ComponentState_t,
                        >,
                    ), bool, 5usize>("GetComponentStateForDevicePath")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetComponentStateForDevicePath",
                            5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    pchRenderModelName,
                    pchComponentName,
                    devicePath,
                    pState,
                    pComponentState,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetRenderModelCount(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), u32, 0usize>("GetRenderModelCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetRenderModelCount",
                            0usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetRenderModelErrorNameFromEnum(
        &mut self,
        error: crate::OVR::OpenVR::EVRRenderModelError,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::OVR::OpenVR::EVRRenderModelError),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        1usize,
                    >("GetRenderModelErrorNameFromEnum")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetRenderModelErrorNameFromEnum", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, (error))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetRenderModelName(
        &mut self,
        unRenderModelIndex: u32,
        pchRenderModelName: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unRenderModelNameLen: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        u32,
                        quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
                        u32,
                    ), u32, 3usize>("GetRenderModelName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetRenderModelName",
                            3usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (unRenderModelIndex, pchRenderModelName, unRenderModelNameLen),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetRenderModelOriginalPath(
        &mut self,
        pchRenderModelName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchOriginalPath: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unOriginalPathLen: u32,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRRenderModelError>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
                        u32,
                        quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRRenderModelError>,
                    ), u32, 4usize>("GetRenderModelOriginalPath")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetRenderModelOriginalPath",
                            4usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    pchRenderModelName,
                    pchOriginalPath,
                    unOriginalPathLen,
                    peError,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetRenderModelThumbnailURL(
        &mut self,
        pchRenderModelName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchThumbnailURL: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unThumbnailURLLen: u32,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRRenderModelError>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
                        u32,
                        quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRRenderModelError>,
                    ), u32, 4usize>("GetRenderModelThumbnailURL")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetRenderModelThumbnailURL",
                            4usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    pchRenderModelName,
                    pchThumbnailURL,
                    unThumbnailURLLen,
                    peError,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadIntoTextureD3D11_Async(
        &mut self,
        textureId: i32,
        pDstTexture: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRRenderModelError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, crate::System::IntPtr),
                        crate::OVR::OpenVR::EVRRenderModelError,
                        2usize,
                    >("LoadIntoTextureD3D11_Async")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LoadIntoTextureD3D11_Async", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRRenderModelError =
            unsafe { cordl_method_info.invoke_unchecked(self, (textureId, pDstTexture))? };
        Ok(__cordl_ret.into())
    }
    pub fn LoadRenderModel_Async(
        &mut self,
        pchRenderModelName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ppRenderModel: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRRenderModelError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
                    ), crate::OVR::OpenVR::EVRRenderModelError, 2usize>(
                        "LoadRenderModel_Async"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LoadRenderModel_Async",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRRenderModelError = unsafe {
            cordl_method_info.invoke_unchecked(self, (pchRenderModelName, ppRenderModel))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadTextureD3D11_Async(
        &mut self,
        textureId: i32,
        pD3D11Device: crate::System::IntPtr,
        ppD3D11Texture2D: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRRenderModelError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
                    ), crate::OVR::OpenVR::EVRRenderModelError, 3usize>(
                        "LoadTextureD3D11_Async"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LoadTextureD3D11_Async",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRRenderModelError = unsafe {
            cordl_method_info.invoke_unchecked(self, (textureId, pD3D11Device, ppD3D11Texture2D))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadTexture_Async(
        &mut self,
        textureId: i32,
        ppTexture: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRRenderModelError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>),
                        crate::OVR::OpenVR::EVRRenderModelError,
                        2usize,
                    >("LoadTexture_Async")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LoadTexture_Async", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRRenderModelError =
            unsafe { cordl_method_info.invoke_unchecked(self, (textureId, ppTexture))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        pInterface: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pInterface))?;
        Ok(__cordl_object.into())
    }
    pub fn RenderModelHasComponent(
        &mut self,
        pchRenderModelName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchComponentName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), bool, 2usize>("RenderModelHasComponent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RenderModelHasComponent",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (pchRenderModelName, pchComponentName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        pInterface: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::System::IntPtr), quest_hook::libil2cpp::Void, 1usize>(
                        ".ctor",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (pInterface))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OVR+OpenVR+CVRRenderModels")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::CVRRenderModels {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVR+OpenVR+CVRRenderModels+GetComponentStateUnion")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[repr(C)]
pub struct CVRRenderModels_GetComponentStateUnion {
    padding: quest_hook::libil2cpp::ValueTypePadding<8usize>,
}
#[cfg(feature = "cordl_class_OVR+OpenVR+CVRRenderModels+GetComponentStateUnion")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::OVR::OpenVR::CVRRenderModels_GetComponentStateUnion
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "CVRRenderModels/GetComponentStateUnion";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_OVR+OpenVR+CVRRenderModels+GetComponentStateUnion")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::OVR::OpenVR::CVRRenderModels_GetComponentStateUnion
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVR+OpenVR+CVRRenderModels+GetComponentStateUnion")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::OVR::OpenVR::CVRRenderModels_GetComponentStateUnion
{
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
#[cfg(feature = "cordl_class_OVR+OpenVR+CVRRenderModels+GetComponentStateUnion")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::OVR::OpenVR::CVRRenderModels_GetComponentStateUnion
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_OVR+OpenVR+CVRRenderModels+GetComponentStateUnion")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::OVR::OpenVR::CVRRenderModels_GetComponentStateUnion
{
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
#[cfg(feature = "cordl_class_OVR+OpenVR+CVRRenderModels+GetComponentStateUnion")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::OVR::OpenVR::CVRRenderModels_GetComponentStateUnion
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRRenderModels+GetComponentStateUnion")]
impl crate::OVR::OpenVR::CVRRenderModels_GetComponentStateUnion {}
#[cfg(feature = "cordl_class_OVR+OpenVR+CVRRenderModels+_GetComponentStatePacked")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct CVRRenderModels__GetComponentStatePacked {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "cordl_class_OVR+OpenVR+CVRRenderModels+_GetComponentStatePacked")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::OVR::OpenVR::CVRRenderModels__GetComponentStatePacked
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "CVRRenderModels/_GetComponentStatePacked";
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
#[cfg(feature = "OVR+OpenVR+CVRRenderModels+_GetComponentStatePacked")]
impl std::ops::Deref for crate::OVR::OpenVR::CVRRenderModels__GetComponentStatePacked {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRRenderModels+_GetComponentStatePacked")]
impl std::ops::DerefMut for crate::OVR::OpenVR::CVRRenderModels__GetComponentStatePacked {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRRenderModels+_GetComponentStatePacked")]
impl crate::OVR::OpenVR::CVRRenderModels__GetComponentStatePacked {
    pub fn BeginInvoke(
        &mut self,
        pchRenderModelName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchComponentName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pControllerState: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VRControllerState_t_Packed,
        >,
        pState: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::RenderModel_ControllerMode_State_t,
        >,
        pComponentState: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::RenderModel_ComponentState_t,
        >,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::OVR::OpenVR::VRControllerState_t_Packed,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::OVR::OpenVR::RenderModel_ControllerMode_State_t,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::OVR::OpenVR::RenderModel_ComponentState_t,
                        >,
                        quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>, 7usize>(
                        "BeginInvoke",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BeginInvoke",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    pchRenderModelName,
                    pchComponentName,
                    pControllerState,
                    pState,
                    pComponentState,
                    callback,
                    object,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pControllerState: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VRControllerState_t_Packed,
        >,
        pState: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::RenderModel_ControllerMode_State_t,
        >,
        pComponentState: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::RenderModel_ComponentState_t,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::OVR::OpenVR::VRControllerState_t_Packed,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::OVR::OpenVR::RenderModel_ControllerMode_State_t,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::OVR::OpenVR::RenderModel_ComponentState_t,
                        >,
                        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                    ), bool, 4usize>("EndInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EndInvoke",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (pControllerState, pState, pComponentState, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchRenderModelName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchComponentName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pControllerState: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VRControllerState_t_Packed,
        >,
        pState: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::RenderModel_ControllerMode_State_t,
        >,
        pComponentState: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::RenderModel_ComponentState_t,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::OVR::OpenVR::VRControllerState_t_Packed,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::OVR::OpenVR::RenderModel_ControllerMode_State_t,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::OVR::OpenVR::RenderModel_ComponentState_t,
                        >,
                    ), bool, 5usize>("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Invoke",
                            5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    pchRenderModelName,
                    pchComponentName,
                    pControllerState,
                    pState,
                    pComponentState,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        crate::System::IntPtr,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (object, method))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OVR+OpenVR+CVRRenderModels+_GetComponentStatePacked")]
impl quest_hook::libil2cpp::ObjectType
    for crate::OVR::OpenVR::CVRRenderModels__GetComponentStatePacked
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
