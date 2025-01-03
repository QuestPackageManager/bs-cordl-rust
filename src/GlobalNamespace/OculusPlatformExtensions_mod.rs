#[cfg(feature = "OculusPlatformExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusPlatformExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OculusPlatformExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OculusPlatformExtensions => ""
    ."OculusPlatformExtensions"
);
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
            *mut crate::Oculus::Platform::Message,
        >,
    > {
        let __cordl_ret: crate::System::Runtime::CompilerServices::TaskAwaiter_1<
            *mut crate::Oculus::Platform::Message,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAwaiter", (oculusRequest))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAwaiter_Request_1_0<T>(
        oculusRequest: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request_1<T>>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::CompilerServices::TaskAwaiter_1<
            *mut crate::Oculus::Platform::Message_1<T>,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::System::Runtime::CompilerServices::TaskAwaiter_1<
            *mut crate::Oculus::Platform::Message_1<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAwaiter", (oculusRequest))?;
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
