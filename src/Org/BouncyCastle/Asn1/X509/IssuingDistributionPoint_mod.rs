#[cfg(feature = "Org+BouncyCastle+Asn1+X509+IssuingDistributionPoint")]
#[repr(C)]
#[derive(Debug)]
pub struct IssuingDistributionPoint {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub _distributionPoint: *mut crate::Org::BouncyCastle::Asn1::X509::DistributionPointName,
    pub _onlyContainsUserCerts: bool,
    pub _onlyContainsCACerts: bool,
    pub _onlySomeReasons: *mut crate::Org::BouncyCastle::Asn1::X509::ReasonFlags,
    pub _indirectCRL: bool,
    pub _onlyContainsAttributeCerts: bool,
    pub seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+IssuingDistributionPoint")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::X509::IssuingDistributionPoint =>
    "Org.BouncyCastle.Asn1.X509"."IssuingDistributionPoint"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+IssuingDistributionPoint")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::X509::IssuingDistributionPoint {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+IssuingDistributionPoint")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::X509::IssuingDistributionPoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+IssuingDistributionPoint")]
impl crate::Org::BouncyCastle::Asn1::X509::IssuingDistributionPoint {
    pub fn New_Asn1Sequence1(
        seq: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Sequence>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object.into())
    }
    pub fn New_DistributionPointName__cordl_bool__cordl_bool_ReasonFlags__cordl_bool__cordl_bool0(
        distributionPoint: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::DistributionPointName,
        >,
        onlyContainsUserCerts: bool,
        onlyContainsCACerts: bool,
        onlySomeReasons: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::ReasonFlags,
        >,
        indirectCRL: bool,
        onlyContainsAttributeCerts: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    distributionPoint,
                    onlyContainsUserCerts,
                    onlyContainsCACerts,
                    onlySomeReasons,
                    indirectCRL,
                    onlyContainsAttributeCerts,
                ),
            )?;
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
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Asn1Sequence1(
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
    pub fn _ctor_DistributionPointName__cordl_bool__cordl_bool_ReasonFlags__cordl_bool__cordl_bool0(
        &mut self,
        distributionPoint: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::DistributionPointName,
        >,
        onlyContainsUserCerts: bool,
        onlyContainsCACerts: bool,
        onlySomeReasons: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::ReasonFlags,
        >,
        indirectCRL: bool,
        onlyContainsAttributeCerts: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    distributionPoint,
                    onlyContainsUserCerts,
                    onlyContainsCACerts,
                    onlySomeReasons,
                    indirectCRL,
                    onlyContainsAttributeCerts,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn appendObject(
        &mut self,
        buf: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        sep: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        val: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("appendObject", (buf, sep, name, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DistributionPoint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::DistributionPointName,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::DistributionPointName,
        > = __cordl_object.invoke("get_DistributionPoint", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsIndirectCrl(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsIndirectCrl", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OnlyContainsAttributeCerts(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_OnlyContainsAttributeCerts", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OnlyContainsCACerts(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_OnlyContainsCACerts", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OnlyContainsUserCerts(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_OnlyContainsUserCerts", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OnlySomeReasons(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::ReasonFlags>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::ReasonFlags,
        > = __cordl_object.invoke("get_OnlySomeReasons", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+IssuingDistributionPoint")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::X509::IssuingDistributionPoint {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
