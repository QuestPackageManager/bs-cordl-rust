#[cfg(feature = "Org+BouncyCastle+Pkix+ReasonsMask")]
#[repr(C)]
#[derive(Debug)]
pub struct ReasonsMask {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _reasons: i32,
}
#[cfg(feature = "Org+BouncyCastle+Pkix+ReasonsMask")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Pkix::ReasonsMask =>
    "Org.BouncyCastle.Pkix"."ReasonsMask"
);
#[cfg(feature = "Org+BouncyCastle+Pkix+ReasonsMask")]
impl std::ops::Deref for crate::Org::BouncyCastle::Pkix::ReasonsMask {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+ReasonsMask")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Pkix::ReasonsMask {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+ReasonsMask")]
impl crate::Org::BouncyCastle::Pkix::ReasonsMask {
    pub fn AddReasons(
        &mut self,
        mask: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Pkix::ReasonsMask>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddReasons", (mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasNewReasons(
        &mut self,
        mask: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Pkix::ReasonsMask>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasNewReasons", (mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn Intersect(
        &mut self,
        mask: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Pkix::ReasonsMask>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Pkix::ReasonsMask>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::ReasonsMask,
        > = __cordl_object.invoke("Intersect", (mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_1() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_0(
        reasons: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (reasons))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_0(
        &mut self,
        reasons: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (reasons))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsAllReasons(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsAllReasons", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Reasons(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::ReasonFlags>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::ReasonFlags,
        > = __cordl_object.invoke("get_Reasons", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+ReasonsMask")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::Pkix::ReasonsMask {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
