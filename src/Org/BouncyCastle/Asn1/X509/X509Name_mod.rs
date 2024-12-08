#[cfg(feature = "Org+BouncyCastle+Asn1+X509+X509Name")]
#[repr(C)]
#[derive(Debug)]
pub struct X509Name {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub ordering: *mut crate::System::Collections::IList,
    pub converter: *mut crate::Org::BouncyCastle::Asn1::X509::X509NameEntryConverter,
    pub values: *mut crate::System::Collections::IList,
    pub added: *mut crate::System::Collections::IList,
    pub seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+X509Name")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::X509::X509Name =>
    "Org.BouncyCastle.Asn1.X509"."X509Name"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+X509Name")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::X509::X509Name {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+X509Name")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::X509::X509Name {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+X509Name")]
impl crate::Org::BouncyCastle::Asn1::X509::X509Name {
    pub fn AppendValue(
        &mut self,
        buf: *mut crate::System::Text::StringBuilder,
        oidSymbols: *mut crate::System::Collections::IDictionary,
        oid: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        val: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AppendValue", (buf, oidSymbols, oid, val))?;
        Ok(__cordl_ret)
    }
    pub fn DecodeOid(
        &mut self,
        name: *mut crate::System::String,
        lookUp: *mut crate::System::Collections::IDictionary,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier = __cordl_object
            .invoke("DecodeOid", (name, lookUp))?;
        Ok(__cordl_ret)
    }
    pub fn Equivalent_X509Name1(
        &mut self,
        other: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equivalent", (other))?;
        Ok(__cordl_ret)
    }
    pub fn Equivalent__cordl_bool0(
        &mut self,
        other: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
        inOrder: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equivalent", (other, inOrder))?;
        Ok(__cordl_ret)
    }
    pub fn GetOidList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IList = __cordl_object
            .invoke("GetOidList", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetValueList_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IList = __cordl_object
            .invoke("GetValueList", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetValueList_DerObjectIdentifier1(
        &mut self,
        oid: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IList = __cordl_object
            .invoke("GetValueList", (oid))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_Asn1Sequence1(
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object)
    }
    pub fn New_IList_IDictionary2(
        ordering: *mut crate::System::Collections::IList,
        attributes: *mut crate::System::Collections::IDictionary,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ordering, attributes))?;
        Ok(__cordl_object)
    }
    pub fn New_IList_IDictionary_X509NameEntryConverter3(
        ordering: *mut crate::System::Collections::IList,
        attributes: *mut crate::System::Collections::IDictionary,
        converter: *mut crate::Org::BouncyCastle::Asn1::X509::X509NameEntryConverter,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ordering, attributes, converter))?;
        Ok(__cordl_object)
    }
    pub fn New_IList_IList4(
        oids: *mut crate::System::Collections::IList,
        values: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (oids, values))?;
        Ok(__cordl_object)
    }
    pub fn New_IList_IList_X509NameEntryConverter5(
        oids: *mut crate::System::Collections::IList,
        values: *mut crate::System::Collections::IList,
        converter: *mut crate::Org::BouncyCastle::Asn1::X509::X509NameEntryConverter,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (oids, values, converter))?;
        Ok(__cordl_object)
    }
    pub fn New_String6(
        dirName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dirName))?;
        Ok(__cordl_object)
    }
    pub fn New_String_X509NameEntryConverter7(
        dirName: *mut crate::System::String,
        converter: *mut crate::Org::BouncyCastle::Asn1::X509::X509NameEntryConverter,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dirName, converter))?;
        Ok(__cordl_object)
    }
    pub fn New__cordl_bool_IDictionary_String10(
        reverse: bool,
        lookUp: *mut crate::System::Collections::IDictionary,
        dirName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (reverse, lookUp, dirName))?;
        Ok(__cordl_object)
    }
    pub fn New__cordl_bool_IDictionary_String_X509NameEntryConverter11(
        reverse: bool,
        lookUp: *mut crate::System::Collections::IDictionary,
        dirName: *mut crate::System::String,
        converter: *mut crate::Org::BouncyCastle::Asn1::X509::X509NameEntryConverter,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (reverse, lookUp, dirName, converter))?;
        Ok(__cordl_object)
    }
    pub fn New__cordl_bool_String8(
        reverse: bool,
        dirName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (reverse, dirName))?;
        Ok(__cordl_object)
    }
    pub fn New__cordl_bool_String_X509NameEntryConverter9(
        reverse: bool,
        dirName: *mut crate::System::String,
        converter: *mut crate::Org::BouncyCastle::Asn1::X509::X509NameEntryConverter,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (reverse, dirName, converter))?;
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
    pub fn ToString_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn ToString__cordl_bool_IDictionary0(
        &mut self,
        reverse: bool,
        oidSymbols: *mut crate::System::Collections::IDictionary,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", (reverse, oidSymbols))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Asn1Sequence1(
        &mut self,
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (seq))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IList_IDictionary2(
        &mut self,
        ordering: *mut crate::System::Collections::IList,
        attributes: *mut crate::System::Collections::IDictionary,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (ordering, attributes))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IList_IDictionary_X509NameEntryConverter3(
        &mut self,
        ordering: *mut crate::System::Collections::IList,
        attributes: *mut crate::System::Collections::IDictionary,
        converter: *mut crate::Org::BouncyCastle::Asn1::X509::X509NameEntryConverter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (ordering, attributes, converter))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IList_IList4(
        &mut self,
        oids: *mut crate::System::Collections::IList,
        values: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (oids, values))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IList_IList_X509NameEntryConverter5(
        &mut self,
        oids: *mut crate::System::Collections::IList,
        values: *mut crate::System::Collections::IList,
        converter: *mut crate::Org::BouncyCastle::Asn1::X509::X509NameEntryConverter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (oids, values, converter))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String6(
        &mut self,
        dirName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (dirName))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_X509NameEntryConverter7(
        &mut self,
        dirName: *mut crate::System::String,
        converter: *mut crate::Org::BouncyCastle::Asn1::X509::X509NameEntryConverter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (dirName, converter))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor__cordl_bool_IDictionary_String10(
        &mut self,
        reverse: bool,
        lookUp: *mut crate::System::Collections::IDictionary,
        dirName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (reverse, lookUp, dirName))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor__cordl_bool_IDictionary_String_X509NameEntryConverter11(
        &mut self,
        reverse: bool,
        lookUp: *mut crate::System::Collections::IDictionary,
        dirName: *mut crate::System::String,
        converter: *mut crate::Org::BouncyCastle::Asn1::X509::X509NameEntryConverter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (reverse, lookUp, dirName, converter))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor__cordl_bool_String8(
        &mut self,
        reverse: bool,
        dirName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (reverse, dirName))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor__cordl_bool_String_X509NameEntryConverter9(
        &mut self,
        reverse: bool,
        dirName: *mut crate::System::String,
        converter: *mut crate::Org::BouncyCastle::Asn1::X509::X509NameEntryConverter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (reverse, dirName, converter))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+X509Name")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::X509::X509Name {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
