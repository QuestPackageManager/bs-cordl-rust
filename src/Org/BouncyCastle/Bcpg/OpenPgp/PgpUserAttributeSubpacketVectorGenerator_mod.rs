#[cfg(
    feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpUserAttributeSubpacketVectorGenerator"
)]
#[repr(C)]
#[derive(Debug)]
pub struct PgpUserAttributeSubpacketVectorGenerator {
    __cordl_parent: crate::System::Object,
    pub list: *mut crate::System::Collections::IList,
}
#[cfg(
    feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpUserAttributeSubpacketVectorGenerator"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Bcpg::OpenPgp::PgpUserAttributeSubpacketVectorGenerator =>
    "Org.BouncyCastle.Bcpg.OpenPgp"."PgpUserAttributeSubpacketVectorGenerator"
);
#[cfg(
    feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpUserAttributeSubpacketVectorGenerator"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUserAttributeSubpacketVectorGenerator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpUserAttributeSubpacketVectorGenerator"
)]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUserAttributeSubpacketVectorGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpUserAttributeSubpacketVectorGenerator"
)]
impl crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUserAttributeSubpacketVectorGenerator {
    pub fn Generate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUserAttributeSubpacketVector,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUserAttributeSubpacketVector = __cordl_object
            .invoke("Generate", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetImageAttribute(
        &mut self,
        imageType: crate::Org::BouncyCastle::Bcpg::Attr::ImageAttrib_Format,
        imageData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetImageAttribute", (imageType, imageData))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpUserAttributeSubpacketVectorGenerator"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUserAttributeSubpacketVectorGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
