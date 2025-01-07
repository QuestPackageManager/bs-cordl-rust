#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+CrlOcspRef")]
#[repr(C)]
#[derive(Debug)]
pub struct CrlOcspRef {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub crlids: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Esf::CrlListID,
    >,
    pub ocspids: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Esf::OcspListID,
    >,
    pub otherRev: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Esf::OtherRevRefs,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+CrlOcspRef")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::Esf::CrlOcspRef {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.Esf";
    const CLASS_NAME: &'static str = "CrlOcspRef";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+CrlOcspRef")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Esf::CrlOcspRef {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+CrlOcspRef")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Esf::CrlOcspRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+CrlOcspRef")]
impl crate::Org::BouncyCastle::Asn1::Esf::CrlOcspRef {
    pub fn GetInstance(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Esf::CrlOcspRef>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Esf::CrlOcspRef,
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
    pub fn New_CrlListID_OcspListID_OtherRevRefs1(
        crlids: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Esf::CrlListID,
        >,
        ocspids: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Esf::OcspListID,
        >,
        otherRev: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Esf::OtherRevRefs,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (crlids, ocspids, otherRev))?;
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
    pub fn _ctor_CrlListID_OcspListID_OtherRevRefs1(
        &mut self,
        crlids: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Esf::CrlListID,
        >,
        ocspids: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Esf::OcspListID,
        >,
        otherRev: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Esf::OtherRevRefs,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (crlids, ocspids, otherRev))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CrlIDs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Esf::CrlListID>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Esf::CrlListID,
        > = __cordl_object.invoke("get_CrlIDs", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OcspIDs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Esf::OcspListID>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Esf::OcspListID,
        > = __cordl_object.invoke("get_OcspIDs", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OtherRev(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Esf::OtherRevRefs>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Esf::OtherRevRefs,
        > = __cordl_object.invoke("get_OtherRev", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+CrlOcspRef")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Esf::CrlOcspRef {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
