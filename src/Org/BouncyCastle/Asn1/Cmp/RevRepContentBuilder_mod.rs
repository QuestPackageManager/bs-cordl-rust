#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+RevRepContentBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct RevRepContentBuilder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub status: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Asn1EncodableVector,
    >,
    pub revCerts: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Asn1EncodableVector,
    >,
    pub crls: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Asn1EncodableVector,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+RevRepContentBuilder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::Cmp::RevRepContentBuilder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.Cmp";
    const CLASS_NAME: &'static str = "RevRepContentBuilder";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+RevRepContentBuilder")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Cmp::RevRepContentBuilder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+RevRepContentBuilder")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Cmp::RevRepContentBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+RevRepContentBuilder")]
impl crate::Org::BouncyCastle::Asn1::Cmp::RevRepContentBuilder {
    pub fn AddCrl(
        &mut self,
        crl: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::CertificateList,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::RevRepContentBuilder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::RevRepContentBuilder,
        > = __cordl_object.invoke("AddCrl", (crl))?;
        Ok(__cordl_ret.into())
    }
    pub fn Add_CertId1(
        &mut self,
        status: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiStatusInfo,
        >,
        certId: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Crmf::CertId>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::RevRepContentBuilder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::RevRepContentBuilder,
        > = __cordl_object.invoke("Add", (status, certId))?;
        Ok(__cordl_ret.into())
    }
    pub fn Add_PkiStatusInfo0(
        &mut self,
        status: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiStatusInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::RevRepContentBuilder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::RevRepContentBuilder,
        > = __cordl_object.invoke("Add", (status))?;
        Ok(__cordl_ret.into())
    }
    pub fn Build(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cmp::RevRepContent>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::RevRepContent,
        > = __cordl_object.invoke("Build", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+RevRepContentBuilder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Cmp::RevRepContentBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
