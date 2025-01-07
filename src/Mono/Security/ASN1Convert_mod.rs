#[cfg(feature = "Mono+Security+ASN1Convert")]
#[repr(C)]
#[derive(Debug)]
pub struct ASN1Convert {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+Security+ASN1Convert")]
unsafe impl quest_hook::libil2cpp::Type for crate::Mono::Security::ASN1Convert {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Security";
    const CLASS_NAME: &'static str = "ASN1Convert";
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
#[cfg(feature = "Mono+Security+ASN1Convert")]
impl std::ops::Deref for crate::Mono::Security::ASN1Convert {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+ASN1Convert")]
impl std::ops::DerefMut for crate::Mono::Security::ASN1Convert {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+ASN1Convert")]
impl crate::Mono::Security::ASN1Convert {
    pub fn FromInt32(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Security::ASN1>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Mono::Security::ASN1> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromInt32", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromOid(
        oid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Security::ASN1>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Mono::Security::ASN1> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromOid", (oid))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromUnsignedBigInteger(
        big: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Security::ASN1>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Mono::Security::ASN1> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromUnsignedBigInteger", (big))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDateTime(
        _cordl_time: quest_hook::libil2cpp::Gc<crate::Mono::Security::ASN1>,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToDateTime", (_cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt32(
        asn1: quest_hook::libil2cpp::Gc<crate::Mono::Security::ASN1>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt32", (asn1))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToOid(
        asn1: quest_hook::libil2cpp::Gc<crate::Mono::Security::ASN1>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToOid", (asn1))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Security+ASN1Convert")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Security::ASN1Convert {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
