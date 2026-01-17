#[cfg(feature = "cordl_class_Oculus+Platform+OculusPlatformExtensions")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct OculusPlatformExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Oculus+Platform+OculusPlatformExtensions")]
unsafe impl quest_hook::libil2cpp::Type for crate::Oculus::Platform::OculusPlatformExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Oculus.Platform";
    const CLASS_NAME: &'static str = "OculusPlatformExtensions";
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
#[cfg(feature = "Oculus+Platform+OculusPlatformExtensions")]
impl std::ops::Deref for crate::Oculus::Platform::OculusPlatformExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+OculusPlatformExtensions")]
impl std::ops::DerefMut for crate::Oculus::Platform::OculusPlatformExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+OculusPlatformExtensions")]
impl crate::Oculus::Platform::OculusPlatformExtensions {
    pub fn GetAwaiter_Request1(
        oculusRequest: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::CompilerServices::TaskAwaiter_1<
            quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>),
                        crate::System::Runtime::CompilerServices::TaskAwaiter_1<
                            quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message>,
                        >,
                        1usize,
                    >("GetAwaiter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetAwaiter", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Runtime::CompilerServices::TaskAwaiter_1<
            quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (oculusRequest))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAwaiter_Request_1_0<T>(
        oculusRequest: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request_1<T>>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::CompilerServices::TaskAwaiter_1<
            quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message_1<T>>,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Request_1<T>,
                        >),
                        crate::System::Runtime::CompilerServices::TaskAwaiter_1<
                            quest_hook::libil2cpp::Gc<
                                crate::Oculus::Platform::Message_1<T>,
                            >,
                        >,
                        1usize,
                    >("GetAwaiter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetAwaiter", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Runtime::CompilerServices::TaskAwaiter_1<
            quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message_1<T>>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (oculusRequest))? };
        Ok(__cordl_ret.into())
    }
    pub fn WaitAsync<T>(
        oculusRequest: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request_1<T>>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message_1<T>>,
            >,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request_1<T>>,
                        crate::System::Threading::CancellationToken,
                    ), quest_hook::libil2cpp::Gc<
                        crate::System::Threading::Tasks::Task_1<
                            quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message_1<T>>,
                        >,
                    >, 2usize>("WaitAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WaitAsync",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message_1<T>>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (oculusRequest, cancellationToken))? };
        Ok(__cordl_ret.into())
    }
    pub fn WaitWithTimeoutAsync<T>(
        oculusRequest: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request_1<T>>,
        timeout: crate::System::TimeSpan,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message_1<T>>,
            >,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request_1<T>>,
                        crate::System::TimeSpan,
                        crate::System::Threading::CancellationToken,
                    ), quest_hook::libil2cpp::Gc<
                        crate::System::Threading::Tasks::Task_1<
                            quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message_1<T>>,
                        >,
                    >, 3usize>("WaitWithTimeoutAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WaitWithTimeoutAsync",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message_1<T>>,
            >,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (oculusRequest, timeout, cancellationToken))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Oculus+Platform+OculusPlatformExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::OculusPlatformExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
