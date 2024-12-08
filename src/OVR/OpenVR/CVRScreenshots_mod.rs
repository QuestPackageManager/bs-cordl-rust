#[cfg(feature = "OVR+OpenVR+CVRScreenshots")]
#[repr(C)]
#[derive(Debug)]
pub struct CVRScreenshots {
    __cordl_parent: crate::System::Object,
    pub FnTable: crate::OVR::OpenVR::IVRScreenshots,
}
#[cfg(feature = "OVR+OpenVR+CVRScreenshots")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::CVRScreenshots => "OVR.OpenVR"
    ."CVRScreenshots"
);
#[cfg(feature = "OVR+OpenVR+CVRScreenshots")]
impl std::ops::Deref for crate::OVR::OpenVR::CVRScreenshots {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRScreenshots")]
impl std::ops::DerefMut for crate::OVR::OpenVR::CVRScreenshots {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRScreenshots")]
impl crate::OVR::OpenVR::CVRScreenshots {
    pub fn RequestScreenshot(
        &mut self,
        pOutScreenshotHandle: quest_hook::libil2cpp::ByRefMut<u32>,
        _cordl_type: crate::OVR::OpenVR::EVRScreenshotType,
        pchPreviewFilename: *mut crate::System::String,
        pchVRFilename: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRScreenshotError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRScreenshotError = __cordl_object
            .invoke(
                "RequestScreenshot",
                (pOutScreenshotHandle, _cordl_type, pchPreviewFilename, pchVRFilename),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetScreenshotPropertyType(
        &mut self,
        screenshotHandle: u32,
        pError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRScreenshotError>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRScreenshotType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRScreenshotType = __cordl_object
            .invoke("GetScreenshotPropertyType", (screenshotHandle, pError))?;
        Ok(__cordl_ret)
    }
    pub fn TakeStereoScreenshot(
        &mut self,
        pOutScreenshotHandle: quest_hook::libil2cpp::ByRefMut<u32>,
        pchPreviewFilename: *mut crate::System::String,
        pchVRFilename: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRScreenshotError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRScreenshotError = __cordl_object
            .invoke(
                "TakeStereoScreenshot",
                (pOutScreenshotHandle, pchPreviewFilename, pchVRFilename),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HookScreenshot(
        &mut self,
        pSupportedTypes: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::OVR::OpenVR::EVRScreenshotType,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRScreenshotError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRScreenshotError = __cordl_object
            .invoke("HookScreenshot", (pSupportedTypes))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateScreenshotProgress(
        &mut self,
        screenshotHandle: u32,
        flProgress: f32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRScreenshotError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRScreenshotError = __cordl_object
            .invoke("UpdateScreenshotProgress", (screenshotHandle, flProgress))?;
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
    pub fn SubmitScreenshot(
        &mut self,
        screenshotHandle: u32,
        _cordl_type: crate::OVR::OpenVR::EVRScreenshotType,
        pchSourcePreviewFilename: *mut crate::System::String,
        pchSourceVRFilename: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRScreenshotError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRScreenshotError = __cordl_object
            .invoke(
                "SubmitScreenshot",
                (
                    screenshotHandle,
                    _cordl_type,
                    pchSourcePreviewFilename,
                    pchSourceVRFilename,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetScreenshotPropertyFilename(
        &mut self,
        screenshotHandle: u32,
        filenameType: crate::OVR::OpenVR::EVRScreenshotPropertyFilenames,
        pchFilename: *mut crate::System::Text::StringBuilder,
        cchFilename: u32,
        pError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRScreenshotError>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke(
                "GetScreenshotPropertyFilename",
                (screenshotHandle, filenameType, pchFilename, cchFilename, pError),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New(
        pInterface: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pInterface))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "OVR+OpenVR+CVRScreenshots")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::CVRScreenshots {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
