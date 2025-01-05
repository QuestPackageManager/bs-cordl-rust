#[cfg(feature = "OVR+OpenVR+IVRExtendedDisplay")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct IVRExtendedDisplay {
    pub GetWindowBounds: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRExtendedDisplay__GetWindowBounds,
    >,
    pub GetEyeOutputViewport: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRExtendedDisplay__GetEyeOutputViewport,
    >,
    pub GetDXGIOutputInfo: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRExtendedDisplay__GetDXGIOutputInfo,
    >,
}
#[cfg(feature = "OVR+OpenVR+IVRExtendedDisplay")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRExtendedDisplay => "OVR.OpenVR"
    ."IVRExtendedDisplay"
);
#[cfg(feature = "OVR+OpenVR+IVRExtendedDisplay")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::OVR::OpenVR::IVRExtendedDisplay {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRExtendedDisplay")]
impl crate::OVR::OpenVR::IVRExtendedDisplay {
    #[cfg(feature = "OVR+OpenVR+IVRExtendedDisplay+_GetDXGIOutputInfo")]
    pub type _GetDXGIOutputInfo = crate::OVR::OpenVR::IVRExtendedDisplay__GetDXGIOutputInfo;
    #[cfg(feature = "OVR+OpenVR+IVRExtendedDisplay+_GetEyeOutputViewport")]
    pub type _GetEyeOutputViewport = crate::OVR::OpenVR::IVRExtendedDisplay__GetEyeOutputViewport;
    #[cfg(feature = "OVR+OpenVR+IVRExtendedDisplay+_GetWindowBounds")]
    pub type _GetWindowBounds = crate::OVR::OpenVR::IVRExtendedDisplay__GetWindowBounds;
}
#[cfg(feature = "OVR+OpenVR+IVRExtendedDisplay+_GetDXGIOutputInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRExtendedDisplay__GetDXGIOutputInfo {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
}
#[cfg(feature = "OVR+OpenVR+IVRExtendedDisplay+_GetDXGIOutputInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRExtendedDisplay__GetDXGIOutputInfo => "OVR.OpenVR"
    ."IVRExtendedDisplay/_GetDXGIOutputInfo"
);
#[cfg(feature = "OVR+OpenVR+IVRExtendedDisplay+_GetDXGIOutputInfo")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRExtendedDisplay__GetDXGIOutputInfo {
    type Target = quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRExtendedDisplay+_GetDXGIOutputInfo")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRExtendedDisplay__GetDXGIOutputInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRExtendedDisplay+_GetDXGIOutputInfo")]
impl crate::OVR::OpenVR::IVRExtendedDisplay__GetDXGIOutputInfo {
    pub fn BeginInvoke(
        &mut self,
        pnAdapterIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        pnAdapterOutputIndex: quest_hook::libil2cpp::ByRefMut<i32>,
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
                (pnAdapterIndex, pnAdapterOutputIndex, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pnAdapterIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        pnAdapterOutputIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (pnAdapterIndex, pnAdapterOutputIndex, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pnAdapterIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        pnAdapterOutputIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (pnAdapterIndex, pnAdapterOutputIndex))?;
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
#[cfg(feature = "OVR+OpenVR+IVRExtendedDisplay+_GetDXGIOutputInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRExtendedDisplay__GetDXGIOutputInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRExtendedDisplay+_GetEyeOutputViewport")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRExtendedDisplay__GetEyeOutputViewport {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
}
#[cfg(feature = "OVR+OpenVR+IVRExtendedDisplay+_GetEyeOutputViewport")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRExtendedDisplay__GetEyeOutputViewport => "OVR.OpenVR"
    ."IVRExtendedDisplay/_GetEyeOutputViewport"
);
#[cfg(feature = "OVR+OpenVR+IVRExtendedDisplay+_GetEyeOutputViewport")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRExtendedDisplay__GetEyeOutputViewport {
    type Target = quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRExtendedDisplay+_GetEyeOutputViewport")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRExtendedDisplay__GetEyeOutputViewport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRExtendedDisplay+_GetEyeOutputViewport")]
impl crate::OVR::OpenVR::IVRExtendedDisplay__GetEyeOutputViewport {
    pub fn BeginInvoke(
        &mut self,
        eEye: crate::OVR::OpenVR::EVREye,
        pnX: quest_hook::libil2cpp::ByRefMut<u32>,
        pnY: quest_hook::libil2cpp::ByRefMut<u32>,
        pnWidth: quest_hook::libil2cpp::ByRefMut<u32>,
        pnHeight: quest_hook::libil2cpp::ByRefMut<u32>,
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
                (eEye, pnX, pnY, pnWidth, pnHeight, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pnX: quest_hook::libil2cpp::ByRefMut<u32>,
        pnY: quest_hook::libil2cpp::ByRefMut<u32>,
        pnWidth: quest_hook::libil2cpp::ByRefMut<u32>,
        pnHeight: quest_hook::libil2cpp::ByRefMut<u32>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (pnX, pnY, pnWidth, pnHeight, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        eEye: crate::OVR::OpenVR::EVREye,
        pnX: quest_hook::libil2cpp::ByRefMut<u32>,
        pnY: quest_hook::libil2cpp::ByRefMut<u32>,
        pnWidth: quest_hook::libil2cpp::ByRefMut<u32>,
        pnHeight: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (eEye, pnX, pnY, pnWidth, pnHeight))?;
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
#[cfg(feature = "OVR+OpenVR+IVRExtendedDisplay+_GetEyeOutputViewport")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRExtendedDisplay__GetEyeOutputViewport {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRExtendedDisplay+_GetWindowBounds")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRExtendedDisplay__GetWindowBounds {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
}
#[cfg(feature = "OVR+OpenVR+IVRExtendedDisplay+_GetWindowBounds")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRExtendedDisplay__GetWindowBounds
    => "OVR.OpenVR"."IVRExtendedDisplay/_GetWindowBounds"
);
#[cfg(feature = "OVR+OpenVR+IVRExtendedDisplay+_GetWindowBounds")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRExtendedDisplay__GetWindowBounds {
    type Target = quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRExtendedDisplay+_GetWindowBounds")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRExtendedDisplay__GetWindowBounds {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRExtendedDisplay+_GetWindowBounds")]
impl crate::OVR::OpenVR::IVRExtendedDisplay__GetWindowBounds {
    pub fn BeginInvoke(
        &mut self,
        pnX: quest_hook::libil2cpp::ByRefMut<i32>,
        pnY: quest_hook::libil2cpp::ByRefMut<i32>,
        pnWidth: quest_hook::libil2cpp::ByRefMut<u32>,
        pnHeight: quest_hook::libil2cpp::ByRefMut<u32>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (pnX, pnY, pnWidth, pnHeight, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pnX: quest_hook::libil2cpp::ByRefMut<i32>,
        pnY: quest_hook::libil2cpp::ByRefMut<i32>,
        pnWidth: quest_hook::libil2cpp::ByRefMut<u32>,
        pnHeight: quest_hook::libil2cpp::ByRefMut<u32>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (pnX, pnY, pnWidth, pnHeight, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pnX: quest_hook::libil2cpp::ByRefMut<i32>,
        pnY: quest_hook::libil2cpp::ByRefMut<i32>,
        pnWidth: quest_hook::libil2cpp::ByRefMut<u32>,
        pnHeight: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (pnX, pnY, pnWidth, pnHeight))?;
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
#[cfg(feature = "OVR+OpenVR+IVRExtendedDisplay+_GetWindowBounds")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRExtendedDisplay__GetWindowBounds {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
