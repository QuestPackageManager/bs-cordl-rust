#[cfg(feature = "System+Net+NetworkInformation+UnicastIPAddressInformationCollection")]
#[repr(C)]
#[derive(Debug)]
pub struct UnicastIPAddressInformationCollection {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub addresses: quest_hook::libil2cpp::Gc<
        crate::System::Collections::ObjectModel::Collection_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Net::NetworkInformation::UnicastIPAddressInformation,
            >,
        >,
    >,
}
#[cfg(feature = "System+Net+NetworkInformation+UnicastIPAddressInformationCollection")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Net::NetworkInformation::UnicastIPAddressInformationCollection {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net.NetworkInformation";
    const CLASS_NAME: &'static str = "UnicastIPAddressInformationCollection";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Net::NetworkInformation::UnicastIPAddressInformation,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Add")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Add", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (address))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Clear")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Clear", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn Contains(
        &mut self,
        address: quest_hook::libil2cpp::Gc<
            crate::System::Net::NetworkInformation::UnicastIPAddressInformation,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Net::NetworkInformation::UnicastIPAddressInformation,
                >),
                bool,
                1usize,
            >("Contains")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Contains", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (address)) };
        Ok(__cordl_ret.into())
    }
    pub fn CopyTo(
        &mut self,
        array: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::System::Net::NetworkInformation::UnicastIPAddressInformation,
                >,
            >,
        >,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Net::NetworkInformation::UnicastIPAddressInformation,
                            >,
                        >,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("CopyTo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CopyTo", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (array, offset))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Net::NetworkInformation::UnicastIPAddressInformation,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IEnumerator_1<
                        quest_hook::libil2cpp::Gc<
                            crate::System::Net::NetworkInformation::UnicastIPAddressInformation,
                        >,
                    >,
                >,
                0usize,
            >("GetEnumerator")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetEnumerator", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Net::NetworkInformation::UnicastIPAddressInformation,
                >,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn InternalAdd(
        &mut self,
        address: quest_hook::libil2cpp::Gc<
            crate::System::Net::NetworkInformation::UnicastIPAddressInformation,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Net::NetworkInformation::UnicastIPAddressInformation,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("InternalAdd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InternalAdd", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (address))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Net::NetworkInformation::UnicastIPAddressInformation,
                >),
                bool,
                1usize,
            >("Remove")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Remove", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (address)) };
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
                0usize,
            >("System.Collections.IEnumerable.GetEnumerator")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "System.Collections.IEnumerable.GetEnumerator", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_Count")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Count", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsReadOnly(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_IsReadOnly")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_IsReadOnly", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
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
        quest_hook::libil2cpp::Gc<
            crate::System::Net::NetworkInformation::UnicastIPAddressInformation,
        >,
    >,
> for crate::System::Net::NetworkInformation::UnicastIPAddressInformationCollection {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::ICollection_1<
        quest_hook::libil2cpp::Gc<
            crate::System::Net::NetworkInformation::UnicastIPAddressInformation,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+UnicastIPAddressInformationCollection")]
impl AsMut<
    crate::System::Collections::Generic::ICollection_1<
        quest_hook::libil2cpp::Gc<
            crate::System::Net::NetworkInformation::UnicastIPAddressInformation,
        >,
    >,
> for crate::System::Net::NetworkInformation::UnicastIPAddressInformationCollection {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::ICollection_1<
        quest_hook::libil2cpp::Gc<
            crate::System::Net::NetworkInformation::UnicastIPAddressInformation,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+UnicastIPAddressInformationCollection")]
impl AsRef<
    crate::System::Collections::Generic::IEnumerable_1<
        quest_hook::libil2cpp::Gc<
            crate::System::Net::NetworkInformation::UnicastIPAddressInformation,
        >,
    >,
> for crate::System::Net::NetworkInformation::UnicastIPAddressInformationCollection {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEnumerable_1<
        quest_hook::libil2cpp::Gc<
            crate::System::Net::NetworkInformation::UnicastIPAddressInformation,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+UnicastIPAddressInformationCollection")]
impl AsMut<
    crate::System::Collections::Generic::IEnumerable_1<
        quest_hook::libil2cpp::Gc<
            crate::System::Net::NetworkInformation::UnicastIPAddressInformation,
        >,
    >,
> for crate::System::Net::NetworkInformation::UnicastIPAddressInformationCollection {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerable_1<
        quest_hook::libil2cpp::Gc<
            crate::System::Net::NetworkInformation::UnicastIPAddressInformation,
        >,
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
