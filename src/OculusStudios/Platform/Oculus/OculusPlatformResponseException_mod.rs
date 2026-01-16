#[cfg(feature = "cordl_class_OculusStudios+Platform+Oculus+OculusPlatformResponseException")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusPlatformResponseException {
    __cordl_parent: crate::System::Exception,
    pub error: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Error>,
    pub errorCode: crate::OculusStudios::Platform::Oculus::OculusPlatformResponseErrorCode,
    pub httpCode: crate::System::Net::HttpStatusCode,
}
#[cfg(feature = "cordl_class_OculusStudios+Platform+Oculus+OculusPlatformResponseException")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::OculusStudios::Platform::Oculus::OculusPlatformResponseException
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OculusStudios.Platform.Oculus";
    const CLASS_NAME: &'static str = "OculusPlatformResponseException";
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
#[cfg(feature = "OculusStudios+Platform+Oculus+OculusPlatformResponseException")]
impl std::ops::Deref for crate::OculusStudios::Platform::Oculus::OculusPlatformResponseException {
    type Target = crate::System::Exception;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusStudios+Platform+Oculus+OculusPlatformResponseException")]
impl std::ops::DerefMut
    for crate::OculusStudios::Platform::Oculus::OculusPlatformResponseException
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusStudios+Platform+Oculus+OculusPlatformResponseException")]
impl crate::OculusStudios::Platform::Oculus::OculusPlatformResponseException {
    pub fn New_Error0(
        error: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Error>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (error))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_Error1(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        error: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Error>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (message, error))?;
        Ok(__cordl_object.into())
    }
    pub fn ParseMessage(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        error: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Error>,
        errorCode: quest_hook::libil2cpp::ByRefMut<
            crate::OculusStudios::Platform::Oculus::OculusPlatformResponseErrorCode,
        >,
        httpCode: quest_hook::libil2cpp::ByRefMut<crate::System::Net::HttpStatusCode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Error>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::OculusStudios::Platform::Oculus::OculusPlatformResponseErrorCode,
                        >,
                        quest_hook::libil2cpp::ByRefMut<crate::System::Net::HttpStatusCode>,
                    ), quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>, 4usize>(
                        "ParseMessage",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ParseMessage",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> = unsafe {
            cordl_method_info.invoke_unchecked((), (message, error, errorCode, httpCode))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Error0(
        &mut self,
        error: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Error>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::Error,
                        >),
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
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (error))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_Error1(
        &mut self,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        error: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Error>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Error>,
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
            unsafe { cordl_method_info.invoke_unchecked(self, (message, error))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_message(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("get_message")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_message", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OculusStudios+Platform+Oculus+OculusPlatformResponseException")]
impl quest_hook::libil2cpp::ObjectType
    for crate::OculusStudios::Platform::Oculus::OculusPlatformResponseException
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
