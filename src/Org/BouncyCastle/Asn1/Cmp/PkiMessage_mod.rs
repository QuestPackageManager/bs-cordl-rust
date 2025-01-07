#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiMessage")]
#[repr(C)]
#[derive(Debug)]
pub struct PkiMessage {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub header: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Cmp::PkiHeader,
    >,
    pub body: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cmp::PkiBody>,
    pub protection: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::DerBitString,
    >,
    pub extraCerts: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiMessage")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::Cmp::PkiMessage {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.Cmp";
    const CLASS_NAME: &'static str = "PkiMessage";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiMessage")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Cmp::PkiMessage {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiMessage")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Cmp::PkiMessage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiMessage")]
impl crate::Org::BouncyCastle::Asn1::Cmp::PkiMessage {
    pub fn GetExtraCerts(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Asn1::Cmp::CmpCertificate,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Asn1::Cmp::CmpCertificate,
                >,
            >,
        > = __cordl_object.invoke("GetExtraCerts", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInstance(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cmp::PkiMessage>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiMessage,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetInstance", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Asn1Sequence0(
        seq: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Sequence>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object.into())
    }
    pub fn New_PkiHeader_PkiBody3(
        header: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiHeader,
        >,
        body: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cmp::PkiBody>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (header, body))?;
        Ok(__cordl_object.into())
    }
    pub fn New_PkiHeader_PkiBody_DerBitString2(
        header: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiHeader,
        >,
        body: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cmp::PkiBody>,
        protection: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerBitString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (header, body, protection))?;
        Ok(__cordl_object.into())
    }
    pub fn New_PkiHeader_PkiBody_DerBitString_Il2CppArray1(
        header: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiHeader,
        >,
        body: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cmp::PkiBody>,
        protection: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerBitString,
        >,
        extraCerts: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Asn1::Cmp::CmpCertificate,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (header, body, protection, extraCerts))?;
        Ok(__cordl_object.into())
    }
    pub fn ToAsn1Object(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Object>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1Object,
        > = __cordl_object.invoke("ToAsn1Object", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Asn1Sequence0(
        &mut self,
        seq: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Sequence>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (seq))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_PkiHeader_PkiBody3(
        &mut self,
        header: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiHeader,
        >,
        body: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cmp::PkiBody>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (header, body))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_PkiHeader_PkiBody_DerBitString2(
        &mut self,
        header: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiHeader,
        >,
        body: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cmp::PkiBody>,
        protection: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerBitString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (header, body, protection))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_PkiHeader_PkiBody_DerBitString_Il2CppArray1(
        &mut self,
        header: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiHeader,
        >,
        body: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cmp::PkiBody>,
        protection: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerBitString,
        >,
        extraCerts: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Asn1::Cmp::CmpCertificate,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (header, body, protection, extraCerts))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Body(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cmp::PkiBody>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiBody,
        > = __cordl_object.invoke("get_Body", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Header(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cmp::PkiHeader>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiHeader,
        > = __cordl_object.invoke("get_Header", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Protection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::DerBitString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerBitString,
        > = __cordl_object.invoke("get_Protection", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiMessage")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Cmp::PkiMessage {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
