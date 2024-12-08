#[cfg(feature = "Org+BouncyCastle+OpenSsl+MiscPemGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct MiscPemGenerator {
    __cordl_parent: crate::System::Object,
    pub obj: *mut crate::System::Object,
    pub algorithm: *mut crate::System::String,
    pub password: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    pub random: *mut crate::Org::BouncyCastle::Security::SecureRandom,
}
#[cfg(feature = "Org+BouncyCastle+OpenSsl+MiscPemGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::OpenSsl::MiscPemGenerator =>
    "Org.BouncyCastle.OpenSsl"."MiscPemGenerator"
);
#[cfg(feature = "Org+BouncyCastle+OpenSsl+MiscPemGenerator")]
impl std::ops::Deref for crate::Org::BouncyCastle::OpenSsl::MiscPemGenerator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+OpenSsl+MiscPemGenerator")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::OpenSsl::MiscPemGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+OpenSsl+MiscPemGenerator")]
impl crate::Org::BouncyCastle::OpenSsl::MiscPemGenerator {
    pub fn _ctor_Object0(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_Il2CppArray_SecureRandom1(
        &mut self,
        obj: *mut crate::System::Object,
        algorithm: *mut crate::System::String,
        password: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        random: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (obj, algorithm, password, random))?;
        Ok(__cordl_ret)
    }
    pub fn Generate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Utilities::IO::Pem::PemObject,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Utilities::IO::Pem::PemObject = __cordl_object
            .invoke("Generate", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_Object0(
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (obj))?;
        Ok(__cordl_object)
    }
    pub fn New_String_Il2CppArray_SecureRandom1(
        obj: *mut crate::System::Object,
        algorithm: *mut crate::System::String,
        password: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        random: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (obj, algorithm, password, random))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+OpenSsl+MiscPemGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::OpenSsl::MiscPemGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
