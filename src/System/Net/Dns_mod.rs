#[cfg(feature = "System+Net+Dns")]
#[repr(C)]
#[derive(Debug)]
pub struct Dns {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+Dns")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Net::Dns {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "Dns";
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
#[cfg(feature = "System+Net+Dns")]
impl std::ops::Deref for crate::System::Net::Dns {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn BeginGetHostAddresses(
        hostNameOrAddress: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        requestCallback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "BeginGetHostAddresses",
                (hostNameOrAddress, requestCallback, state),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndGetHostAddresses(
        asyncResult: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Net::IPAddress>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Net::IPAddress>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EndGetHostAddresses", (asyncResult))?;
        Ok(__cordl_ret.into())
    }
    pub fn Error_11001(
        hostName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Error_11001", (hostName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHostAddresses(
        hostNameOrAddress: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Net::IPAddress>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Net::IPAddress>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHostAddresses", (hostNameOrAddress))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHostAddressesAsync(
        hostNameOrAddress: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<crate::System::Net::IPAddress>,
                    >,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<crate::System::Net::IPAddress>,
                    >,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHostAddressesAsync", (hostNameOrAddress))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHostByAddr_icall(
        addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        h_name: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        h_aliases: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                >,
            >,
        >,
        h_addr_list: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                >,
            >,
        >,
        hint: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetHostByAddr_icall",
                (addr, h_name, h_aliases, h_addr_list, hint),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHostByAddressFromString(
        address: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parse: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::IPHostEntry>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::IPHostEntry> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHostByAddressFromString", (address, parse))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHostByName(
        hostName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::IPHostEntry>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::IPHostEntry> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHostByName", (hostName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHostByName_icall(
        host: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        h_name: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        h_aliases: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                >,
            >,
        >,
        h_addr_list: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                >,
            >,
        >,
        hint: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetHostByName_icall",
                (host, h_name, h_aliases, h_addr_list, hint),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHostEntry_IPAddress1(
        address: quest_hook::libil2cpp::Gc<crate::System::Net::IPAddress>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::IPHostEntry>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::IPHostEntry> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHostEntry", (address))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHostEntry_Il2CppString0(
        hostNameOrAddress: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::IPHostEntry>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::IPHostEntry> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHostEntry", (hostNameOrAddress))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHostName() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetHostName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHostName_icall(
        h_name: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHostName_icall", (h_name))?;
        Ok(__cordl_ret.into())
    }
    pub fn hostent_to_IPHostEntry(
        originalHostName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        h_name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        h_aliases: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        h_addrlist: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::IPHostEntry>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::IPHostEntry> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "hostent_to_IPHostEntry",
                (originalHostName, h_name, h_aliases, h_addrlist),
            )?;
        Ok(__cordl_ret.into())
    }
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
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Net::Dns_GetHostAddressesCallback {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "Dns/GetHostAddressesCallback";
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
        hostName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (hostName, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Net::IPAddress>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Net::IPAddress>,
            >,
        > = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        hostName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Net::IPAddress>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Net::IPAddress>,
            >,
        > = __cordl_object.invoke("Invoke", (hostName))?;
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
