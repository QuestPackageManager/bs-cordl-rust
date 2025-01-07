#[cfg(feature = "Org+BouncyCastle+Cms+OriginatorInfoGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct OriginatorInfoGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub origCerts: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    pub origCrls: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
}
#[cfg(feature = "Org+BouncyCastle+Cms+OriginatorInfoGenerator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Cms::OriginatorInfoGenerator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Cms";
    const CLASS_NAME: &'static str = "OriginatorInfoGenerator";
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
#[cfg(feature = "Org+BouncyCastle+Cms+OriginatorInfoGenerator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::OriginatorInfoGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+OriginatorInfoGenerator")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cms::OriginatorInfoGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+OriginatorInfoGenerator")]
impl crate::Org::BouncyCastle::Cms::OriginatorInfoGenerator {
    pub fn Generate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cms::OriginatorInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::OriginatorInfo,
        > = __cordl_object.invoke("Generate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_IX509Store1(
        origCerts: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::Store::IX509Store,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (origCerts))?;
        Ok(__cordl_object.into())
    }
    pub fn New_IX509Store_IX509Store2(
        origCerts: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::Store::IX509Store,
        >,
        origCrls: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::Store::IX509Store,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (origCerts, origCrls))?;
        Ok(__cordl_object.into())
    }
    pub fn New_X509Certificate0(
        origCert: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::X509Certificate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (origCert))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_IX509Store1(
        &mut self,
        origCerts: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::Store::IX509Store,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (origCerts))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IX509Store_IX509Store2(
        &mut self,
        origCerts: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::Store::IX509Store,
        >,
        origCrls: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::Store::IX509Store,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (origCerts, origCrls))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_X509Certificate0(
        &mut self,
        origCert: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::X509Certificate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (origCert))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+OriginatorInfoGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::OriginatorInfoGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
