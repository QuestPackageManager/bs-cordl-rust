#[cfg(feature = "Org+BouncyCastle+Asn1+X509+DisplayText")]
#[repr(C)]
#[derive(Debug)]
pub struct DisplayText {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub contentType: i32,
    pub contents: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::IAsn1String>,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+DisplayText")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::X509::DisplayText =>
    "Org.BouncyCastle.Asn1.X509"."DisplayText"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+DisplayText")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::X509::DisplayText {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+DisplayText")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::X509::DisplayText {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+DisplayText")]
impl crate::Org::BouncyCastle::Asn1::X509::DisplayText {
    pub const ContentTypeBmpString: i32 = 1i32;
    pub const ContentTypeIA5String: i32 = 0i32;
    pub const ContentTypeUtf8String: i32 = 2i32;
    pub const ContentTypeVisibleString: i32 = 3i32;
    pub const DisplayTextMaximumSize: i32 = 200i32;
    pub fn GetInstance(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::DisplayText>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::DisplayText,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetInstance", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_IAsn1String2(
        contents: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::IAsn1String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (contents))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString1(
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (text))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_Il2CppString0(
        _cordl_type: i32,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, text))?;
        Ok(__cordl_object.into())
    }
    pub fn ToAsn1Object(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Object>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1Object,
        > = __cordl_object.invoke("ToAsn1Object", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IAsn1String2(
        &mut self,
        contents: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::IAsn1String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (contents))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString1(
        &mut self,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (text))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_Il2CppString0(
        &mut self,
        _cordl_type: i32,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, text))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+DisplayText")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::X509::DisplayText {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+DisplayText")]
impl AsRef<crate::Org::BouncyCastle::Asn1::IAsn1Choice>
for crate::Org::BouncyCastle::Asn1::X509::DisplayText {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Asn1::IAsn1Choice {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+DisplayText")]
impl AsMut<crate::Org::BouncyCastle::Asn1::IAsn1Choice>
for crate::Org::BouncyCastle::Asn1::X509::DisplayText {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Asn1::IAsn1Choice {
        unsafe { std::mem::transmute(self) }
    }
}
