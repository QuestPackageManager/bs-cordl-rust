#[cfg(feature = "System+Net+Dns")]
#[repr(C)]
#[derive(Debug)]
pub struct Dns {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Net+Dns")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Dns => "System.Net"."Dns"
);
#[cfg(feature = "System+Net+Dns")]
impl std::ops::Deref for crate::System::Net::Dns {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Dns")]
impl std::ops::DerefMut for crate::System::Net::Dns {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Dns")]
impl crate::System::Net::Dns {
    #[cfg(feature = "System+Net+Dns+GetHostAddressesCallback")]
    pub type GetHostAddressesCallback = crate::System::Net::Dns_GetHostAddressesCallback;
}
#[cfg(feature = "System+Net+Dns")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::Dns {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+Dns+GetHostAddressesCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct Dns_GetHostAddressesCallback {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "System+Net+Dns+GetHostAddressesCallback")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Dns_GetHostAddressesCallback =>
    "System.Net"."Dns/GetHostAddressesCallback"
);
#[cfg(feature = "System+Net+Dns+GetHostAddressesCallback")]
impl std::ops::Deref for crate::System::Net::Dns_GetHostAddressesCallback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Dns+GetHostAddressesCallback")]
impl std::ops::DerefMut for crate::System::Net::Dns_GetHostAddressesCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Dns+GetHostAddressesCallback")]
impl crate::System::Net::Dns_GetHostAddressesCallback {
    pub fn BeginInvoke(
        &mut self,
        hostName: *mut crate::System::String,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (hostName, callback, object))?;
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
    pub fn Invoke(
        &mut self,
        hostName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Net::IPAddress>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Net::IPAddress,
        > = __cordl_object.invoke("Invoke", (hostName))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Net::IPAddress>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Net::IPAddress,
        > = __cordl_object.invoke("EndInvoke", (result))?;
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
#[cfg(feature = "System+Net+Dns+GetHostAddressesCallback")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Dns_GetHostAddressesCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
