#[cfg(feature = "Org+BouncyCastle+Asn1+X509+Qualified+Iso4217CurrencyCode")]
#[repr(C)]
#[derive(Debug)]
pub struct Iso4217CurrencyCode {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub obj: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+Qualified+Iso4217CurrencyCode")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::X509::Qualified::Iso4217CurrencyCode =>
    "Org.BouncyCastle.Asn1.X509.Qualified"."Iso4217CurrencyCode"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+Qualified+Iso4217CurrencyCode")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::X509::Qualified::Iso4217CurrencyCode {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+Qualified+Iso4217CurrencyCode")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::X509::Qualified::Iso4217CurrencyCode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+Qualified+Iso4217CurrencyCode")]
impl crate::Org::BouncyCastle::Asn1::X509::Qualified::Iso4217CurrencyCode {
    pub const AlphabeticMaxSize: i32 = 3i32;
    pub const NumericMaxSize: i32 = 999i32;
    pub const NumericMinSize: i32 = 1i32;
    pub fn get_Numeric(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Numeric", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsAlphabetic(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsAlphabetic", ())?;
        Ok(__cordl_ret)
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
    pub fn _ctor_i32_0(
        &mut self,
        numeric: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (numeric))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String1(
        &mut self,
        alphabetic: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (alphabetic))?;
        Ok(__cordl_ret)
    }
    pub fn get_Alphabetic(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Alphabetic", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_i32_0(numeric: i32) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (numeric))?;
        Ok(__cordl_object)
    }
    pub fn New_String1(
        alphabetic: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (alphabetic))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+Qualified+Iso4217CurrencyCode")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::X509::Qualified::Iso4217CurrencyCode {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
