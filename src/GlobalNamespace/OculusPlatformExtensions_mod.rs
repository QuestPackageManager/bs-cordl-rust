#[cfg(feature = "OculusPlatformExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusPlatformExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OculusPlatformExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OculusPlatformExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OculusPlatformExtensions";
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
#[cfg(feature = "OculusPlatformExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::OculusPlatformExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusPlatformExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::OculusPlatformExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusPlatformExtensions")]
impl crate::GlobalNamespace::OculusPlatformExtensions {
    pub fn GetAwaiter_Request1(
        oculusRequest: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::CompilerServices::TaskAwaiter_1<
            quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message>,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
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
                    self, "GetAwaiter", 1usize
                )
            });
        let __cordl_ret: crate::System::Runtime::CompilerServices::TaskAwaiter_1<
            quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message>,
        > = unsafe { method.invoke_unchecked((), (oculusRequest)) };
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
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request_1<T>>),
                crate::System::Runtime::CompilerServices::TaskAwaiter_1<
                    quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message_1<T>>,
                >,
                1usize,
            >("GetAwaiter")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetAwaiter", 1usize
                )
            });
        let __cordl_ret: crate::System::Runtime::CompilerServices::TaskAwaiter_1<
            quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message_1<T>>,
        > = unsafe { method.invoke_unchecked((), (oculusRequest)) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OculusPlatformExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OculusPlatformExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
