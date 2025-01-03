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
        buf: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        oidSymbols: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
        oid: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
        val: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AppendValue", (buf, oidSymbols, oid, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn DecodeOid(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        lookUp: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::DerObjectIdentifier>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        > = __cordl_object.invoke("DecodeOid", (name, lookUp))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equivalent_X509Name1(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::X509Name>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equivalent", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equivalent__cordl_bool0(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::X509Name>,
        inOrder: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equivalent", (other, inOrder))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInstance_Asn1TaggedObject__cordl_bool0(
        obj: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1TaggedObject>,
        explicitly: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::X509Name>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509Name,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInstance", (obj, explicitly))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInstance_Il2CppObject1(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::X509Name>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509Name,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetInstance", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOidList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Collections::IList> = __cordl_object
            .invoke("GetOidList", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetValueList_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Collections::IList> = __cordl_object
            .invoke("GetValueList", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetValueList_DerObjectIdentifier1(
        &mut self,
        oid: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Collections::IList> = __cordl_object
            .invoke("GetValueList", (oid))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_Asn1Sequence1(
        seq: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Sequence>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object.into())
    }
    pub fn New_IList_IDictionary2(
        ordering: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        attributes: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ordering, attributes))?;
        Ok(__cordl_object.into())
    }
    pub fn New_IList_IDictionary_X509NameEntryConverter3(
        ordering: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        attributes: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
        converter: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509NameEntryConverter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ordering, attributes, converter))?;
        Ok(__cordl_object.into())
    }
    pub fn New_IList_IList4(
        oids: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        values: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (oids, values))?;
        Ok(__cordl_object.into())
    }
    pub fn New_IList_IList_X509NameEntryConverter5(
        oids: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        values: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        converter: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509NameEntryConverter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (oids, values, converter))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString6(
        dirName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dirName))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_X509NameEntryConverter7(
        dirName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        converter: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509NameEntryConverter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dirName, converter))?;
        Ok(__cordl_object.into())
    }
    pub fn New__cordl_bool_IDictionary_Il2CppString10(
        reverse: bool,
        lookUp: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
        dirName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (reverse, lookUp, dirName))?;
        Ok(__cordl_object.into())
    }
    pub fn New__cordl_bool_IDictionary_Il2CppString_X509NameEntryConverter11(
        reverse: bool,
        lookUp: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
        dirName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        converter: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509NameEntryConverter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (reverse, lookUp, dirName, converter))?;
        Ok(__cordl_object.into())
    }
    pub fn New__cordl_bool_Il2CppString8(
        reverse: bool,
        dirName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (reverse, dirName))?;
        Ok(__cordl_object.into())
    }
    pub fn New__cordl_bool_Il2CppString_X509NameEntryConverter9(
        reverse: bool,
        dirName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        converter: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509NameEntryConverter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (reverse, dirName, converter))?;
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
    pub fn ToString_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString__cordl_bool_IDictionary0(
        &mut self,
        reverse: bool,
        oidSymbols: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", (reverse, oidSymbols))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Asn1Sequence1(
        &mut self,
        seq: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Sequence>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (seq))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IList_IDictionary2(
        &mut self,
        ordering: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        attributes: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (ordering, attributes))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IList_IDictionary_X509NameEntryConverter3(
        &mut self,
        ordering: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        attributes: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
        converter: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509NameEntryConverter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (ordering, attributes, converter))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IList_IList4(
        &mut self,
        oids: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        values: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (oids, values))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IList_IList_X509NameEntryConverter5(
        &mut self,
        oids: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        values: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        converter: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509NameEntryConverter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (oids, values, converter))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString6(
        &mut self,
        dirName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (dirName))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_X509NameEntryConverter7(
        &mut self,
        dirName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        converter: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509NameEntryConverter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (dirName, converter))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool_IDictionary_Il2CppString10(
        &mut self,
        reverse: bool,
        lookUp: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
        dirName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (reverse, lookUp, dirName))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool_IDictionary_Il2CppString_X509NameEntryConverter11(
        &mut self,
        reverse: bool,
        lookUp: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
        dirName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        converter: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509NameEntryConverter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (reverse, lookUp, dirName, converter))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool_Il2CppString8(
        &mut self,
        reverse: bool,
        dirName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (reverse, dirName))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool_Il2CppString_X509NameEntryConverter9(
        &mut self,
        reverse: bool,
        dirName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        converter: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509NameEntryConverter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (reverse, dirName, converter))?;
        Ok(__cordl_ret.into())
    }
    pub fn canonicalize(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("canonicalize", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn decodeObject(
        v: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Object>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1Object,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("decodeObject", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn equivalentStrings(
        s1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        s2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("equivalentStrings", (s1, s2))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DefaultReverse() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_DefaultReverse", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_DefaultReverse(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_DefaultReverse", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn stripInternalSpaces(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("stripInternalSpaces", (str))?;
        Ok(__cordl_ret.into())
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
