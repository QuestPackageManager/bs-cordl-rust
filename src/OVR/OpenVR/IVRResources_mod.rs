#[cfg(feature = "OVR+OpenVR+IVRResources")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct IVRResources {
    pub LoadSharedResource: *mut crate::OVR::OpenVR::IVRResources__LoadSharedResource,
    pub GetResourceFullPath: *mut crate::OVR::OpenVR::IVRResources__GetResourceFullPath,
}
#[cfg(feature = "OVR+OpenVR+IVRResources")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRResources => "OVR.OpenVR"
    ."IVRResources"
);
#[cfg(feature = "OVR+OpenVR+IVRResources")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::OVR::OpenVR::IVRResources {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRResources")]
impl crate::OVR::OpenVR::IVRResources {
    #[cfg(feature = "OVR+OpenVR+IVRResources+_LoadSharedResource")]
    pub type _LoadSharedResource = crate::OVR::OpenVR::IVRResources__LoadSharedResource;
    #[cfg(feature = "OVR+OpenVR+IVRResources+_GetResourceFullPath")]
    pub type _GetResourceFullPath = crate::OVR::OpenVR::IVRResources__GetResourceFullPath;
}
#[cfg(feature = "OVR+OpenVR+IVRResources+_GetResourceFullPath")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRResources__GetResourceFullPath {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRResources+_GetResourceFullPath")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRResources__GetResourceFullPath
    => "OVR.OpenVR"."IVRResources/_GetResourceFullPath"
);
#[cfg(feature = "OVR+OpenVR+IVRResources+_GetResourceFullPath")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRResources__GetResourceFullPath {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRResources+_GetResourceFullPath")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRResources__GetResourceFullPath {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRResources+_GetResourceFullPath")]
impl crate::OVR::OpenVR::IVRResources__GetResourceFullPath {
    pub fn BeginInvoke(
        &mut self,
        pchResourceName: *mut crate::System::String,
        pchResourceTypeDirectory: *mut crate::System::String,
        pchPathBuffer: *mut crate::System::Text::StringBuilder,
        unBufferLen: u32,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke(
                "BeginInvoke",
                (
                    pchResourceName,
                    pchResourceTypeDirectory,
                    pchPathBuffer,
                    unBufferLen,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        pchResourceName: *mut crate::System::String,
        pchResourceTypeDirectory: *mut crate::System::String,
        pchPathBuffer: *mut crate::System::Text::StringBuilder,
        unBufferLen: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke(
                "Invoke",
                (pchResourceName, pchResourceTypeDirectory, pchPathBuffer, unBufferLen),
            )?;
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
#[cfg(feature = "OVR+OpenVR+IVRResources+_GetResourceFullPath")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRResources__GetResourceFullPath {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRResources+_LoadSharedResource")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRResources__LoadSharedResource {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRResources+_LoadSharedResource")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRResources__LoadSharedResource =>
    "OVR.OpenVR"."IVRResources/_LoadSharedResource"
);
#[cfg(feature = "OVR+OpenVR+IVRResources+_LoadSharedResource")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRResources__LoadSharedResource {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRResources+_LoadSharedResource")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRResources__LoadSharedResource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRResources+_LoadSharedResource")]
impl crate::OVR::OpenVR::IVRResources__LoadSharedResource {
    pub fn BeginInvoke(
        &mut self,
        pchResourceName: *mut crate::System::String,
        pchBuffer: *mut crate::System::String,
        unBufferLen: u32,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke(
                "BeginInvoke",
                (pchResourceName, pchBuffer, unBufferLen, callback, object),
            )?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        pchResourceName: *mut crate::System::String,
        pchBuffer: *mut crate::System::String,
        unBufferLen: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke("Invoke", (pchResourceName, pchBuffer, unBufferLen))?;
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
#[cfg(feature = "OVR+OpenVR+IVRResources+_LoadSharedResource")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRResources__LoadSharedResource {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
