#[cfg(feature = "OVR+OpenVR+CVROverlay")]
#[repr(C)]
#[derive(Debug)]
pub struct CVROverlay {
    __cordl_parent: crate::System::Object,
    pub FnTable: crate::OVR::OpenVR::IVROverlay,
}
#[cfg(feature = "OVR+OpenVR+CVROverlay")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::CVROverlay => "OVR.OpenVR"
    ."CVROverlay"
);
#[cfg(feature = "OVR+OpenVR+CVROverlay")]
impl std::ops::Deref for crate::OVR::OpenVR::CVROverlay {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVROverlay")]
impl std::ops::DerefMut for crate::OVR::OpenVR::CVROverlay {
    fn deref_mut(&mut self) -> &mut Self::Target {
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("ClearOverlayTexture", (ulOverlayHandle))?;
        Ok(__cordl_ret)
    }
    pub fn CloseMessageOverlay(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CloseMessageOverlay", ())?;
        Ok(__cordl_ret)
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ComputeOverlayIntersection", (ulOverlayHandle, pParams, pResults))?;
        Ok(__cordl_ret)
    }
    pub fn CreateDashboardOverlay(
        &mut self,
        pchOverlayKey: *mut crate::System::String,
        pchOverlayFriendlyName: *mut crate::System::String,
        pMainHandle: quest_hook::libil2cpp::ByRefMut<u64>,
        pThumbnailHandle: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "CreateDashboardOverlay",
                (pchOverlayKey, pchOverlayFriendlyName, pMainHandle, pThumbnailHandle),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CreateOverlay(
        &mut self,
        pchOverlayKey: *mut crate::System::String,
        pchOverlayName: *mut crate::System::String,
        pOverlayHandle: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("CreateOverlay", (pchOverlayKey, pchOverlayName, pOverlayHandle))?;
        Ok(__cordl_ret)
    }
    pub fn DestroyOverlay(
        &mut self,
        ulOverlayHandle: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("DestroyOverlay", (ulOverlayHandle))?;
        Ok(__cordl_ret)
    }
    pub fn FindOverlay(
        &mut self,
        pchOverlayKey: *mut crate::System::String,
        pOverlayHandle: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("FindOverlay", (pchOverlayKey, pOverlayHandle))?;
        Ok(__cordl_ret)
    }
    pub fn GetDashboardOverlaySceneProcess(
        &mut self,
        ulOverlayHandle: u64,
        punProcessId: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("GetDashboardOverlaySceneProcess", (ulOverlayHandle, punProcessId))?;
        Ok(__cordl_ret)
    }
    pub fn GetGamepadFocusOverlay(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object.invoke("GetGamepadFocusOverlay", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetHighQualityOverlay(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object.invoke("GetHighQualityOverlay", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetKeyboardText(
        &mut self,
        pchText: *mut crate::System::Text::StringBuilder,
        cchText: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke("GetKeyboardText", (pchText, cchText))?;
        Ok(__cordl_ret)
    }
    pub fn GetOverlayAlpha(
        &mut self,
        ulOverlayHandle: u64,
        pfAlpha: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("GetOverlayAlpha", (ulOverlayHandle, pfAlpha))?;
        Ok(__cordl_ret)
    }
    pub fn GetOverlayAutoCurveDistanceRangeInMeters(
        &mut self,
        ulOverlayHandle: u64,
        pfMinDistanceInMeters: quest_hook::libil2cpp::ByRefMut<f32>,
        pfMaxDistanceInMeters: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "GetOverlayAutoCurveDistanceRangeInMeters",
                (ulOverlayHandle, pfMinDistanceInMeters, pfMaxDistanceInMeters),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetOverlayColor(
        &mut self,
        ulOverlayHandle: u64,
        pfRed: quest_hook::libil2cpp::ByRefMut<f32>,
        pfGreen: quest_hook::libil2cpp::ByRefMut<f32>,
        pfBlue: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("GetOverlayColor", (ulOverlayHandle, pfRed, pfGreen, pfBlue))?;
        Ok(__cordl_ret)
    }
    pub fn GetOverlayDualAnalogTransform(
        &mut self,
        ulOverlay: u64,
        eWhich: crate::OVR::OpenVR::EDualAnalogWhich,
        pvCenter: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdVector2_t>,
        pfRadius: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "GetOverlayDualAnalogTransform",
                (ulOverlay, eWhich, pvCenter, pfRadius),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetOverlayErrorNameFromEnum(
        &mut self,
        error: crate::OVR::OpenVR::EVROverlayError,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetOverlayErrorNameFromEnum", (error))?;
        Ok(__cordl_ret)
    }
    pub fn GetOverlayFlag(
        &mut self,
        ulOverlayHandle: u64,
        eOverlayFlag: crate::OVR::OpenVR::VROverlayFlags,
        pbEnabled: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("GetOverlayFlag", (ulOverlayHandle, eOverlayFlag, pbEnabled))?;
        Ok(__cordl_ret)
    }
    pub fn GetOverlayFlags(
        &mut self,
        ulOverlayHandle: u64,
        pFlags: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("GetOverlayFlags", (ulOverlayHandle, pFlags))?;
        Ok(__cordl_ret)
    }
    pub fn GetOverlayImageData(
        &mut self,
        ulOverlayHandle: u64,
        pvBuffer: crate::System::IntPtr,
        unBufferSize: u32,
        punWidth: quest_hook::libil2cpp::ByRefMut<u32>,
        punHeight: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "GetOverlayImageData",
                (ulOverlayHandle, pvBuffer, unBufferSize, punWidth, punHeight),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetOverlayInputMethod(
        &mut self,
        ulOverlayHandle: u64,
        peInputMethod: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VROverlayInputMethod,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("GetOverlayInputMethod", (ulOverlayHandle, peInputMethod))?;
        Ok(__cordl_ret)
    }
    pub fn GetOverlayKey(
        &mut self,
        ulOverlayHandle: u64,
        pchValue: *mut crate::System::Text::StringBuilder,
        unBufferSize: u32,
        pError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVROverlayError>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke("GetOverlayKey", (ulOverlayHandle, pchValue, unBufferSize, pError))?;
        Ok(__cordl_ret)
    }
    pub fn GetOverlayMouseScale(
        &mut self,
        ulOverlayHandle: u64,
        pvecMouseScale: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdVector2_t>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("GetOverlayMouseScale", (ulOverlayHandle, pvecMouseScale))?;
        Ok(__cordl_ret)
    }
    pub fn GetOverlayName(
        &mut self,
        ulOverlayHandle: u64,
        pchValue: *mut crate::System::Text::StringBuilder,
        unBufferSize: u32,
        pError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVROverlayError>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke(
                "GetOverlayName",
                (ulOverlayHandle, pchValue, unBufferSize, pError),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetOverlayRenderModel(
        &mut self,
        ulOverlayHandle: u64,
        pchValue: *mut crate::System::Text::StringBuilder,
        unBufferSize: u32,
        pColor: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdColor_t>,
        pError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVROverlayError>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke(
                "GetOverlayRenderModel",
                (ulOverlayHandle, pchValue, unBufferSize, pColor, pError),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetOverlayRenderingPid(
        &mut self,
        ulOverlayHandle: u64,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke("GetOverlayRenderingPid", (ulOverlayHandle))?;
        Ok(__cordl_ret)
    }
    pub fn GetOverlaySortOrder(
        &mut self,
        ulOverlayHandle: u64,
        punSortOrder: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("GetOverlaySortOrder", (ulOverlayHandle, punSortOrder))?;
        Ok(__cordl_ret)
    }
    pub fn GetOverlayTexelAspect(
        &mut self,
        ulOverlayHandle: u64,
        pfTexelAspect: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("GetOverlayTexelAspect", (ulOverlayHandle, pfTexelAspect))?;
        Ok(__cordl_ret)
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "GetOverlayTexture",
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
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetOverlayTextureBounds(
        &mut self,
        ulOverlayHandle: u64,
        pOverlayTextureBounds: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VRTextureBounds_t,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "GetOverlayTextureBounds",
                (ulOverlayHandle, pOverlayTextureBounds),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetOverlayTextureColorSpace(
        &mut self,
        ulOverlayHandle: u64,
        peTextureColorSpace: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::EColorSpace,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "GetOverlayTextureColorSpace",
                (ulOverlayHandle, peTextureColorSpace),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetOverlayTextureSize(
        &mut self,
        ulOverlayHandle: u64,
        pWidth: quest_hook::libil2cpp::ByRefMut<u32>,
        pHeight: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("GetOverlayTextureSize", (ulOverlayHandle, pWidth, pHeight))?;
        Ok(__cordl_ret)
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "GetOverlayTransformAbsolute",
                (ulOverlayHandle, peTrackingOrigin, pmatTrackingOriginToOverlayTransform),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetOverlayTransformOverlayRelative(
        &mut self,
        ulOverlayHandle: u64,
        ulOverlayHandleParent: quest_hook::libil2cpp::ByRefMut<u64>,
        pmatParentOverlayToOverlayTransform: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "GetOverlayTransformOverlayRelative",
                (
                    ulOverlayHandle,
                    ulOverlayHandleParent,
                    pmatParentOverlayToOverlayTransform,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetOverlayTransformTrackedDeviceComponent(
        &mut self,
        ulOverlayHandle: u64,
        punDeviceIndex: quest_hook::libil2cpp::ByRefMut<u32>,
        pchComponentName: *mut crate::System::Text::StringBuilder,
        unComponentNameSize: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "GetOverlayTransformTrackedDeviceComponent",
                (ulOverlayHandle, punDeviceIndex, pchComponentName, unComponentNameSize),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetOverlayTransformTrackedDeviceRelative(
        &mut self,
        ulOverlayHandle: u64,
        punTrackedDevice: quest_hook::libil2cpp::ByRefMut<u32>,
        pmatTrackedDeviceToOverlayTransform: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "GetOverlayTransformTrackedDeviceRelative",
                (ulOverlayHandle, punTrackedDevice, pmatTrackedDeviceToOverlayTransform),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetOverlayTransformType(
        &mut self,
        ulOverlayHandle: u64,
        peTransformType: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VROverlayTransformType,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("GetOverlayTransformType", (ulOverlayHandle, peTransformType))?;
        Ok(__cordl_ret)
    }
    pub fn GetOverlayWidthInMeters(
        &mut self,
        ulOverlayHandle: u64,
        pfWidthInMeters: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("GetOverlayWidthInMeters", (ulOverlayHandle, pfWidthInMeters))?;
        Ok(__cordl_ret)
    }
    pub fn GetPrimaryDashboardDevice(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("GetPrimaryDashboardDevice", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetTransformForOverlayCoordinates(
        &mut self,
        ulOverlayHandle: u64,
        eTrackingOrigin: crate::OVR::OpenVR::ETrackingUniverseOrigin,
        coordinatesInOverlay: crate::OVR::OpenVR::HmdVector2_t,
        pmatTransform: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdMatrix34_t>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "GetTransformForOverlayCoordinates",
                (ulOverlayHandle, eTrackingOrigin, coordinatesInOverlay, pmatTransform),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HideKeyboard(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HideKeyboard", ())?;
        Ok(__cordl_ret)
    }
    pub fn HideOverlay(
        &mut self,
        ulOverlayHandle: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("HideOverlay", (ulOverlayHandle))?;
        Ok(__cordl_ret)
    }
    pub fn IsActiveDashboardOverlay(
        &mut self,
        ulOverlayHandle: u64,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsActiveDashboardOverlay", (ulOverlayHandle))?;
        Ok(__cordl_ret)
    }
    pub fn IsDashboardVisible(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsDashboardVisible", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsHoverTargetOverlay(
        &mut self,
        ulOverlayHandle: u64,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsHoverTargetOverlay", (ulOverlayHandle))?;
        Ok(__cordl_ret)
    }
    pub fn IsOverlayVisible(
        &mut self,
        ulOverlayHandle: u64,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsOverlayVisible", (ulOverlayHandle))?;
        Ok(__cordl_ret)
    }
    pub fn MoveGamepadFocusToNeighbor(
        &mut self,
        eDirection: crate::OVR::OpenVR::EOverlayDirection,
        ulFrom: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("MoveGamepadFocusToNeighbor", (eDirection, ulFrom))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        pInterface: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pInterface))?;
        Ok(__cordl_object)
    }
    pub fn PollNextOverlayEvent(
        &mut self,
        ulOverlayHandle: u64,
        pEvent: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::VREvent_t>,
        uncbVREvent: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("PollNextOverlayEvent", (ulOverlayHandle, pEvent, uncbVREvent))?;
        Ok(__cordl_ret)
    }
    pub fn ReleaseNativeOverlayHandle(
        &mut self,
        ulOverlayHandle: u64,
        pNativeTextureHandle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "ReleaseNativeOverlayHandle",
                (ulOverlayHandle, pNativeTextureHandle),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SetDashboardOverlaySceneProcess(
        &mut self,
        ulOverlayHandle: u64,
        unProcessId: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("SetDashboardOverlaySceneProcess", (ulOverlayHandle, unProcessId))?;
        Ok(__cordl_ret)
    }
    pub fn SetGamepadFocusOverlay(
        &mut self,
        ulNewFocusOverlay: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("SetGamepadFocusOverlay", (ulNewFocusOverlay))?;
        Ok(__cordl_ret)
    }
    pub fn SetHighQualityOverlay(
        &mut self,
        ulOverlayHandle: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("SetHighQualityOverlay", (ulOverlayHandle))?;
        Ok(__cordl_ret)
    }
    pub fn SetKeyboardPositionForOverlay(
        &mut self,
        ulOverlayHandle: u64,
        avoidRect: crate::OVR::OpenVR::HmdRect2_t,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetKeyboardPositionForOverlay", (ulOverlayHandle, avoidRect))?;
        Ok(__cordl_ret)
    }
    pub fn SetKeyboardTransformAbsolute(
        &mut self,
        eTrackingOrigin: crate::OVR::OpenVR::ETrackingUniverseOrigin,
        pmatTrackingOriginToKeyboardTransform: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetKeyboardTransformAbsolute",
                (eTrackingOrigin, pmatTrackingOriginToKeyboardTransform),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SetOverlayAlpha(
        &mut self,
        ulOverlayHandle: u64,
        fAlpha: f32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("SetOverlayAlpha", (ulOverlayHandle, fAlpha))?;
        Ok(__cordl_ret)
    }
    pub fn SetOverlayAutoCurveDistanceRangeInMeters(
        &mut self,
        ulOverlayHandle: u64,
        fMinDistanceInMeters: f32,
        fMaxDistanceInMeters: f32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "SetOverlayAutoCurveDistanceRangeInMeters",
                (ulOverlayHandle, fMinDistanceInMeters, fMaxDistanceInMeters),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SetOverlayColor(
        &mut self,
        ulOverlayHandle: u64,
        fRed: f32,
        fGreen: f32,
        fBlue: f32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("SetOverlayColor", (ulOverlayHandle, fRed, fGreen, fBlue))?;
        Ok(__cordl_ret)
    }
    pub fn SetOverlayDualAnalogTransform(
        &mut self,
        ulOverlay: u64,
        eWhich: crate::OVR::OpenVR::EDualAnalogWhich,
        vCenter: crate::System::IntPtr,
        fRadius: f32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "SetOverlayDualAnalogTransform",
                (ulOverlay, eWhich, vCenter, fRadius),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SetOverlayFlag(
        &mut self,
        ulOverlayHandle: u64,
        eOverlayFlag: crate::OVR::OpenVR::VROverlayFlags,
        bEnabled: bool,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("SetOverlayFlag", (ulOverlayHandle, eOverlayFlag, bEnabled))?;
        Ok(__cordl_ret)
    }
    pub fn SetOverlayFromFile(
        &mut self,
        ulOverlayHandle: u64,
        pchFilePath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("SetOverlayFromFile", (ulOverlayHandle, pchFilePath))?;
        Ok(__cordl_ret)
    }
    pub fn SetOverlayInputMethod(
        &mut self,
        ulOverlayHandle: u64,
        eInputMethod: crate::OVR::OpenVR::VROverlayInputMethod,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("SetOverlayInputMethod", (ulOverlayHandle, eInputMethod))?;
        Ok(__cordl_ret)
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "SetOverlayIntersectionMask",
                (ulOverlayHandle, pMaskPrimitives, unNumMaskPrimitives, unPrimitiveSize),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SetOverlayMouseScale(
        &mut self,
        ulOverlayHandle: u64,
        pvecMouseScale: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdVector2_t>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("SetOverlayMouseScale", (ulOverlayHandle, pvecMouseScale))?;
        Ok(__cordl_ret)
    }
    pub fn SetOverlayName(
        &mut self,
        ulOverlayHandle: u64,
        pchName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("SetOverlayName", (ulOverlayHandle, pchName))?;
        Ok(__cordl_ret)
    }
    pub fn SetOverlayNeighbor(
        &mut self,
        eDirection: crate::OVR::OpenVR::EOverlayDirection,
        ulFrom: u64,
        ulTo: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("SetOverlayNeighbor", (eDirection, ulFrom, ulTo))?;
        Ok(__cordl_ret)
    }
    pub fn SetOverlayRaw(
        &mut self,
        ulOverlayHandle: u64,
        pvBuffer: crate::System::IntPtr,
        unWidth: u32,
        unHeight: u32,
        unDepth: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "SetOverlayRaw",
                (ulOverlayHandle, pvBuffer, unWidth, unHeight, unDepth),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SetOverlayRenderModel(
        &mut self,
        ulOverlayHandle: u64,
        pchRenderModel: *mut crate::System::String,
        pColor: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdColor_t>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("SetOverlayRenderModel", (ulOverlayHandle, pchRenderModel, pColor))?;
        Ok(__cordl_ret)
    }
    pub fn SetOverlayRenderingPid(
        &mut self,
        ulOverlayHandle: u64,
        unPID: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("SetOverlayRenderingPid", (ulOverlayHandle, unPID))?;
        Ok(__cordl_ret)
    }
    pub fn SetOverlaySortOrder(
        &mut self,
        ulOverlayHandle: u64,
        unSortOrder: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("SetOverlaySortOrder", (ulOverlayHandle, unSortOrder))?;
        Ok(__cordl_ret)
    }
    pub fn SetOverlayTexelAspect(
        &mut self,
        ulOverlayHandle: u64,
        fTexelAspect: f32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("SetOverlayTexelAspect", (ulOverlayHandle, fTexelAspect))?;
        Ok(__cordl_ret)
    }
    pub fn SetOverlayTexture(
        &mut self,
        ulOverlayHandle: u64,
        pTexture: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::Texture_t>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("SetOverlayTexture", (ulOverlayHandle, pTexture))?;
        Ok(__cordl_ret)
    }
    pub fn SetOverlayTextureBounds(
        &mut self,
        ulOverlayHandle: u64,
        pOverlayTextureBounds: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VRTextureBounds_t,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "SetOverlayTextureBounds",
                (ulOverlayHandle, pOverlayTextureBounds),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SetOverlayTextureColorSpace(
        &mut self,
        ulOverlayHandle: u64,
        eTextureColorSpace: crate::OVR::OpenVR::EColorSpace,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "SetOverlayTextureColorSpace",
                (ulOverlayHandle, eTextureColorSpace),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SetOverlayTransformAbsolute(
        &mut self,
        ulOverlayHandle: u64,
        eTrackingOrigin: crate::OVR::OpenVR::ETrackingUniverseOrigin,
        pmatTrackingOriginToOverlayTransform: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "SetOverlayTransformAbsolute",
                (ulOverlayHandle, eTrackingOrigin, pmatTrackingOriginToOverlayTransform),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SetOverlayTransformOverlayRelative(
        &mut self,
        ulOverlayHandle: u64,
        ulOverlayHandleParent: u64,
        pmatParentOverlayToOverlayTransform: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "SetOverlayTransformOverlayRelative",
                (
                    ulOverlayHandle,
                    ulOverlayHandleParent,
                    pmatParentOverlayToOverlayTransform,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SetOverlayTransformTrackedDeviceComponent(
        &mut self,
        ulOverlayHandle: u64,
        unDeviceIndex: u32,
        pchComponentName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "SetOverlayTransformTrackedDeviceComponent",
                (ulOverlayHandle, unDeviceIndex, pchComponentName),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SetOverlayTransformTrackedDeviceRelative(
        &mut self,
        ulOverlayHandle: u64,
        unTrackedDevice: u32,
        pmatTrackedDeviceToOverlayTransform: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "SetOverlayTransformTrackedDeviceRelative",
                (ulOverlayHandle, unTrackedDevice, pmatTrackedDeviceToOverlayTransform),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SetOverlayWidthInMeters(
        &mut self,
        ulOverlayHandle: u64,
        fWidthInMeters: f32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("SetOverlayWidthInMeters", (ulOverlayHandle, fWidthInMeters))?;
        Ok(__cordl_ret)
    }
    pub fn ShowDashboard(
        &mut self,
        pchOverlayToShow: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowDashboard", (pchOverlayToShow))?;
        Ok(__cordl_ret)
    }
    pub fn ShowKeyboard(
        &mut self,
        eInputMode: i32,
        eLineInputMode: i32,
        pchDescription: *mut crate::System::String,
        unCharMax: u32,
        pchExistingText: *mut crate::System::String,
        bUseMinimalMode: bool,
        uUserValue: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "ShowKeyboard",
                (
                    eInputMode,
                    eLineInputMode,
                    pchDescription,
                    unCharMax,
                    pchExistingText,
                    bUseMinimalMode,
                    uUserValue,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ShowKeyboardForOverlay(
        &mut self,
        ulOverlayHandle: u64,
        eInputMode: i32,
        eLineInputMode: i32,
        pchDescription: *mut crate::System::String,
        unCharMax: u32,
        pchExistingText: *mut crate::System::String,
        bUseMinimalMode: bool,
        uUserValue: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "ShowKeyboardForOverlay",
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
            )?;
        Ok(__cordl_ret)
    }
    pub fn ShowMessageOverlay(
        &mut self,
        pchText: *mut crate::System::String,
        pchCaption: *mut crate::System::String,
        pchButton0Text: *mut crate::System::String,
        pchButton1Text: *mut crate::System::String,
        pchButton2Text: *mut crate::System::String,
        pchButton3Text: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::VRMessageOverlayResponse> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::VRMessageOverlayResponse = __cordl_object
            .invoke(
                "ShowMessageOverlay",
                (
                    pchText,
                    pchCaption,
                    pchButton0Text,
                    pchButton1Text,
                    pchButton2Text,
                    pchButton3Text,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ShowOverlay(
        &mut self,
        ulOverlayHandle: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("ShowOverlay", (ulOverlayHandle))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        pInterface: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (pInterface))?;
        Ok(__cordl_ret)
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
#[derive(Debug, Clone)]
pub struct CVROverlay_PollNextOverlayEventUnion {
    padding: [u8; 8usize],
}
#[cfg(feature = "OVR+OpenVR+CVROverlay+PollNextOverlayEventUnion")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::CVROverlay_PollNextOverlayEventUnion => "OVR.OpenVR"
    ."CVROverlay/PollNextOverlayEventUnion"
);
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::CVROverlay__PollNextOverlayEventPacked => "OVR.OpenVR"
    ."CVROverlay/_PollNextOverlayEventPacked"
);
#[cfg(feature = "OVR+OpenVR+CVROverlay+_PollNextOverlayEventPacked")]
impl std::ops::Deref for crate::OVR::OpenVR::CVROverlay__PollNextOverlayEventPacked {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVROverlay+_PollNextOverlayEventPacked")]
impl std::ops::DerefMut for crate::OVR::OpenVR::CVROverlay__PollNextOverlayEventPacked {
    fn deref_mut(&mut self) -> &mut Self::Target {
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
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke(
                "BeginInvoke",
                (ulOverlayHandle, pEvent, uncbVREvent, callback, object),
            )?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        pEvent: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::VREvent_t_Packed>,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EndInvoke", (pEvent, result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        pEvent: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::VREvent_t_Packed>,
        uncbVREvent: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Invoke", (ulOverlayHandle, pEvent, uncbVREvent))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
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
