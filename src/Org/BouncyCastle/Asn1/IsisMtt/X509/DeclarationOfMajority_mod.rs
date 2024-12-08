#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+X509+DeclarationOfMajority+Choice")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclarationOfMajority_Choice {
    DateOfBirth = 2i32,
    FullAgeAtCountry = 1i32,
    NotYoungerThan = 0i32,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+X509+DeclarationOfMajority+Choice")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::IsisMtt::X509::DeclarationOfMajority_Choice =>
    "Org.BouncyCastle.Asn1.IsisMtt.X509"."DeclarationOfMajority/Choice"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+X509+DeclarationOfMajority")]
#[repr(C)]
#[derive(Debug)]
pub struct DeclarationOfMajority {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub declaration: *mut crate::Org::BouncyCastle::Asn1::Asn1TaggedObject,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+X509+DeclarationOfMajority")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::IsisMtt::X509::DeclarationOfMajority =>
    "Org.BouncyCastle.Asn1.IsisMtt.X509"."DeclarationOfMajority"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+X509+DeclarationOfMajority")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::IsisMtt::X509::DeclarationOfMajority {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+X509+DeclarationOfMajority")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::IsisMtt::X509::DeclarationOfMajority {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+X509+DeclarationOfMajority")]
impl crate::Org::BouncyCastle::Asn1::IsisMtt::X509::DeclarationOfMajority {
    #[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+X509+DeclarationOfMajority+Choice")]
    pub type Choice = crate::Org::BouncyCastle::Asn1::IsisMtt::X509::DeclarationOfMajority_Choice;
    pub fn New_Asn1TaggedObject3(
        o: *mut crate::Org::BouncyCastle::Asn1::Asn1TaggedObject,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (o))?;
        Ok(__cordl_object)
    }
    pub fn New_DerGeneralizedTime2(
        dateOfBirth: *mut crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dateOfBirth))?;
        Ok(__cordl_object)
    }
    pub fn New__cordl_bool_String1(
        fullAge: bool,
        country: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (fullAge, country))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_0(notYoungerThan: i32) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (notYoungerThan))?;
        Ok(__cordl_object)
    }
    pub fn ToAsn1Object(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::Asn1Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Object = __cordl_object
            .invoke("ToAsn1Object", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Asn1TaggedObject3(
        &mut self,
        o: *mut crate::Org::BouncyCastle::Asn1::Asn1TaggedObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (o))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DerGeneralizedTime2(
        &mut self,
        dateOfBirth: *mut crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (dateOfBirth))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor__cordl_bool_String1(
        &mut self,
        fullAge: bool,
        country: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (fullAge, country))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_0(
        &mut self,
        notYoungerThan: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (notYoungerThan))?;
        Ok(__cordl_ret)
    }
    pub fn get_DateOfBirth(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerGeneralizedTime = __cordl_object
            .invoke("get_DateOfBirth", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_FullAgeAtCountry(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence = __cordl_object
            .invoke("get_FullAgeAtCountry", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_NotYoungerThan(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_NotYoungerThan", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Org::BouncyCastle::Asn1::IsisMtt::X509::DeclarationOfMajority_Choice,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Org::BouncyCastle::Asn1::IsisMtt::X509::DeclarationOfMajority_Choice = __cordl_object
            .invoke("get_Type", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+X509+DeclarationOfMajority")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::IsisMtt::X509::DeclarationOfMajority {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
