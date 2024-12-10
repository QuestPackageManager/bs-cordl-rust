#[cfg(feature = "OVR+OpenVR+IVRScreenshots")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct IVRScreenshots {
    pub RequestScreenshot: *mut crate::OVR::OpenVR::IVRScreenshots__RequestScreenshot,
    pub HookScreenshot: *mut crate::OVR::OpenVR::IVRScreenshots__HookScreenshot,
    pub GetScreenshotPropertyType: *mut crate::OVR::OpenVR::IVRScreenshots__GetScreenshotPropertyType,
    pub GetScreenshotPropertyFilename: *mut crate::OVR::OpenVR::IVRScreenshots__GetScreenshotPropertyFilename,
    pub UpdateScreenshotProgress: *mut crate::OVR::OpenVR::IVRScreenshots__UpdateScreenshotProgress,
    pub TakeStereoScreenshot: *mut crate::OVR::OpenVR::IVRScreenshots__TakeStereoScreenshot,
    pub SubmitScreenshot: *mut crate::OVR::OpenVR::IVRScreenshots__SubmitScreenshot,
}
#[cfg(feature = "OVR+OpenVR+IVRScreenshots")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRScreenshots => "OVR.OpenVR"
    ."IVRScreenshots"
);
#[cfg(feature = "OVR+OpenVR+IVRScreenshots")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::OVR::OpenVR::IVRScreenshots {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRScreenshots")]
impl crate::OVR::OpenVR::IVRScreenshots {
    #[cfg(feature = "OVR+OpenVR+IVRScreenshots+_GetScreenshotPropertyFilename")]
    pub type _GetScreenshotPropertyFilename = crate::OVR::OpenVR::IVRScreenshots__GetScreenshotPropertyFilename;
    #[cfg(feature = "OVR+OpenVR+IVRScreenshots+_GetScreenshotPropertyType")]
    pub type _GetScreenshotPropertyType = crate::OVR::OpenVR::IVRScreenshots__GetScreenshotPropertyType;
    #[cfg(feature = "OVR+OpenVR+IVRScreenshots+_HookScreenshot")]
    pub type _HookScreenshot = crate::OVR::OpenVR::IVRScreenshots__HookScreenshot;
    #[cfg(feature = "OVR+OpenVR+IVRScreenshots+_RequestScreenshot")]
    pub type _RequestScreenshot = crate::OVR::OpenVR::IVRScreenshots__RequestScreenshot;
    #[cfg(feature = "OVR+OpenVR+IVRScreenshots+_SubmitScreenshot")]
    pub type _SubmitScreenshot = crate::OVR::OpenVR::IVRScreenshots__SubmitScreenshot;
    #[cfg(feature = "OVR+OpenVR+IVRScreenshots+_TakeStereoScreenshot")]
    pub type _TakeStereoScreenshot = crate::OVR::OpenVR::IVRScreenshots__TakeStereoScreenshot;
    #[cfg(feature = "OVR+OpenVR+IVRScreenshots+_UpdateScreenshotProgress")]
    pub type _UpdateScreenshotProgress = crate::OVR::OpenVR::IVRScreenshots__UpdateScreenshotProgress;
}
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_GetScreenshotPropertyFilename")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRScreenshots__GetScreenshotPropertyFilename {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_GetScreenshotPropertyFilename")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRScreenshots__GetScreenshotPropertyFilename => "OVR.OpenVR"
    ."IVRScreenshots/_GetScreenshotPropertyFilename"
);
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_GetScreenshotPropertyFilename")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRScreenshots__GetScreenshotPropertyFilename {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_GetScreenshotPropertyFilename")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRScreenshots__GetScreenshotPropertyFilename {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_GetScreenshotPropertyFilename")]
impl crate::OVR::OpenVR::IVRScreenshots__GetScreenshotPropertyFilename {
    pub fn BeginInvoke(
        &mut self,
        screenshotHandle: u32,
        filenameType: crate::OVR::OpenVR::EVRScreenshotPropertyFilenames,
        pchFilename: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        cchFilename: u32,
        pError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRScreenshotError>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke(
                "BeginInvoke",
                (
                    screenshotHandle,
                    filenameType,
                    pchFilename,
                    cchFilename,
                    pError,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRScreenshotError>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("EndInvoke", (pError, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        screenshotHandle: u32,
        filenameType: crate::OVR::OpenVR::EVRScreenshotPropertyFilenames,
        pchFilename: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        cchFilename: u32,
        pError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRScreenshotError>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke(
                "Invoke",
                (screenshotHandle, filenameType, pchFilename, cchFilename, pError),
            )?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_GetScreenshotPropertyFilename")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRScreenshots__GetScreenshotPropertyFilename {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_GetScreenshotPropertyType")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRScreenshots__GetScreenshotPropertyType {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_GetScreenshotPropertyType")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRScreenshots__GetScreenshotPropertyType => "OVR.OpenVR"
    ."IVRScreenshots/_GetScreenshotPropertyType"
);
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_GetScreenshotPropertyType")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRScreenshots__GetScreenshotPropertyType {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_GetScreenshotPropertyType")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRScreenshots__GetScreenshotPropertyType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_GetScreenshotPropertyType")]
impl crate::OVR::OpenVR::IVRScreenshots__GetScreenshotPropertyType {
    pub fn BeginInvoke(
        &mut self,
        screenshotHandle: u32,
        pError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRScreenshotError>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (screenshotHandle, pError, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRScreenshotError>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRScreenshotType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRScreenshotType = __cordl_object
            .invoke("EndInvoke", (pError, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        screenshotHandle: u32,
        pError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRScreenshotError>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRScreenshotType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRScreenshotType = __cordl_object
            .invoke("Invoke", (screenshotHandle, pError))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_GetScreenshotPropertyType")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRScreenshots__GetScreenshotPropertyType {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_HookScreenshot")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRScreenshots__HookScreenshot {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_HookScreenshot")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRScreenshots__HookScreenshot =>
    "OVR.OpenVR"."IVRScreenshots/_HookScreenshot"
);
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_HookScreenshot")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRScreenshots__HookScreenshot {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_HookScreenshot")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRScreenshots__HookScreenshot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_HookScreenshot")]
impl crate::OVR::OpenVR::IVRScreenshots__HookScreenshot {
    pub fn BeginInvoke(
        &mut self,
        pSupportedTypes: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                crate::OVR::OpenVR::EVRScreenshotType,
            >,
        >,
        numTypes: i32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (pSupportedTypes, numTypes, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRScreenshotError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRScreenshotError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pSupportedTypes: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                crate::OVR::OpenVR::EVRScreenshotType,
            >,
        >,
        numTypes: i32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRScreenshotError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRScreenshotError = __cordl_object
            .invoke("Invoke", (pSupportedTypes, numTypes))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_HookScreenshot")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRScreenshots__HookScreenshot {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_RequestScreenshot")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRScreenshots__RequestScreenshot {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_RequestScreenshot")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRScreenshots__RequestScreenshot
    => "OVR.OpenVR"."IVRScreenshots/_RequestScreenshot"
);
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_RequestScreenshot")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRScreenshots__RequestScreenshot {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_RequestScreenshot")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRScreenshots__RequestScreenshot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_RequestScreenshot")]
impl crate::OVR::OpenVR::IVRScreenshots__RequestScreenshot {
    pub fn BeginInvoke(
        &mut self,
        pOutScreenshotHandle: quest_hook::libil2cpp::ByRefMut<u32>,
        _cordl_type: crate::OVR::OpenVR::EVRScreenshotType,
        pchPreviewFilename: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        pchVRFilename: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke(
                "BeginInvoke",
                (
                    pOutScreenshotHandle,
                    _cordl_type,
                    pchPreviewFilename,
                    pchVRFilename,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pOutScreenshotHandle: quest_hook::libil2cpp::ByRefMut<u32>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRScreenshotError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRScreenshotError = __cordl_object
            .invoke("EndInvoke", (pOutScreenshotHandle, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pOutScreenshotHandle: quest_hook::libil2cpp::ByRefMut<u32>,
        _cordl_type: crate::OVR::OpenVR::EVRScreenshotType,
        pchPreviewFilename: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        pchVRFilename: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRScreenshotError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRScreenshotError = __cordl_object
            .invoke(
                "Invoke",
                (pOutScreenshotHandle, _cordl_type, pchPreviewFilename, pchVRFilename),
            )?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_RequestScreenshot")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRScreenshots__RequestScreenshot {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_SubmitScreenshot")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRScreenshots__SubmitScreenshot {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_SubmitScreenshot")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRScreenshots__SubmitScreenshot =>
    "OVR.OpenVR"."IVRScreenshots/_SubmitScreenshot"
);
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_SubmitScreenshot")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRScreenshots__SubmitScreenshot {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_SubmitScreenshot")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRScreenshots__SubmitScreenshot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_SubmitScreenshot")]
impl crate::OVR::OpenVR::IVRScreenshots__SubmitScreenshot {
    pub fn BeginInvoke(
        &mut self,
        screenshotHandle: u32,
        _cordl_type: crate::OVR::OpenVR::EVRScreenshotType,
        pchSourcePreviewFilename: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        pchSourceVRFilename: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke(
                "BeginInvoke",
                (
                    screenshotHandle,
                    _cordl_type,
                    pchSourcePreviewFilename,
                    pchSourceVRFilename,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRScreenshotError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRScreenshotError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRScreenshotError = __cordl_object
            .invoke(
                "Invoke",
                (
                    screenshotHandle,
                    _cordl_type,
                    pchSourcePreviewFilename,
                    pchSourceVRFilename,
                ),
            )?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_SubmitScreenshot")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRScreenshots__SubmitScreenshot {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_TakeStereoScreenshot")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRScreenshots__TakeStereoScreenshot {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_TakeStereoScreenshot")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRScreenshots__TakeStereoScreenshot => "OVR.OpenVR"
    ."IVRScreenshots/_TakeStereoScreenshot"
);
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_TakeStereoScreenshot")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRScreenshots__TakeStereoScreenshot {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_TakeStereoScreenshot")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRScreenshots__TakeStereoScreenshot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_TakeStereoScreenshot")]
impl crate::OVR::OpenVR::IVRScreenshots__TakeStereoScreenshot {
    pub fn BeginInvoke(
        &mut self,
        pOutScreenshotHandle: quest_hook::libil2cpp::ByRefMut<u32>,
        pchPreviewFilename: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        pchVRFilename: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke(
                "BeginInvoke",
                (
                    pOutScreenshotHandle,
                    pchPreviewFilename,
                    pchVRFilename,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pOutScreenshotHandle: quest_hook::libil2cpp::ByRefMut<u32>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRScreenshotError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRScreenshotError = __cordl_object
            .invoke("EndInvoke", (pOutScreenshotHandle, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pOutScreenshotHandle: quest_hook::libil2cpp::ByRefMut<u32>,
        pchPreviewFilename: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        pchVRFilename: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRScreenshotError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRScreenshotError = __cordl_object
            .invoke(
                "Invoke",
                (pOutScreenshotHandle, pchPreviewFilename, pchVRFilename),
            )?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_TakeStereoScreenshot")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRScreenshots__TakeStereoScreenshot {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_UpdateScreenshotProgress")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRScreenshots__UpdateScreenshotProgress {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_UpdateScreenshotProgress")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRScreenshots__UpdateScreenshotProgress => "OVR.OpenVR"
    ."IVRScreenshots/_UpdateScreenshotProgress"
);
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_UpdateScreenshotProgress")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRScreenshots__UpdateScreenshotProgress {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_UpdateScreenshotProgress")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRScreenshots__UpdateScreenshotProgress {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_UpdateScreenshotProgress")]
impl crate::OVR::OpenVR::IVRScreenshots__UpdateScreenshotProgress {
    pub fn BeginInvoke(
        &mut self,
        screenshotHandle: u32,
        flProgress: f32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (screenshotHandle, flProgress, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRScreenshotError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRScreenshotError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        screenshotHandle: u32,
        flProgress: f32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRScreenshotError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRScreenshotError = __cordl_object
            .invoke("Invoke", (screenshotHandle, flProgress))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRScreenshots+_UpdateScreenshotProgress")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRScreenshots__UpdateScreenshotProgress {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
