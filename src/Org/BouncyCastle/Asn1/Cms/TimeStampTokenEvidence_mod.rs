#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+TimeStampTokenEvidence")]
#[repr(C)]
#[derive(Debug)]
pub struct TimeStampTokenEvidence {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub timeStampAndCrls: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Asn1::Cms::TimeStampAndCrl,
        >,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+TimeStampTokenEvidence")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::Cms::TimeStampTokenEvidence => "Org.BouncyCastle.Asn1.Cms"
    ."TimeStampTokenEvidence"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+TimeStampTokenEvidence")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Cms::TimeStampTokenEvidence {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+TimeStampTokenEvidence")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Cms::TimeStampTokenEvidence {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+TimeStampTokenEvidence")]
impl crate::Org::BouncyCastle::Asn1::Cms::TimeStampTokenEvidence {
    pub fn GetInstance_Asn1TaggedObject__cordl_bool0(
        tagged: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1TaggedObject,
        >,
        isExplicit: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::TimeStampTokenEvidence,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::TimeStampTokenEvidence,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInstance", (tagged, isExplicit))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInstance_Il2CppObject1(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::TimeStampTokenEvidence,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::TimeStampTokenEvidence,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetInstance", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Asn1Sequence2(
        seq: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Sequence>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppArray0(
        timeStampAndCrls: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Asn1::Cms::TimeStampAndCrl,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (timeStampAndCrls))?;
        Ok(__cordl_object.into())
    }
    pub fn New_TimeStampAndCrl1(
        timeStampAndCrl: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::TimeStampAndCrl,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (timeStampAndCrl))?;
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
    pub fn ToTimeStampAndCrlArray(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Asn1::Cms::TimeStampAndCrl,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Asn1::Cms::TimeStampAndCrl,
            >,
        > = __cordl_object.invoke("ToTimeStampAndCrlArray", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Asn1Sequence2(
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
    pub fn _ctor_Il2CppArray0(
        &mut self,
        timeStampAndCrls: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Asn1::Cms::TimeStampAndCrl,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (timeStampAndCrls))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TimeStampAndCrl1(
        &mut self,
        timeStampAndCrl: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::TimeStampAndCrl,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (timeStampAndCrl))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+TimeStampTokenEvidence")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Cms::TimeStampTokenEvidence {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
