#[cfg(feature = "System+Net+Http+IMonoHttpClientHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct IMonoHttpClientHandler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+Http+IMonoHttpClientHandler")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Net::Http::IMonoHttpClientHandler {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net.Http";
    const CLASS_NAME: &'static str = "IMonoHttpClientHandler";
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
#[cfg(feature = "System+Net+Http+IMonoHttpClientHandler")]
impl std::ops::Deref for crate::System::Net::Http::IMonoHttpClientHandler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+IMonoHttpClientHandler")]
impl std::ops::DerefMut for crate::System::Net::Http::IMonoHttpClientHandler {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+IMonoHttpClientHandler")]
impl crate::System::Net::Http::IMonoHttpClientHandler {
    pub fn SendAsync(
        &mut self,
        request: quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpRequestMessage>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpResponseMessage>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Net::Http::HttpRequestMessage,
                            >,
                            crate::System::Threading::CancellationToken,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Net::Http::HttpResponseMessage,
                                >,
                            >,
                        >,
                        2usize,
                    >("SendAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SendAsync", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpResponseMessage>,
            >,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (request, cancellationToken))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetWebRequestTimeout(
        &mut self,
        timeout: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::TimeSpan),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetWebRequestTimeout")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetWebRequestTimeout", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (timeout))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_SslOptions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Net::Security::SslClientAuthenticationOptions,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Net::Security::SslClientAuthenticationOptions,
                        >,
                        0usize,
                    >("get_SslOptions")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_SslOptions", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::Security::SslClientAuthenticationOptions,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_SslOptions(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Net::Security::SslClientAuthenticationOptions,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Net::Security::SslClientAuthenticationOptions,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_SslOptions")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_SslOptions", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Http+IMonoHttpClientHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Http::IMonoHttpClientHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+Http+IMonoHttpClientHandler")]
impl AsRef<crate::System::IDisposable>
for crate::System::Net::Http::IMonoHttpClientHandler {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Net+Http+IMonoHttpClientHandler")]
impl AsMut<crate::System::IDisposable>
for crate::System::Net::Http::IMonoHttpClientHandler {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
