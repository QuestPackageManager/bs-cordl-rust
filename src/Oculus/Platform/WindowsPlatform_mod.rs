#[cfg(feature = "Oculus+Platform+WindowsPlatform")]
#[repr(C)]
#[derive(Debug)]
pub struct WindowsPlatform {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Oculus+Platform+WindowsPlatform")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::WindowsPlatform =>
    "Oculus.Platform"."WindowsPlatform"
);
#[cfg(feature = "Oculus+Platform+WindowsPlatform")]
impl std::ops::Deref for crate::Oculus::Platform::WindowsPlatform {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+WindowsPlatform")]
impl std::ops::DerefMut for crate::Oculus::Platform::WindowsPlatform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+WindowsPlatform")]
impl crate::Oculus::Platform::WindowsPlatform {
    #[cfg(feature = "Oculus+Platform+WindowsPlatform+UnityLogDelegate")]
    pub type UnityLogDelegate = crate::Oculus::Platform::WindowsPlatform_UnityLogDelegate;
    pub fn AsyncInitialize(
        &mut self,
        appId: *mut crate::System::String,
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
        > = __cordl_object.invoke("AsyncInitialize", (appId))?;
        Ok(__cordl_ret)
    }
    pub fn CPPLogCallback(
        &mut self,
        tag: crate::System::IntPtr,
        message: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CPPLogCallback", (tag, message))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize(
        &mut self,
        appId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Initialize", (appId))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn getCallbackPointer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("getCallbackPointer", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Oculus+Platform+WindowsPlatform")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::WindowsPlatform {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Oculus+Platform+WindowsPlatform+UnityLogDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct WindowsPlatform_UnityLogDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "Oculus+Platform+WindowsPlatform+UnityLogDelegate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Oculus::Platform::WindowsPlatform_UnityLogDelegate => "Oculus.Platform"
    ."WindowsPlatform/UnityLogDelegate"
);
#[cfg(feature = "Oculus+Platform+WindowsPlatform+UnityLogDelegate")]
impl std::ops::Deref for crate::Oculus::Platform::WindowsPlatform_UnityLogDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+WindowsPlatform+UnityLogDelegate")]
impl std::ops::DerefMut for crate::Oculus::Platform::WindowsPlatform_UnityLogDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+WindowsPlatform+UnityLogDelegate")]
impl crate::Oculus::Platform::WindowsPlatform_UnityLogDelegate {
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
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "Oculus+Platform+WindowsPlatform+UnityLogDelegate")]
impl quest_hook::libil2cpp::ObjectType
for crate::Oculus::Platform::WindowsPlatform_UnityLogDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
