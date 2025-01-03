#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X509SubjectKeyIdentifierExtension"
)]
#[repr(C)]
#[derive(Debug)]
pub struct X509SubjectKeyIdentifierExtension {
    __cordl_parent: crate::System::Security::Cryptography::X509Certificates::X509Extension,
    pub _subjectKeyIdentifier: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub _ski: *mut quest_hook::libil2cpp::Il2CppString,
    pub _status: crate::System::Security::Cryptography::AsnDecodeStatus,
}
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X509SubjectKeyIdentifierExtension"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::X509Certificates::X509SubjectKeyIdentifierExtension
    => "System.Security.Cryptography.X509Certificates"
    ."X509SubjectKeyIdentifierExtension"
);
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyFrom", (asnEncodedData))?;
        Ok(__cordl_ret.into())
    }
    pub fn Decode(
        &mut self,
        extension: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::Cryptography::AsnDecodeStatus,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Security::Cryptography::AsnDecodeStatus = __cordl_object
            .invoke("Decode", (extension))?;
        Ok(__cordl_ret.into())
    }
    pub fn Encode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("Encode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FromHex(
        hex: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("FromHex", (hex))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromHexChar(c: char) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromHexChar", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromHexChars(c1: char, c2: char) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromHexChars", (c1, c2))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", (multiLine))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_AsnEncodedData__cordl_bool1(
        &mut self,
        encodedSubjectKeyIdentifier: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::AsnEncodedData,
        >,
        critical: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (encodedSubjectKeyIdentifier, critical))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray__cordl_bool2(
        &mut self,
        subjectKeyIdentifier: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        critical: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (subjectKeyIdentifier, critical))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString__cordl_bool3(
        &mut self,
        subjectKeyIdentifier: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        critical: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (subjectKeyIdentifier, critical))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (key, algorithm, critical))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_PublicKey__cordl_bool4(
        &mut self,
        key: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::PublicKey,
        >,
        critical: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (key, critical))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SubjectKeyIdentifier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_SubjectKeyIdentifier", ())?;
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
