#[cfg(feature = "System+Net+IPAddress+ReadOnlyIPAddress")]
#[repr(C)]
#[derive(Debug)]
pub struct IPAddress_ReadOnlyIPAddress {
    __cordl_parent: crate::System::Net::IPAddress,
}
#[cfg(feature = "System+Net+IPAddress+ReadOnlyIPAddress")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::IPAddress_ReadOnlyIPAddress {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "IPAddress/ReadOnlyIPAddress";
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
#[cfg(feature = "System+Net+IPAddress+ReadOnlyIPAddress")]
impl std::ops::Deref for crate::GlobalNamespace::IPAddress_ReadOnlyIPAddress {
    type Target = crate::System::Net::IPAddress;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+IPAddress+ReadOnlyIPAddress")]
impl std::ops::DerefMut for crate::GlobalNamespace::IPAddress_ReadOnlyIPAddress {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+IPAddress+ReadOnlyIPAddress")]
impl crate::GlobalNamespace::IPAddress_ReadOnlyIPAddress {
    pub fn New(
        newAddress: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (newAddress))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        newAddress: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IPAddress_ReadOnlyIPAddress as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i64), quest_hook::libil2cpp::Void, 1usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IPAddress_ReadOnlyIPAddress as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (newAddress))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+IPAddress+ReadOnlyIPAddress")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IPAddress_ReadOnlyIPAddress {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
