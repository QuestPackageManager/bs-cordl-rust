#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509KeyUsageExtension")]
#[repr(C)]
#[derive(Debug)]
pub struct X509KeyUsageExtension {
    __cordl_parent: crate::System::Security::Cryptography::X509Certificates::X509Extension,
    pub _keyUsages: crate::System::Security::Cryptography::X509Certificates::X509KeyUsageFlags,
    pub _status: crate::System::Security::Cryptography::AsnDecodeStatus,
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509KeyUsageExtension")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Security::Cryptography::X509Certificates::X509KeyUsageExtension {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Security.Cryptography.X509Certificates";
    const CLASS_NAME: &'static str = "X509KeyUsageExtension";
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
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509KeyUsageExtension")]
impl std::ops::Deref
for crate::System::Security::Cryptography::X509Certificates::X509KeyUsageExtension {
    type Target = crate::System::Security::Cryptography::X509Certificates::X509Extension;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509KeyUsageExtension")]
impl std::ops::DerefMut
for crate::System::Security::Cryptography::X509Certificates::X509KeyUsageExtension {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509KeyUsageExtension")]
impl crate::System::Security::Cryptography::X509Certificates::X509KeyUsageExtension {
    pub const friendlyName: &'static str = "Key Usage";
    pub const oid: &'static str = "2.5.29.15";
    pub fn CopyFrom(
        &mut self,
        asnEncodedData: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::AsnEncodedData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Security::Cryptography::AsnEncodedData,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CopyFrom")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CopyFrom", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (asnEncodedData))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Decode(
        &mut self,
        extension: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::Cryptography::AsnDecodeStatus,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>),
                crate::System::Security::Cryptography::AsnDecodeStatus,
                1usize,
            >("Decode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Decode", 1usize
                )
            });
        let __cordl_ret: crate::System::Security::Cryptography::AsnDecodeStatus = unsafe {
            method.invoke_unchecked(self, (extension))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Encode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                0usize,
            >("Encode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Encode", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn GetValidFlags(
        &mut self,
        flags: crate::System::Security::Cryptography::X509Certificates::X509KeyUsageFlags,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::Cryptography::X509Certificates::X509KeyUsageFlags,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::Security::Cryptography::X509Certificates::X509KeyUsageFlags),
                crate::System::Security::Cryptography::X509Certificates::X509KeyUsageFlags,
                1usize,
            >("GetValidFlags")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetValidFlags", 1usize
                )
            });
        let __cordl_ret: crate::System::Security::Cryptography::X509Certificates::X509KeyUsageFlags = unsafe {
            method.invoke_unchecked(self, (flags))
        };
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_AsnEncodedData__cordl_bool1(
        encodedKeyUsage: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::AsnEncodedData,
        >,
        critical: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (encodedKeyUsage, critical))?;
        Ok(__cordl_object.into())
    }
    pub fn New_X509KeyUsageFlags__cordl_bool2(
        keyUsages: crate::System::Security::Cryptography::X509Certificates::X509KeyUsageFlags,
        critical: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (keyUsages, critical))?;
        Ok(__cordl_object.into())
    }
    pub fn ToString(
        &mut self,
        multiLine: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (multiLine)) };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
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
    pub fn _ctor_AsnEncodedData__cordl_bool1(
        &mut self,
        encodedKeyUsage: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::AsnEncodedData,
        >,
        critical: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Security::Cryptography::AsnEncodedData,
                    >,
                    bool,
                ),
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
            method.invoke_unchecked(self, (encodedKeyUsage, critical))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_X509KeyUsageFlags__cordl_bool2(
        &mut self,
        keyUsages: crate::System::Security::Cryptography::X509Certificates::X509KeyUsageFlags,
        critical: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::System::Security::Cryptography::X509Certificates::X509KeyUsageFlags,
                    bool,
                ),
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
            method.invoke_unchecked(self, (keyUsages, critical))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_KeyUsages(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::Cryptography::X509Certificates::X509KeyUsageFlags,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::System::Security::Cryptography::X509Certificates::X509KeyUsageFlags,
                0usize,
            >("get_KeyUsages")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_KeyUsages", 0usize
                )
            });
        let __cordl_ret: crate::System::Security::Cryptography::X509Certificates::X509KeyUsageFlags = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509KeyUsageExtension")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::X509Certificates::X509KeyUsageExtension {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
