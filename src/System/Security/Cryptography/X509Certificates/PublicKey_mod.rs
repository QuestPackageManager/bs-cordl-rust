#[cfg(feature = "System+Security+Cryptography+X509Certificates+PublicKey")]
#[repr(C)]
#[derive(Debug)]
pub struct PublicKey {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _keyValue: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::AsnEncodedData,
    >,
    pub _params: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::AsnEncodedData,
    >,
    pub _oid: quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::Oid>,
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+PublicKey")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Security::Cryptography::X509Certificates::PublicKey {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Security.Cryptography.X509Certificates";
    const CLASS_NAME: &'static str = "PublicKey";
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
#[cfg(feature = "System+Security+Cryptography+X509Certificates+PublicKey")]
impl std::ops::Deref
for crate::System::Security::Cryptography::X509Certificates::PublicKey {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+PublicKey")]
impl std::ops::DerefMut
for crate::System::Security::Cryptography::X509Certificates::PublicKey {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+PublicKey")]
impl crate::System::Security::Cryptography::X509Certificates::PublicKey {
    pub fn DecodeDSA(
        rawPublicKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        rawParameters: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::DSA>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::PublicKey as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::DSA>,
                2usize,
            >("DecodeDSA")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Security::Cryptography::X509Certificates::PublicKey
                    as quest_hook::libil2cpp::Type > ::class(), "DecodeDSA", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::DSA,
        > = unsafe { method.invoke_unchecked((), (rawPublicKey, rawParameters))? };
        Ok(__cordl_ret.into())
    }
    pub fn DecodeRSA(
        rawPublicKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::RSA>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::PublicKey as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>),
                quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::RSA>,
                1usize,
            >("DecodeRSA")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Security::Cryptography::X509Certificates::PublicKey
                    as quest_hook::libil2cpp::Type > ::class(), "DecodeRSA", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::RSA,
        > = unsafe { method.invoke_unchecked((), (rawPublicKey))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetUnsignedBigInteger(
        integer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::PublicKey as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                1usize,
            >("GetUnsignedBigInteger")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Security::Cryptography::X509Certificates::PublicKey
                    as quest_hook::libil2cpp::Type > ::class(), "GetUnsignedBigInteger",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked((), (integer))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        oid: quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::Oid>,
        parameters: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::AsnEncodedData,
        >,
        keyValue: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::AsnEncodedData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (oid, parameters, keyValue))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        oid: quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::Oid>,
        parameters: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::AsnEncodedData,
        >,
        keyValue: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::AsnEncodedData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::PublicKey as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Security::Cryptography::Oid,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Security::Cryptography::AsnEncodedData,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Security::Cryptography::AsnEncodedData,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Security::Cryptography::X509Certificates::PublicKey
                    as quest_hook::libil2cpp::Type > ::class(), ".ctor", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (oid, parameters, keyValue))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_EncodedKeyValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::AsnEncodedData>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::PublicKey as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Security::Cryptography::AsnEncodedData,
                >,
                0usize,
            >("get_EncodedKeyValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Security::Cryptography::X509Certificates::PublicKey
                    as quest_hook::libil2cpp::Type > ::class(), "get_EncodedKeyValue",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::AsnEncodedData,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_EncodedParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::AsnEncodedData>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::PublicKey as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Security::Cryptography::AsnEncodedData,
                >,
                0usize,
            >("get_EncodedParameters")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Security::Cryptography::X509Certificates::PublicKey
                    as quest_hook::libil2cpp::Type > ::class(), "get_EncodedParameters",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::AsnEncodedData,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Key(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::AsymmetricAlgorithm,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::PublicKey as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Security::Cryptography::AsymmetricAlgorithm,
                >,
                0usize,
            >("get_Key")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Security::Cryptography::X509Certificates::PublicKey
                    as quest_hook::libil2cpp::Type > ::class(), "get_Key", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::AsymmetricAlgorithm,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Oid(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::Oid>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::PublicKey as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::Oid>,
                0usize,
            >("get_Oid")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Security::Cryptography::X509Certificates::PublicKey
                    as quest_hook::libil2cpp::Type > ::class(), "get_Oid", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::Oid,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+PublicKey")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::X509Certificates::PublicKey {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
