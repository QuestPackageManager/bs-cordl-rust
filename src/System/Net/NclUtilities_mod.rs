#[cfg(feature = "System+Net+NclUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct NclUtilities {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Net+NclUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::NclUtilities => "System.Net"
    ."NclUtilities"
);
#[cfg(feature = "System+Net+NclUtilities")]
impl std::ops::Deref for crate::System::Net::NclUtilities {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NclUtilities")]
impl std::ops::DerefMut for crate::System::Net::NclUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NclUtilities")]
impl crate::System::Net::NclUtilities {
    pub fn GetLocalHost() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::IPHostEntry>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::IPHostEntry> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLocalHost", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsAddressLocal(
        ipAddress: quest_hook::libil2cpp::Gc<crate::System::Net::IPAddress>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsAddressLocal", (ipAddress))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsFatal(
        exception: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsFatal", (exception))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LocalAddresses() -> quest_hook::libil2cpp::Result<
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
            .invoke("get_LocalAddresses", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LocalAddressesLock() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_LocalAddressesLock", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+NclUtilities")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::NclUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
