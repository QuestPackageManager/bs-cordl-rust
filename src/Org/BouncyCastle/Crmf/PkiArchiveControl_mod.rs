#[cfg(feature = "Org+BouncyCastle+Crmf+PkiArchiveControl")]
#[repr(C)]
#[derive(Debug)]
pub struct PkiArchiveControl {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub pkiArchiveOptions: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Crmf::PkiArchiveOptions,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Crmf+PkiArchiveControl")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Crmf::PkiArchiveControl {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crmf";
    const CLASS_NAME: &'static str = "PkiArchiveControl";
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
#[cfg(feature = "Org+BouncyCastle+Crmf+PkiArchiveControl")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crmf::PkiArchiveControl {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+PkiArchiveControl")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crmf::PkiArchiveControl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+PkiArchiveControl")]
impl crate::Org::BouncyCastle::Crmf::PkiArchiveControl {
    pub fn GetEnvelopedData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::CmsEnvelopedData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsEnvelopedData,
        > = __cordl_object.invoke("GetEnvelopedData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        pkiArchiveOptions: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Crmf::PkiArchiveOptions,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pkiArchiveOptions))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        pkiArchiveOptions: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Crmf::PkiArchiveOptions,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (pkiArchiveOptions))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ArchiveType(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ArchiveType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_EnvelopedData(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_EnvelopedData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::DerObjectIdentifier>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        > = __cordl_object.invoke("get_Type", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Encodable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1Encodable,
        > = __cordl_object.invoke("get_Value", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+PkiArchiveControl")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crmf::PkiArchiveControl {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+PkiArchiveControl")]
impl AsRef<crate::Org::BouncyCastle::Crmf::IControl>
for crate::Org::BouncyCastle::Crmf::PkiArchiveControl {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crmf::IControl {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+PkiArchiveControl")]
impl AsMut<crate::Org::BouncyCastle::Crmf::IControl>
for crate::Org::BouncyCastle::Crmf::PkiArchiveControl {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crmf::IControl {
        unsafe { std::mem::transmute(self) }
    }
}
