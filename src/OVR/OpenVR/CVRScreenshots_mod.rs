#[cfg(feature = "OVR+OpenVR+CVRScreenshots")]
#[repr(C)]
#[derive(Debug)]
pub struct CVRScreenshots {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub FnTable: crate::OVR::OpenVR::IVRScreenshots,
}
#[cfg(feature = "OVR+OpenVR+CVRScreenshots")]
unsafe impl quest_hook::libil2cpp::Type for crate::OVR::OpenVR::CVRScreenshots {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "CVRScreenshots";
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
#[cfg(feature = "OVR+OpenVR+CVRScreenshots")]
impl std::ops::Deref for crate::OVR::OpenVR::CVRScreenshots {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn GetScreenshotPropertyFilename(
        &mut self,
        screenshotHandle: u32,
        filenameType: crate::OVR::OpenVR::EVRScreenshotPropertyFilenames,
        pchFilename: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        cchFilename: u32,
        pError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRScreenshotError>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRScreenshots as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    u32,
                    crate::OVR::OpenVR::EVRScreenshotPropertyFilenames,
                    quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
                    u32,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::OVR::OpenVR::EVRScreenshotError,
                    >,
                ),
                u32,
                5usize,
            >("GetScreenshotPropertyFilename")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRScreenshots as quest_hook::libil2cpp::Type >
                    ::class(), "GetScreenshotPropertyFilename", 5usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (screenshotHandle, filenameType, pchFilename, cchFilename, pError),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetScreenshotPropertyType(
        &mut self,
        screenshotHandle: u32,
        pError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRScreenshotError>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRScreenshotType> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRScreenshots as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    u32,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::OVR::OpenVR::EVRScreenshotError,
                    >,
                ),
                crate::OVR::OpenVR::EVRScreenshotType,
                2usize,
            >("GetScreenshotPropertyType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRScreenshots as quest_hook::libil2cpp::Type >
                    ::class(), "GetScreenshotPropertyType", 2usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRScreenshotType = unsafe {
            method.invoke_unchecked(self, (screenshotHandle, pError))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HookScreenshot(
        &mut self,
        pSupportedTypes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::EVRScreenshotType>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRScreenshotError> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRScreenshots as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        crate::OVR::OpenVR::EVRScreenshotType,
                    >,
                >),
                crate::OVR::OpenVR::EVRScreenshotError,
                1usize,
            >("HookScreenshot")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRScreenshots as quest_hook::libil2cpp::Type >
                    ::class(), "HookScreenshot", 1usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRScreenshotError = unsafe {
            method.invoke_unchecked(self, (pSupportedTypes))?
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
    pub fn RequestScreenshot(
        &mut self,
        pOutScreenshotHandle: quest_hook::libil2cpp::ByRefMut<u32>,
        _cordl_type: crate::OVR::OpenVR::EVRScreenshotType,
        pchPreviewFilename: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        pchVRFilename: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRScreenshotError> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRScreenshots as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<u32>,
                    crate::OVR::OpenVR::EVRScreenshotType,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                crate::OVR::OpenVR::EVRScreenshotError,
                4usize,
            >("RequestScreenshot")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRScreenshots as quest_hook::libil2cpp::Type >
                    ::class(), "RequestScreenshot", 4usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRScreenshotError = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        pOutScreenshotHandle,
                        _cordl_type,
                        pchPreviewFilename,
                        pchVRFilename,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SubmitScreenshot(
        &mut self,
        screenshotHandle: u32,
        _cordl_type: crate::OVR::OpenVR::EVRScreenshotType,
        pchSourcePreviewFilename: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        pchSourceVRFilename: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRScreenshotError> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRScreenshots as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    u32,
                    crate::OVR::OpenVR::EVRScreenshotType,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                crate::OVR::OpenVR::EVRScreenshotError,
                4usize,
            >("SubmitScreenshot")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRScreenshots as quest_hook::libil2cpp::Type >
                    ::class(), "SubmitScreenshot", 4usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRScreenshotError = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        screenshotHandle,
                        _cordl_type,
                        pchSourcePreviewFilename,
                        pchSourceVRFilename,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TakeStereoScreenshot(
        &mut self,
        pOutScreenshotHandle: quest_hook::libil2cpp::ByRefMut<u32>,
        pchPreviewFilename: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        pchVRFilename: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRScreenshotError> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRScreenshots as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<u32>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                crate::OVR::OpenVR::EVRScreenshotError,
                3usize,
            >("TakeStereoScreenshot")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRScreenshots as quest_hook::libil2cpp::Type >
                    ::class(), "TakeStereoScreenshot", 3usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRScreenshotError = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (pOutScreenshotHandle, pchPreviewFilename, pchVRFilename),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateScreenshotProgress(
        &mut self,
        screenshotHandle: u32,
        flProgress: f32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRScreenshotError> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRScreenshots as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (u32, f32),
                crate::OVR::OpenVR::EVRScreenshotError,
                2usize,
            >("UpdateScreenshotProgress")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRScreenshots as quest_hook::libil2cpp::Type >
                    ::class(), "UpdateScreenshotProgress", 2usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRScreenshotError = unsafe {
            method.invoke_unchecked(self, (screenshotHandle, flProgress))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        pInterface: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::CVRScreenshots as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::CVRScreenshots as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (pInterface))?
        };
        Ok(__cordl_ret.into())
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
