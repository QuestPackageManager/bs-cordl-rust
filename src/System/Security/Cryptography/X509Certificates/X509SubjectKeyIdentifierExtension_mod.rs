#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X509SubjectKeyIdentifierExtension"
)]
#[repr(C)]
#[derive(Debug)]
pub struct X509SubjectKeyIdentifierExtension {
    __cordl_parent: crate::System::Security::Cryptography::X509Certificates::X509Extension,
    pub _subjectKeyIdentifier: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<u8>,
    >,
    pub _ski: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _status: crate::System::Security::Cryptography::AsnDecodeStatus,
}
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X509SubjectKeyIdentifierExtension"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Security::Cryptography::X509Certificates::X509SubjectKeyIdentifierExtension {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Security.Cryptography.X509Certificates";
    const CLASS_NAME: &'static str = "X509SubjectKeyIdentifierExtension";
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
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X509SubjectKeyIdentifierExtension"
)]
impl std::ops::Deref
for crate::System::Security::Cryptography::X509Certificates::X509SubjectKeyIdentifierExtension {
    type Target = crate::System::Security::Cryptography::X509Certificates::X509Extension;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X509SubjectKeyIdentifierExtension"
)]
impl std::ops::DerefMut
for crate::System::Security::Cryptography::X509Certificates::X509SubjectKeyIdentifierExtension {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X509SubjectKeyIdentifierExtension"
)]
impl crate::System::Security::Cryptography::X509Certificates::X509SubjectKeyIdentifierExtension {
    pub const friendlyName: &'static str = "Subject Key Identifier";
    pub const oid: &'static str = "2.5.29.14";
    pub fn CopyFrom(
        &mut self,
        asnEncodedData: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::AsnEncodedData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509SubjectKeyIdentifierExtension as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Security::Cryptography::AsnEncodedData,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CopyFrom")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509SubjectKeyIdentifierExtension
                    as quest_hook::libil2cpp::Type > ::class(), "CopyFrom", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (asnEncodedData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Decode(
        &mut self,
        extension: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::Cryptography::AsnDecodeStatus,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509SubjectKeyIdentifierExtension as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>),
                crate::System::Security::Cryptography::AsnDecodeStatus,
                1usize,
            >("Decode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509SubjectKeyIdentifierExtension
                    as quest_hook::libil2cpp::Type > ::class(), "Decode", 1usize
                )
            });
        let __cordl_ret: crate::System::Security::Cryptography::AsnDecodeStatus = unsafe {
            method.invoke_unchecked(self, (extension))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Encode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509SubjectKeyIdentifierExtension as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                0usize,
            >("Encode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509SubjectKeyIdentifierExtension
                    as quest_hook::libil2cpp::Type > ::class(), "Encode", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn FromHex(
        hex: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509SubjectKeyIdentifierExtension as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                1usize,
            >("FromHex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509SubjectKeyIdentifierExtension
                    as quest_hook::libil2cpp::Type > ::class(), "FromHex", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked((), (hex))? };
        Ok(__cordl_ret.into())
    }
    pub fn FromHexChar(c: char) -> quest_hook::libil2cpp::Result<u8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509SubjectKeyIdentifierExtension as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(char), u8, 1usize>("FromHexChar")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509SubjectKeyIdentifierExtension
                    as quest_hook::libil2cpp::Type > ::class(), "FromHexChar", 1usize
                )
            });
        let __cordl_ret: u8 = unsafe { method.invoke_unchecked((), (c))? };
        Ok(__cordl_ret.into())
    }
    pub fn FromHexChars(c1: char, c2: char) -> quest_hook::libil2cpp::Result<u8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509SubjectKeyIdentifierExtension as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(char, char), u8, 2usize>("FromHexChars")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509SubjectKeyIdentifierExtension
                    as quest_hook::libil2cpp::Type > ::class(), "FromHexChars", 2usize
                )
            });
        let __cordl_ret: u8 = unsafe { method.invoke_unchecked((), (c1, c2))? };
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
        encodedSubjectKeyIdentifier: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::AsnEncodedData,
        >,
        critical: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (encodedSubjectKeyIdentifier, critical))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppArray__cordl_bool2(
        subjectKeyIdentifier: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        critical: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (subjectKeyIdentifier, critical))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString__cordl_bool3(
        subjectKeyIdentifier: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        critical: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (subjectKeyIdentifier, critical))?;
        Ok(__cordl_object.into())
    }
    pub fn New_PublicKey_X509SubjectKeyIdentifierHashAlgorithm__cordl_bool5(
        key: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::PublicKey,
        >,
        algorithm: crate::System::Security::Cryptography::X509Certificates::X509SubjectKeyIdentifierHashAlgorithm,
        critical: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (key, algorithm, critical))?;
        Ok(__cordl_object.into())
    }
    pub fn New_PublicKey__cordl_bool4(
        key: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::PublicKey,
        >,
        critical: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (key, critical))?;
        Ok(__cordl_object.into())
    }
    pub fn ToString(
        &mut self,
        multiLine: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509SubjectKeyIdentifierExtension as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509SubjectKeyIdentifierExtension
                    as quest_hook::libil2cpp::Type > ::class(), "ToString", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (multiLine))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509SubjectKeyIdentifierExtension as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509SubjectKeyIdentifierExtension
                    as quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_AsnEncodedData__cordl_bool1(
        &mut self,
        encodedSubjectKeyIdentifier: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::AsnEncodedData,
        >,
        critical: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509SubjectKeyIdentifierExtension as quest_hook::libil2cpp::Type>::class()
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
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509SubjectKeyIdentifierExtension
                    as quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (encodedSubjectKeyIdentifier, critical))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray__cordl_bool2(
        &mut self,
        subjectKeyIdentifier: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        critical: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509SubjectKeyIdentifierExtension as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509SubjectKeyIdentifierExtension
                    as quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (subjectKeyIdentifier, critical))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString__cordl_bool3(
        &mut self,
        subjectKeyIdentifier: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        critical: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509SubjectKeyIdentifierExtension as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509SubjectKeyIdentifierExtension
                    as quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (subjectKeyIdentifier, critical))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_PublicKey_X509SubjectKeyIdentifierHashAlgorithm__cordl_bool5(
        &mut self,
        key: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::PublicKey,
        >,
        algorithm: crate::System::Security::Cryptography::X509Certificates::X509SubjectKeyIdentifierHashAlgorithm,
        critical: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509SubjectKeyIdentifierExtension as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Security::Cryptography::X509Certificates::PublicKey,
                    >,
                    crate::System::Security::Cryptography::X509Certificates::X509SubjectKeyIdentifierHashAlgorithm,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509SubjectKeyIdentifierExtension
                    as quest_hook::libil2cpp::Type > ::class(), ".ctor", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (key, algorithm, critical))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_PublicKey__cordl_bool4(
        &mut self,
        key: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::PublicKey,
        >,
        critical: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509SubjectKeyIdentifierExtension as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Security::Cryptography::X509Certificates::PublicKey,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509SubjectKeyIdentifierExtension
                    as quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (key, critical))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_SubjectKeyIdentifier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509SubjectKeyIdentifierExtension as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_SubjectKeyIdentifier")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509SubjectKeyIdentifierExtension
                    as quest_hook::libil2cpp::Type > ::class(),
                    "get_SubjectKeyIdentifier", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X509SubjectKeyIdentifierExtension"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::X509Certificates::X509SubjectKeyIdentifierExtension {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
