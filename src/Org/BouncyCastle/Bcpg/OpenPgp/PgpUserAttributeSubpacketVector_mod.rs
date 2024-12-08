#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpUserAttributeSubpacketVector")]
#[repr(C)]
#[derive(Debug)]
pub struct PgpUserAttributeSubpacketVector {
    __cordl_parent: crate::System::Object,
    pub packets: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::Org::BouncyCastle::Bcpg::UserAttributeSubpacket,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpUserAttributeSubpacketVector")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Bcpg::OpenPgp::PgpUserAttributeSubpacketVector =>
    "Org.BouncyCastle.Bcpg.OpenPgp"."PgpUserAttributeSubpacketVector"
);
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpUserAttributeSubpacketVector")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUserAttributeSubpacketVector {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpUserAttributeSubpacketVector")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUserAttributeSubpacketVector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpUserAttributeSubpacketVector")]
impl crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUserAttributeSubpacketVector {
    pub fn GetSubpacket(
        &mut self,
        _cordl_type: crate::Org::BouncyCastle::Bcpg::UserAttributeSubpacketTag,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Bcpg::UserAttributeSubpacket,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Bcpg::UserAttributeSubpacket = __cordl_object
            .invoke("GetSubpacket", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        packets: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Bcpg::UserAttributeSubpacket,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (packets))?;
        Ok(__cordl_ret)
    }
    pub fn ToSubpacketArray(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Bcpg::UserAttributeSubpacket,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Bcpg::UserAttributeSubpacket,
        > = __cordl_object.invoke("ToSubpacketArray", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn Equals(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn GetImageAttribute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Bcpg::Attr::ImageAttrib,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Bcpg::Attr::ImageAttrib = __cordl_object
            .invoke("GetImageAttribute", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        packets: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Bcpg::UserAttributeSubpacket,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (packets))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpUserAttributeSubpacketVector")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUserAttributeSubpacketVector {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
