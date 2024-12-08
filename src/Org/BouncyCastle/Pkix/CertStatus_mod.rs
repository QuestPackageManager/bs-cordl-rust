#[cfg(feature = "Org+BouncyCastle+Pkix+CertStatus")]
#[repr(C)]
#[derive(Debug)]
pub struct CertStatus {
    __cordl_parent: crate::System::Object,
    pub status: i32,
    pub revocationDate: *mut crate::Org::BouncyCastle::Utilities::Date::DateTimeObject,
}
#[cfg(feature = "Org+BouncyCastle+Pkix+CertStatus")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Pkix::CertStatus =>
    "Org.BouncyCastle.Pkix"."CertStatus"
);
#[cfg(feature = "Org+BouncyCastle+Pkix+CertStatus")]
impl std::ops::Deref for crate::Org::BouncyCastle::Pkix::CertStatus {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+CertStatus")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Pkix::CertStatus {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+CertStatus")]
impl crate::Org::BouncyCastle::Pkix::CertStatus {
    pub const Undetermined: i32 = 12i32;
    pub const Unrevoked: i32 = 11i32;
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
    pub fn get_Status(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Status", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_RevocationDate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Utilities::Date::DateTimeObject,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Utilities::Date::DateTimeObject = __cordl_object
            .invoke("get_RevocationDate", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_RevocationDate(
        &mut self,
        value: *mut crate::Org::BouncyCastle::Utilities::Date::DateTimeObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_RevocationDate", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Status(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Status", (value))?;
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
#[cfg(feature = "Org+BouncyCastle+Pkix+CertStatus")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::Pkix::CertStatus {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
