#[cfg(feature = "OVR+OpenVR+IVRSettings")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct IVRSettings {
    pub GetSettingsErrorNameFromEnum: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSettings__GetSettingsErrorNameFromEnum,
    >,
    pub Sync: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::IVRSettings__Sync>,
    pub SetBool: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::IVRSettings__SetBool>,
    pub SetInt32: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::IVRSettings__SetInt32>,
    pub SetFloat: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::IVRSettings__SetFloat>,
    pub SetString: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::IVRSettings__SetString>,
    pub GetBool: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::IVRSettings__GetBool>,
    pub GetInt32: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::IVRSettings__GetInt32>,
    pub GetFloat: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::IVRSettings__GetFloat>,
    pub GetString: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::IVRSettings__GetString>,
    pub RemoveSection: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSettings__RemoveSection,
    >,
    pub RemoveKeyInSection: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRSettings__RemoveKeyInSection,
    >,
}
#[cfg(feature = "OVR+OpenVR+IVRSettings")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRSettings => "OVR.OpenVR"
    ."IVRSettings"
);
#[cfg(feature = "OVR+OpenVR+IVRSettings")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::OVR::OpenVR::IVRSettings {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSettings")]
impl crate::OVR::OpenVR::IVRSettings {
    #[cfg(feature = "OVR+OpenVR+IVRSettings+_GetBool")]
    pub type _GetBool = crate::OVR::OpenVR::IVRSettings__GetBool;
    #[cfg(feature = "OVR+OpenVR+IVRSettings+_GetFloat")]
    pub type _GetFloat = crate::OVR::OpenVR::IVRSettings__GetFloat;
    #[cfg(feature = "OVR+OpenVR+IVRSettings+_GetInt32")]
    pub type _GetInt32 = crate::OVR::OpenVR::IVRSettings__GetInt32;
    #[cfg(feature = "OVR+OpenVR+IVRSettings+_GetSettingsErrorNameFromEnum")]
    pub type _GetSettingsErrorNameFromEnum = crate::OVR::OpenVR::IVRSettings__GetSettingsErrorNameFromEnum;
    #[cfg(feature = "OVR+OpenVR+IVRSettings+_GetString")]
    pub type _GetString = crate::OVR::OpenVR::IVRSettings__GetString;
    #[cfg(feature = "OVR+OpenVR+IVRSettings+_RemoveKeyInSection")]
    pub type _RemoveKeyInSection = crate::OVR::OpenVR::IVRSettings__RemoveKeyInSection;
    #[cfg(feature = "OVR+OpenVR+IVRSettings+_RemoveSection")]
    pub type _RemoveSection = crate::OVR::OpenVR::IVRSettings__RemoveSection;
    #[cfg(feature = "OVR+OpenVR+IVRSettings+_SetBool")]
    pub type _SetBool = crate::OVR::OpenVR::IVRSettings__SetBool;
    #[cfg(feature = "OVR+OpenVR+IVRSettings+_SetFloat")]
    pub type _SetFloat = crate::OVR::OpenVR::IVRSettings__SetFloat;
    #[cfg(feature = "OVR+OpenVR+IVRSettings+_SetInt32")]
    pub type _SetInt32 = crate::OVR::OpenVR::IVRSettings__SetInt32;
    #[cfg(feature = "OVR+OpenVR+IVRSettings+_SetString")]
    pub type _SetString = crate::OVR::OpenVR::IVRSettings__SetString;
    #[cfg(feature = "OVR+OpenVR+IVRSettings+_Sync")]
    pub type _Sync = crate::OVR::OpenVR::IVRSettings__Sync;
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_GetBool")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSettings__GetBool {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_GetBool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRSettings__GetBool =>
    "OVR.OpenVR"."IVRSettings/_GetBool"
);
#[cfg(feature = "OVR+OpenVR+IVRSettings+_GetBool")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSettings__GetBool {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_GetBool")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSettings__GetBool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_GetBool")]
impl crate::OVR::OpenVR::IVRSettings__GetBool {
    pub fn BeginInvoke(
        &mut self,
        pchSection: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchSettingsKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
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
                (pchSection, pchSettingsKey, peError, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EndInvoke", (peError, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchSection: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchSettingsKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Invoke", (pchSection, pchSettingsKey, peError))?;
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
#[cfg(feature = "OVR+OpenVR+IVRSettings+_GetBool")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::IVRSettings__GetBool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_GetFloat")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSettings__GetFloat {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_GetFloat")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRSettings__GetFloat =>
    "OVR.OpenVR"."IVRSettings/_GetFloat"
);
#[cfg(feature = "OVR+OpenVR+IVRSettings+_GetFloat")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSettings__GetFloat {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_GetFloat")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSettings__GetFloat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_GetFloat")]
impl crate::OVR::OpenVR::IVRSettings__GetFloat {
    pub fn BeginInvoke(
        &mut self,
        pchSection: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchSettingsKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
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
                (pchSection, pchSettingsKey, peError, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("EndInvoke", (peError, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchSection: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchSettingsKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("Invoke", (pchSection, pchSettingsKey, peError))?;
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
#[cfg(feature = "OVR+OpenVR+IVRSettings+_GetFloat")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::IVRSettings__GetFloat {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_GetInt32")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSettings__GetInt32 {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_GetInt32")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRSettings__GetInt32 =>
    "OVR.OpenVR"."IVRSettings/_GetInt32"
);
#[cfg(feature = "OVR+OpenVR+IVRSettings+_GetInt32")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSettings__GetInt32 {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_GetInt32")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSettings__GetInt32 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_GetInt32")]
impl crate::OVR::OpenVR::IVRSettings__GetInt32 {
    pub fn BeginInvoke(
        &mut self,
        pchSection: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchSettingsKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
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
                (pchSection, pchSettingsKey, peError, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("EndInvoke", (peError, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchSection: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchSettingsKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Invoke", (pchSection, pchSettingsKey, peError))?;
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
#[cfg(feature = "OVR+OpenVR+IVRSettings+_GetInt32")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::IVRSettings__GetInt32 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_GetSettingsErrorNameFromEnum")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSettings__GetSettingsErrorNameFromEnum {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_GetSettingsErrorNameFromEnum")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::IVRSettings__GetSettingsErrorNameFromEnum => "OVR.OpenVR"
    ."IVRSettings/_GetSettingsErrorNameFromEnum"
);
#[cfg(feature = "OVR+OpenVR+IVRSettings+_GetSettingsErrorNameFromEnum")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSettings__GetSettingsErrorNameFromEnum {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_GetSettingsErrorNameFromEnum")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRSettings__GetSettingsErrorNameFromEnum {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_GetSettingsErrorNameFromEnum")]
impl crate::OVR::OpenVR::IVRSettings__GetSettingsErrorNameFromEnum {
    pub fn BeginInvoke(
        &mut self,
        eError: crate::OVR::OpenVR::EVRSettingsError,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (eError, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        eError: crate::OVR::OpenVR::EVRSettingsError,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("Invoke", (eError))?;
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
#[cfg(feature = "OVR+OpenVR+IVRSettings+_GetSettingsErrorNameFromEnum")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSettings__GetSettingsErrorNameFromEnum {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_GetString")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSettings__GetString {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_GetString")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRSettings__GetString =>
    "OVR.OpenVR"."IVRSettings/_GetString"
);
#[cfg(feature = "OVR+OpenVR+IVRSettings+_GetString")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSettings__GetString {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_GetString")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSettings__GetString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_GetString")]
impl crate::OVR::OpenVR::IVRSettings__GetString {
    pub fn BeginInvoke(
        &mut self,
        pchSection: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchSettingsKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchValue: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unValueLen: u32,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
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
                (
                    pchSection,
                    pchSettingsKey,
                    pchValue,
                    unValueLen,
                    peError,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (peError, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchSection: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchSettingsKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchValue: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unValueLen: u32,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Invoke",
                (pchSection, pchSettingsKey, pchValue, unValueLen, peError),
            )?;
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
#[cfg(feature = "OVR+OpenVR+IVRSettings+_GetString")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::IVRSettings__GetString {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_RemoveKeyInSection")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSettings__RemoveKeyInSection {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_RemoveKeyInSection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRSettings__RemoveKeyInSection =>
    "OVR.OpenVR"."IVRSettings/_RemoveKeyInSection"
);
#[cfg(feature = "OVR+OpenVR+IVRSettings+_RemoveKeyInSection")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSettings__RemoveKeyInSection {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_RemoveKeyInSection")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSettings__RemoveKeyInSection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_RemoveKeyInSection")]
impl crate::OVR::OpenVR::IVRSettings__RemoveKeyInSection {
    pub fn BeginInvoke(
        &mut self,
        pchSection: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchSettingsKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
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
                (pchSection, pchSettingsKey, peError, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (peError, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchSection: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchSettingsKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (pchSection, pchSettingsKey, peError))?;
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
#[cfg(feature = "OVR+OpenVR+IVRSettings+_RemoveKeyInSection")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSettings__RemoveKeyInSection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_RemoveSection")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSettings__RemoveSection {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_RemoveSection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRSettings__RemoveSection =>
    "OVR.OpenVR"."IVRSettings/_RemoveSection"
);
#[cfg(feature = "OVR+OpenVR+IVRSettings+_RemoveSection")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSettings__RemoveSection {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_RemoveSection")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSettings__RemoveSection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_RemoveSection")]
impl crate::OVR::OpenVR::IVRSettings__RemoveSection {
    pub fn BeginInvoke(
        &mut self,
        pchSection: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (pchSection, peError, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (peError, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchSection: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (pchSection, peError))?;
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
#[cfg(feature = "OVR+OpenVR+IVRSettings+_RemoveSection")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRSettings__RemoveSection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_SetBool")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSettings__SetBool {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_SetBool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRSettings__SetBool =>
    "OVR.OpenVR"."IVRSettings/_SetBool"
);
#[cfg(feature = "OVR+OpenVR+IVRSettings+_SetBool")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSettings__SetBool {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_SetBool")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSettings__SetBool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_SetBool")]
impl crate::OVR::OpenVR::IVRSettings__SetBool {
    pub fn BeginInvoke(
        &mut self,
        pchSection: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchSettingsKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bValue: bool,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
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
                (pchSection, pchSettingsKey, bValue, peError, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (peError, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchSection: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchSettingsKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bValue: bool,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (pchSection, pchSettingsKey, bValue, peError))?;
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
#[cfg(feature = "OVR+OpenVR+IVRSettings+_SetBool")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::IVRSettings__SetBool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_SetFloat")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSettings__SetFloat {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_SetFloat")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRSettings__SetFloat =>
    "OVR.OpenVR"."IVRSettings/_SetFloat"
);
#[cfg(feature = "OVR+OpenVR+IVRSettings+_SetFloat")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSettings__SetFloat {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_SetFloat")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSettings__SetFloat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_SetFloat")]
impl crate::OVR::OpenVR::IVRSettings__SetFloat {
    pub fn BeginInvoke(
        &mut self,
        pchSection: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchSettingsKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        flValue: f32,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
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
                (pchSection, pchSettingsKey, flValue, peError, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (peError, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchSection: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchSettingsKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        flValue: f32,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (pchSection, pchSettingsKey, flValue, peError))?;
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
#[cfg(feature = "OVR+OpenVR+IVRSettings+_SetFloat")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::IVRSettings__SetFloat {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_SetInt32")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSettings__SetInt32 {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_SetInt32")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRSettings__SetInt32 =>
    "OVR.OpenVR"."IVRSettings/_SetInt32"
);
#[cfg(feature = "OVR+OpenVR+IVRSettings+_SetInt32")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSettings__SetInt32 {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_SetInt32")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSettings__SetInt32 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_SetInt32")]
impl crate::OVR::OpenVR::IVRSettings__SetInt32 {
    pub fn BeginInvoke(
        &mut self,
        pchSection: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchSettingsKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        nValue: i32,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
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
                (pchSection, pchSettingsKey, nValue, peError, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (peError, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchSection: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchSettingsKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        nValue: i32,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (pchSection, pchSettingsKey, nValue, peError))?;
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
#[cfg(feature = "OVR+OpenVR+IVRSettings+_SetInt32")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::IVRSettings__SetInt32 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_SetString")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSettings__SetString {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_SetString")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRSettings__SetString =>
    "OVR.OpenVR"."IVRSettings/_SetString"
);
#[cfg(feature = "OVR+OpenVR+IVRSettings+_SetString")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSettings__SetString {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_SetString")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSettings__SetString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_SetString")]
impl crate::OVR::OpenVR::IVRSettings__SetString {
    pub fn BeginInvoke(
        &mut self,
        pchSection: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchSettingsKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
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
                (pchSection, pchSettingsKey, pchValue, peError, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (peError, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchSection: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchSettingsKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (pchSection, pchSettingsKey, pchValue, peError))?;
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
#[cfg(feature = "OVR+OpenVR+IVRSettings+_SetString")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::IVRSettings__SetString {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_Sync")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRSettings__Sync {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_Sync")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRSettings__Sync => "OVR.OpenVR"
    ."IVRSettings/_Sync"
);
#[cfg(feature = "OVR+OpenVR+IVRSettings+_Sync")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRSettings__Sync {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_Sync")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRSettings__Sync {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRSettings+_Sync")]
impl crate::OVR::OpenVR::IVRSettings__Sync {
    pub fn BeginInvoke(
        &mut self,
        bForce: bool,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (bForce, peError, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EndInvoke", (peError, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        bForce: bool,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Invoke", (bForce, peError))?;
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
#[cfg(feature = "OVR+OpenVR+IVRSettings+_Sync")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::IVRSettings__Sync {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
