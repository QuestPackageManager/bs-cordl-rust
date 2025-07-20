#[cfg(feature = "OVR+OpenVR+IVRScreenshots")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct IVRScreenshots {
    pub RequestScreenshot: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRScreenshots__RequestScreenshot,
    >,
    pub HookScreenshot: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRScreenshots__HookScreenshot,
    >,
    pub GetScreenshotPropertyType: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRScreenshots__GetScreenshotPropertyType,
    >,
    pub GetScreenshotPropertyFilename: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRScreenshots__GetScreenshotPropertyFilename,
    >,
    pub UpdateScreenshotProgress: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRScreenshots__UpdateScreenshotProgress,
    >,
    pub TakeStereoScreenshot: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRScreenshots__TakeStereoScreenshot,
    >,
    pub SubmitScreenshot: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRScreenshots__SubmitScreenshot,
    >,
}
#[cfg(feature = "OVR+OpenVR+IVRScreenshots")]
unsafe impl quest_hook::libil2cpp::Type for crate::OVR::OpenVR::IVRScreenshots {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "IVRScreenshots";
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
#[cfg(feature = "OVR+OpenVR+IVRScreenshots")]
unsafe impl quest_hook::libil2cpp::Argument for crate::OVR::OpenVR::IVRScreenshots {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVR+OpenVR+IVRScreenshots")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::OVR::OpenVR::IVRScreenshots {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "OVR+OpenVR+IVRScreenshots")]
unsafe impl quest_hook::libil2cpp::Returned for crate::OVR::OpenVR::IVRScreenshots {
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
#[cfg(feature = "OVR+OpenVR+IVRScreenshots")]
unsafe impl quest_hook::libil2cpp::Return for crate::OVR::OpenVR::IVRScreenshots {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVRScreenshots__GetScreenshotPropertyFilename {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "IVRScreenshots/_GetScreenshotPropertyFilename";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRScreenshots__GetScreenshotPropertyFilename as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    u32,
                    crate::OVR::OpenVR::EVRScreenshotPropertyFilenames,
                    quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
                    u32,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::OVR::OpenVR::EVRScreenshotError,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                7usize,
            >("BeginInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRScreenshots__GetScreenshotPropertyFilename as
                    quest_hook::libil2cpp::Type > ::class(), "BeginInvoke", 7usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        screenshotHandle,
                        filenameType,
                        pchFilename,
                        cchFilename,
                        pError,
                        callback,
                        object,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRScreenshotError>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRScreenshots__GetScreenshotPropertyFilename as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::OVR::OpenVR::EVRScreenshotError,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                ),
                u32,
                2usize,
            >("EndInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRScreenshots__GetScreenshotPropertyFilename as
                    quest_hook::libil2cpp::Type > ::class(), "EndInvoke", 2usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked(self, (pError, result))?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRScreenshots__GetScreenshotPropertyFilename as quest_hook::libil2cpp::Type>::class()
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
            >("Invoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRScreenshots__GetScreenshotPropertyFilename as
                    quest_hook::libil2cpp::Type > ::class(), "Invoke", 5usize
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRScreenshots__GetScreenshotPropertyFilename as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::System::IntPtr,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRScreenshots__GetScreenshotPropertyFilename as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
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
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVRScreenshots__GetScreenshotPropertyType {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "IVRScreenshots/_GetScreenshotPropertyType";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRScreenshots__GetScreenshotPropertyType as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    u32,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::OVR::OpenVR::EVRScreenshotError,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                4usize,
            >("BeginInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRScreenshots__GetScreenshotPropertyType as
                    quest_hook::libil2cpp::Type > ::class(), "BeginInvoke", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            method.invoke_unchecked(self, (screenshotHandle, pError, callback, object))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRScreenshotError>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRScreenshotType> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRScreenshots__GetScreenshotPropertyType as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::OVR::OpenVR::EVRScreenshotError,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                ),
                crate::OVR::OpenVR::EVRScreenshotType,
                2usize,
            >("EndInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRScreenshots__GetScreenshotPropertyType as
                    quest_hook::libil2cpp::Type > ::class(), "EndInvoke", 2usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRScreenshotType = unsafe {
            method.invoke_unchecked(self, (pError, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        screenshotHandle: u32,
        pError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRScreenshotError>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRScreenshotType> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRScreenshots__GetScreenshotPropertyType as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    u32,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::OVR::OpenVR::EVRScreenshotError,
                    >,
                ),
                crate::OVR::OpenVR::EVRScreenshotType,
                2usize,
            >("Invoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRScreenshots__GetScreenshotPropertyType as
                    quest_hook::libil2cpp::Type > ::class(), "Invoke", 2usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRScreenshotType = unsafe {
            method.invoke_unchecked(self, (screenshotHandle, pError))?
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRScreenshots__GetScreenshotPropertyType as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::System::IntPtr,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRScreenshots__GetScreenshotPropertyType as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
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
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVRScreenshots__HookScreenshot {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "IVRScreenshots/_HookScreenshot";
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
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::EVRScreenshotType>,
            >,
        >,
        numTypes: i32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRScreenshots__HookScreenshot as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::OVR::OpenVR::EVRScreenshotType,
                            >,
                        >,
                    >,
                    i32,
                    quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                4usize,
            >("BeginInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRScreenshots__HookScreenshot as
                    quest_hook::libil2cpp::Type > ::class(), "BeginInvoke", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            method.invoke_unchecked(self, (pSupportedTypes, numTypes, callback, object))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRScreenshotError> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRScreenshots__HookScreenshot as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>),
                crate::OVR::OpenVR::EVRScreenshotError,
                1usize,
            >("EndInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRScreenshots__HookScreenshot as
                    quest_hook::libil2cpp::Type > ::class(), "EndInvoke", 1usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRScreenshotError = unsafe {
            method.invoke_unchecked(self, (result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pSupportedTypes: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::EVRScreenshotType>,
            >,
        >,
        numTypes: i32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRScreenshotError> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRScreenshots__HookScreenshot as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::OVR::OpenVR::EVRScreenshotType,
                            >,
                        >,
                    >,
                    i32,
                ),
                crate::OVR::OpenVR::EVRScreenshotError,
                2usize,
            >("Invoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRScreenshots__HookScreenshot as
                    quest_hook::libil2cpp::Type > ::class(), "Invoke", 2usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRScreenshotError = unsafe {
            method.invoke_unchecked(self, (pSupportedTypes, numTypes))?
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRScreenshots__HookScreenshot as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::System::IntPtr,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRScreenshots__HookScreenshot as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
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
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVRScreenshots__RequestScreenshot {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "IVRScreenshots/_RequestScreenshot";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRScreenshots__RequestScreenshot as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<u32>,
                    crate::OVR::OpenVR::EVRScreenshotType,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                6usize,
            >("BeginInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRScreenshots__RequestScreenshot as
                    quest_hook::libil2cpp::Type > ::class(), "BeginInvoke", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        pOutScreenshotHandle,
                        _cordl_type,
                        pchPreviewFilename,
                        pchVRFilename,
                        callback,
                        object,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pOutScreenshotHandle: quest_hook::libil2cpp::ByRefMut<u32>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRScreenshotError> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRScreenshots__RequestScreenshot as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<u32>,
                    quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                ),
                crate::OVR::OpenVR::EVRScreenshotError,
                2usize,
            >("EndInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRScreenshots__RequestScreenshot as
                    quest_hook::libil2cpp::Type > ::class(), "EndInvoke", 2usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRScreenshotError = unsafe {
            method.invoke_unchecked(self, (pOutScreenshotHandle, result))?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRScreenshots__RequestScreenshot as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<u32>,
                    crate::OVR::OpenVR::EVRScreenshotType,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                crate::OVR::OpenVR::EVRScreenshotError,
                4usize,
            >("Invoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRScreenshots__RequestScreenshot as
                    quest_hook::libil2cpp::Type > ::class(), "Invoke", 4usize
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRScreenshots__RequestScreenshot as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::System::IntPtr,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRScreenshots__RequestScreenshot as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
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
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVRScreenshots__SubmitScreenshot {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "IVRScreenshots/_SubmitScreenshot";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRScreenshots__SubmitScreenshot as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    u32,
                    crate::OVR::OpenVR::EVRScreenshotType,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                6usize,
            >("BeginInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRScreenshots__SubmitScreenshot as
                    quest_hook::libil2cpp::Type > ::class(), "BeginInvoke", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        screenshotHandle,
                        _cordl_type,
                        pchSourcePreviewFilename,
                        pchSourceVRFilename,
                        callback,
                        object,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRScreenshotError> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRScreenshots__SubmitScreenshot as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>),
                crate::OVR::OpenVR::EVRScreenshotError,
                1usize,
            >("EndInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRScreenshots__SubmitScreenshot as
                    quest_hook::libil2cpp::Type > ::class(), "EndInvoke", 1usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRScreenshotError = unsafe {
            method.invoke_unchecked(self, (result))?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRScreenshots__SubmitScreenshot as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    u32,
                    crate::OVR::OpenVR::EVRScreenshotType,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                crate::OVR::OpenVR::EVRScreenshotError,
                4usize,
            >("Invoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRScreenshots__SubmitScreenshot as
                    quest_hook::libil2cpp::Type > ::class(), "Invoke", 4usize
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRScreenshots__SubmitScreenshot as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::System::IntPtr,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRScreenshots__SubmitScreenshot as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
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
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVRScreenshots__TakeStereoScreenshot {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "IVRScreenshots/_TakeStereoScreenshot";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRScreenshots__TakeStereoScreenshot as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<u32>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                5usize,
            >("BeginInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRScreenshots__TakeStereoScreenshot as
                    quest_hook::libil2cpp::Type > ::class(), "BeginInvoke", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        pOutScreenshotHandle,
                        pchPreviewFilename,
                        pchVRFilename,
                        callback,
                        object,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pOutScreenshotHandle: quest_hook::libil2cpp::ByRefMut<u32>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRScreenshotError> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRScreenshots__TakeStereoScreenshot as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<u32>,
                    quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                ),
                crate::OVR::OpenVR::EVRScreenshotError,
                2usize,
            >("EndInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRScreenshots__TakeStereoScreenshot as
                    quest_hook::libil2cpp::Type > ::class(), "EndInvoke", 2usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRScreenshotError = unsafe {
            method.invoke_unchecked(self, (pOutScreenshotHandle, result))?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRScreenshots__TakeStereoScreenshot as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<u32>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                crate::OVR::OpenVR::EVRScreenshotError,
                3usize,
            >("Invoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRScreenshots__TakeStereoScreenshot as
                    quest_hook::libil2cpp::Type > ::class(), "Invoke", 3usize
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRScreenshots__TakeStereoScreenshot as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::System::IntPtr,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRScreenshots__TakeStereoScreenshot as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
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
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVRScreenshots__UpdateScreenshotProgress {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "IVRScreenshots/_UpdateScreenshotProgress";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRScreenshots__UpdateScreenshotProgress as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    u32,
                    f32,
                    quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                4usize,
            >("BeginInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRScreenshots__UpdateScreenshotProgress as
                    quest_hook::libil2cpp::Type > ::class(), "BeginInvoke", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (screenshotHandle, flProgress, callback, object),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRScreenshotError> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRScreenshots__UpdateScreenshotProgress as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>),
                crate::OVR::OpenVR::EVRScreenshotError,
                1usize,
            >("EndInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRScreenshots__UpdateScreenshotProgress as
                    quest_hook::libil2cpp::Type > ::class(), "EndInvoke", 1usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRScreenshotError = unsafe {
            method.invoke_unchecked(self, (result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        screenshotHandle: u32,
        flProgress: f32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRScreenshotError> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRScreenshots__UpdateScreenshotProgress as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (u32, f32),
                crate::OVR::OpenVR::EVRScreenshotError,
                2usize,
            >("Invoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRScreenshots__UpdateScreenshotProgress as
                    quest_hook::libil2cpp::Type > ::class(), "Invoke", 2usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRScreenshotError = unsafe {
            method.invoke_unchecked(self, (screenshotHandle, flProgress))?
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::OVR::OpenVR::IVRScreenshots__UpdateScreenshotProgress as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::System::IntPtr,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::OVR::OpenVR::IVRScreenshots__UpdateScreenshotProgress as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
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
