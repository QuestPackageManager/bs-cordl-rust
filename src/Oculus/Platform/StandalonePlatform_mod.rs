#[cfg(feature = "Oculus+Platform+StandalonePlatform")]
#[repr(C)]
#[derive(Debug)]
pub struct StandalonePlatform {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Oculus+Platform+StandalonePlatform")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::StandalonePlatform =>
    "Oculus.Platform"."StandalonePlatform"
);
#[cfg(feature = "Oculus+Platform+StandalonePlatform")]
impl std::ops::Deref for crate::Oculus::Platform::StandalonePlatform {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+StandalonePlatform")]
impl std::ops::DerefMut for crate::Oculus::Platform::StandalonePlatform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+StandalonePlatform")]
impl crate::Oculus::Platform::StandalonePlatform {
    #[cfg(feature = "Oculus+Platform+StandalonePlatform+UnityLogDelegate")]
    pub type UnityLogDelegate = crate::Oculus::Platform::StandalonePlatform_UnityLogDelegate;
    pub fn AsyncInitialize(
        &mut self,
        appID: u64,
        accessToken: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Request_1<
            *mut crate::Oculus::Platform::Models::PlatformInitialize,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Request_1<
            *mut crate::Oculus::Platform::Models::PlatformInitialize,
        > = __cordl_object.invoke("AsyncInitialize", (appID, accessToken))?;
        Ok(__cordl_ret)
    }
    pub fn AsyncInitializeWithAccessTokenAndOptions(
        &mut self,
        appId: *mut crate::System::String,
        accessToken: *mut crate::System::String,
        initConfigOptions: *mut crate::System::Collections::Generic::Dictionary_2<
            crate::Oculus::Platform::InitConfigOptions,
            bool,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Request_1<
            *mut crate::Oculus::Platform::Models::PlatformInitialize,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Request_1<
            *mut crate::Oculus::Platform::Models::PlatformInitialize,
        > = __cordl_object
            .invoke(
                "AsyncInitializeWithAccessTokenAndOptions",
                (appId, accessToken, initConfigOptions),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn InitializeInEditor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Request_1<
            *mut crate::Oculus::Platform::Models::PlatformInitialize,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Request_1<
            *mut crate::Oculus::Platform::Models::PlatformInitialize,
        > = __cordl_object.invoke("InitializeInEditor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Oculus+Platform+StandalonePlatform")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::StandalonePlatform {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Oculus+Platform+StandalonePlatform+UnityLogDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct StandalonePlatform_UnityLogDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "Oculus+Platform+StandalonePlatform+UnityLogDelegate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Oculus::Platform::StandalonePlatform_UnityLogDelegate => "Oculus.Platform"
    ."StandalonePlatform/UnityLogDelegate"
);
#[cfg(feature = "Oculus+Platform+StandalonePlatform+UnityLogDelegate")]
impl std::ops::Deref for crate::Oculus::Platform::StandalonePlatform_UnityLogDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+StandalonePlatform+UnityLogDelegate")]
impl std::ops::DerefMut
for crate::Oculus::Platform::StandalonePlatform_UnityLogDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+StandalonePlatform+UnityLogDelegate")]
impl crate::Oculus::Platform::StandalonePlatform_UnityLogDelegate {
    pub fn Invoke(
        &mut self,
        tag: crate::System::IntPtr,
        msg: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (tag, msg))?;
        Ok(__cordl_ret)
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
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn BeginInvoke(
        &mut self,
        tag: crate::System::IntPtr,
        msg: crate::System::IntPtr,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (tag, msg, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Oculus+Platform+StandalonePlatform+UnityLogDelegate")]
impl quest_hook::libil2cpp::ObjectType
for crate::Oculus::Platform::StandalonePlatform_UnityLogDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
