#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpSignatureList")]
#[repr(C)]
#[derive(Debug)]
pub struct PgpSignatureList {
    __cordl_parent: crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpObject,
    pub sigs: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignature,
        >,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpSignatureList")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureList =>
    "Org.BouncyCastle.Bcpg.OpenPgp"."PgpSignatureList"
);
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpSignatureList")]
impl std::ops::Deref for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureList {
    type Target = crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpSignatureList")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpSignatureList")]
impl crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureList {
    pub fn Get(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignature>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignature,
        > = __cordl_object.invoke("Get", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Il2CppArray0(
        sigs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignature,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sigs))?;
        Ok(__cordl_object.into())
    }
    pub fn New_PgpSignature1(
        sig: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignature,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sig))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_Il2CppArray0(
        &mut self,
        sigs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignature,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sigs))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_PgpSignature1(
        &mut self,
        sig: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignature,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sig))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsEmpty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignature>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignature,
        > = __cordl_object.invoke("get_Item", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Size(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Size", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpSignatureList")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureList {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
