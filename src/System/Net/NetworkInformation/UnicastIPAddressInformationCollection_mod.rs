#[cfg(feature = "System+Net+NetworkInformation+UnicastIPAddressInformationCollection")]
#[repr(C)]
#[derive(Debug)]
pub struct UnicastIPAddressInformationCollection {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub addresses: *mut crate::System::Collections::ObjectModel::Collection_1<
        *mut crate::System::Net::NetworkInformation::UnicastIPAddressInformation,
    >,
}
#[cfg(feature = "System+Net+NetworkInformation+UnicastIPAddressInformationCollection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::NetworkInformation::UnicastIPAddressInformationCollection =>
    "System.Net.NetworkInformation"."UnicastIPAddressInformationCollection"
);
#[cfg(feature = "System+Net+NetworkInformation+UnicastIPAddressInformationCollection")]
impl std::ops::Deref
for crate::System::Net::NetworkInformation::UnicastIPAddressInformationCollection {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+UnicastIPAddressInformationCollection")]
impl std::ops::DerefMut
for crate::System::Net::NetworkInformation::UnicastIPAddressInformationCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+UnicastIPAddressInformationCollection")]
impl crate::System::Net::NetworkInformation::UnicastIPAddressInformationCollection {
    pub fn Add(
        &mut self,
        address: quest_hook::libil2cpp::Gc<
            crate::System::Net::NetworkInformation::UnicastIPAddressInformation,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (address))?;
        Ok(__cordl_ret.into())
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Contains(
        &mut self,
        address: quest_hook::libil2cpp::Gc<
            crate::System::Net::NetworkInformation::UnicastIPAddressInformation,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Contains", (address))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyTo(
        &mut self,
        array: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Net::NetworkInformation::UnicastIPAddressInformation,
            >,
        >,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyTo", (array, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                *mut crate::System::Net::NetworkInformation::UnicastIPAddressInformation,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                *mut crate::System::Net::NetworkInformation::UnicastIPAddressInformation,
            >,
        > = __cordl_object.invoke("GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalAdd(
        &mut self,
        address: quest_hook::libil2cpp::Gc<
            crate::System::Net::NetworkInformation::UnicastIPAddressInformation,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalAdd", (address))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Remove(
        &mut self,
        address: quest_hook::libil2cpp::Gc<
            crate::System::Net::NetworkInformation::UnicastIPAddressInformation,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Remove", (address))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("System.Collections.IEnumerable.GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsReadOnly(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsReadOnly", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+NetworkInformation+UnicastIPAddressInformationCollection")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::NetworkInformation::UnicastIPAddressInformationCollection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+NetworkInformation+UnicastIPAddressInformationCollection")]
impl AsRef<
    crate::System::Collections::Generic::ICollection_1<
        *mut crate::System::Net::NetworkInformation::UnicastIPAddressInformation,
    >,
> for crate::System::Net::NetworkInformation::UnicastIPAddressInformationCollection {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::ICollection_1<
        *mut crate::System::Net::NetworkInformation::UnicastIPAddressInformation,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+UnicastIPAddressInformationCollection")]
impl AsMut<
    crate::System::Collections::Generic::ICollection_1<
        *mut crate::System::Net::NetworkInformation::UnicastIPAddressInformation,
    >,
> for crate::System::Net::NetworkInformation::UnicastIPAddressInformationCollection {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::ICollection_1<
        *mut crate::System::Net::NetworkInformation::UnicastIPAddressInformation,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+UnicastIPAddressInformationCollection")]
impl AsRef<
    crate::System::Collections::Generic::IEnumerable_1<
        *mut crate::System::Net::NetworkInformation::UnicastIPAddressInformation,
    >,
> for crate::System::Net::NetworkInformation::UnicastIPAddressInformationCollection {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEnumerable_1<
        *mut crate::System::Net::NetworkInformation::UnicastIPAddressInformation,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+UnicastIPAddressInformationCollection")]
impl AsMut<
    crate::System::Collections::Generic::IEnumerable_1<
        *mut crate::System::Net::NetworkInformation::UnicastIPAddressInformation,
    >,
> for crate::System::Net::NetworkInformation::UnicastIPAddressInformationCollection {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerable_1<
        *mut crate::System::Net::NetworkInformation::UnicastIPAddressInformation,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+UnicastIPAddressInformationCollection")]
impl AsRef<crate::System::Collections::IEnumerable>
for crate::System::Net::NetworkInformation::UnicastIPAddressInformationCollection {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+UnicastIPAddressInformationCollection")]
impl AsMut<crate::System::Collections::IEnumerable>
for crate::System::Net::NetworkInformation::UnicastIPAddressInformationCollection {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
