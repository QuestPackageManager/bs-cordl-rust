#[cfg(feature = "System+Net+NetworkInformation+MacOsNetworkInterface")]
#[repr(C)]
#[derive(Debug)]
pub struct MacOsNetworkInterface {
    __cordl_parent: crate::System::Net::NetworkInformation::UnixNetworkInterface,
    pub _ifa_flags: u32,
}
#[cfg(feature = "System+Net+NetworkInformation+MacOsNetworkInterface")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Net::NetworkInformation::MacOsNetworkInterface {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net.NetworkInformation";
    const CLASS_NAME: &'static str = "MacOsNetworkInterface";
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
#[cfg(feature = "System+Net+NetworkInformation+MacOsNetworkInterface")]
impl std::ops::Deref for crate::System::Net::NetworkInformation::MacOsNetworkInterface {
    type Target = crate::System::Net::NetworkInformation::UnixNetworkInterface;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+MacOsNetworkInterface")]
impl std::ops::DerefMut
for crate::System::Net::NetworkInformation::MacOsNetworkInterface {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+MacOsNetworkInterface")]
impl crate::System::Net::NetworkInformation::MacOsNetworkInterface {
    pub fn GetIPProperties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Net::NetworkInformation::IPInterfaceProperties,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Net::NetworkInformation::IPInterfaceProperties,
                >,
                0usize,
            >("GetIPProperties")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetIPProperties", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::NetworkInformation::IPInterfaceProperties,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ifa_flags: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, ifa_flags))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ifa_flags: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>, u32),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (name, ifa_flags))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_OperationalStatus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Net::NetworkInformation::OperationalStatus,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::System::Net::NetworkInformation::OperationalStatus,
                0usize,
            >("get_OperationalStatus")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_OperationalStatus", 0usize
                )
            });
        let __cordl_ret: crate::System::Net::NetworkInformation::OperationalStatus = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+NetworkInformation+MacOsNetworkInterface")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::NetworkInformation::MacOsNetworkInterface {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
