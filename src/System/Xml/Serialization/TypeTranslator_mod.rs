#[cfg(feature = "System+Xml+Serialization+TypeTranslator")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeTranslator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Xml+Serialization+TypeTranslator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Serialization::TypeTranslator =>
    "System.Xml.Serialization"."TypeTranslator"
);
#[cfg(feature = "System+Xml+Serialization+TypeTranslator")]
impl std::ops::Deref for crate::System::Xml::Serialization::TypeTranslator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+TypeTranslator")]
impl std::ops::DerefMut for crate::System::Xml::Serialization::TypeTranslator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+TypeTranslator")]
impl crate::System::Xml::Serialization::TypeTranslator {
    pub fn FindPrimitiveTypeData(
        typeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::TypeData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::TypeData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindPrimitiveTypeData", (typeName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetArrayName(
        elemName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetArrayName", (elemName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPrimitiveTypeData_Il2CppString0(
        typeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::TypeData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::TypeData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPrimitiveTypeData", (typeName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPrimitiveTypeData__cordl_bool1(
        typeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        nullable: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::TypeData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::TypeData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPrimitiveTypeData", (typeName, nullable))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTypeData_Il2CppString__cordl_bool1(
        runtimeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        xmlDataType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        underlyingEnumType: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::TypeData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::TypeData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTypeData", (runtimeType, xmlDataType, underlyingEnumType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTypeData_Type0(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::TypeData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::TypeData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTypeData", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseArrayType(
        arrayType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        ns: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        dimensions: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseArrayType", (arrayType, _cordl_type, ns, dimensions))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Serialization+TypeTranslator")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::TypeTranslator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
