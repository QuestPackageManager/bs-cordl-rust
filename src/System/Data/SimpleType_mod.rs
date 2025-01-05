#[cfg(feature = "System+Data+SimpleType")]
#[repr(C)]
#[derive(Debug)]
pub struct SimpleType {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _baseType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _baseSimpleType: quest_hook::libil2cpp::Gc<crate::System::Data::SimpleType>,
    pub _xmlBaseType: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub _name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _length: i32,
    pub _minLength: i32,
    pub _maxLength: i32,
    pub _pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _maxExclusive: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _maxInclusive: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _minExclusive: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _minInclusive: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _enumeration: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "System+Data+SimpleType")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::SimpleType => "System.Data"
    ."SimpleType"
);
#[cfg(feature = "System+Data+SimpleType")]
impl std::ops::Deref for crate::System::Data::SimpleType {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+SimpleType")]
impl std::ops::DerefMut for crate::System::Data::SimpleType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+SimpleType")]
impl crate::System::Data::SimpleType {
    pub fn CanHaveMaxLength(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanHaveMaxLength", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToAnnonymousSimpleType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConvertToAnnonymousSimpleType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateByteArrayType(
        encoding: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::SimpleType>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::SimpleType> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateByteArrayType", (encoding))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateEnumeratedType(
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::SimpleType>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::SimpleType> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateEnumeratedType", (values))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateLimitedStringType(
        length: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::SimpleType>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::SimpleType> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateLimitedStringType", (length))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateSimpleType(
        typeCode: crate::System::Data::Common::StorageType,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::SimpleType>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::SimpleType> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateSimpleType", (typeCode, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasConflictingDefinition(
        &mut self,
        otherSimpleType: quest_hook::libil2cpp::Gc<crate::System::Data::SimpleType>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("HasConflictingDefinition", (otherSimpleType))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPlainString(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsPlainString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadTypeValues(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaSimpleType>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadTypeValues", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Il2CppString0(
        baseType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (baseType))?;
        Ok(__cordl_object.into())
    }
    pub fn New_XmlSchemaSimpleType1(
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaSimpleType>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (node))?;
        Ok(__cordl_object.into())
    }
    pub fn QualifiedName(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("QualifiedName", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Runtime_Serialization_ISerializable_GetObjectData(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Runtime.Serialization.ISerializable.GetObjectData",
                (info, context),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToNode(
        &mut self,
        dc: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocument>,
        prefixes: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
        inRemoting: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode> = __cordl_object
            .invoke("ToNode", (dc, prefixes, inRemoting))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString0(
        &mut self,
        baseType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (baseType))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_XmlSchemaSimpleType1(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaSimpleType>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_BaseSimpleType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::SimpleType>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::SimpleType> = __cordl_object
            .invoke("get_BaseSimpleType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_BaseType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_BaseType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Length(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Length", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MaxLength(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_MaxLength", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Name", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Namespace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Namespace", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SimpleTypeQualifiedName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_SimpleTypeQualifiedName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_XmlBaseType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::XmlQualifiedName,
        > = __cordl_object.invoke("get_XmlBaseType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_MaxLength(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MaxLength", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Data+SimpleType")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::SimpleType {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Data+SimpleType")]
impl AsRef<crate::System::Runtime::Serialization::ISerializable>
for crate::System::Data::SimpleType {
    fn as_ref(&self) -> &crate::System::Runtime::Serialization::ISerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Data+SimpleType")]
impl AsMut<crate::System::Runtime::Serialization::ISerializable>
for crate::System::Data::SimpleType {
    fn as_mut(&mut self) -> &mut crate::System::Runtime::Serialization::ISerializable {
        unsafe { std::mem::transmute(self) }
    }
}
