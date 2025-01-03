#[cfg(feature = "System+Data+SqlTypes+SqlBoolean")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct SqlBoolean {
    pub m_value: u8,
}
#[cfg(feature = "System+Data+SqlTypes+SqlBoolean")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Data::SqlTypes::SqlBoolean =>
    "System.Data.SqlTypes"."SqlBoolean"
);
#[cfg(feature = "System+Data+SqlTypes+SqlBoolean")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Data::SqlTypes::SqlBoolean {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlBoolean")]
impl crate::System::Data::SqlTypes::SqlBoolean {
    pub fn And(
        x: crate::System::Data::SqlTypes::SqlBoolean,
        y: crate::System::Data::SqlTypes::SqlBoolean,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlBoolean> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlBoolean = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("And", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareTo_Il2CppObject0(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompareTo",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareTo_SqlBoolean1(
        &mut self,
        value: crate::System::Data::SqlTypes::SqlBoolean,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompareTo",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetXsdType(
        schemaSet: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaSet>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::XmlQualifiedName,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetXsdType", (schemaSet))?;
        Ok(__cordl_ret.into())
    }
    pub fn Or(
        x: crate::System::Data::SqlTypes::SqlBoolean,
        y: crate::System::Data::SqlTypes::SqlBoolean,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlBoolean> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlBoolean = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Or", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Xml_Serialization_IXmlSerializable_GetSchema(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchema,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Xml.Serialization.IXmlSerializable.GetSchema",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Xml_Serialization_IXmlSerializable_ReadXml(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Xml.Serialization.IXmlSerializable.ReadXml",
            (reader),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Xml_Serialization_IXmlSerializable_WriteXml(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Xml.Serialization.IXmlSerializable.WriteXml",
            (writer),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool0(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_1(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32__cordl_bool2(
        &mut self,
        value: i32,
        fNull: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value, fNull),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ByteValue(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ByteValue",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsFalse(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsFalse",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsNull(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsNull",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsTrue(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsTrue",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Value(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Value",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseAnd(
        x: crate::System::Data::SqlTypes::SqlBoolean,
        y: crate::System::Data::SqlTypes::SqlBoolean,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlBoolean> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlBoolean = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_BitwiseAnd", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseOr(
        x: crate::System::Data::SqlTypes::SqlBoolean,
        y: crate::System::Data::SqlTypes::SqlBoolean,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlBoolean> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlBoolean = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_BitwiseOr", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        x: crate::System::Data::SqlTypes::SqlBoolean,
        y: crate::System::Data::SqlTypes::SqlBoolean,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlBoolean> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlBoolean = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        x: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlBoolean> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlBoolean = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_True(
        x: crate::System::Data::SqlTypes::SqlBoolean,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_True", (x))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlBoolean")]
impl AsRef<crate::System::Data::SqlTypes::INullable>
for crate::System::Data::SqlTypes::SqlBoolean {
    fn as_ref(&self) -> &crate::System::Data::SqlTypes::INullable {
        todo!()
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlBoolean")]
impl AsMut<crate::System::Data::SqlTypes::INullable>
for crate::System::Data::SqlTypes::SqlBoolean {
    fn as_mut(&mut self) -> &mut crate::System::Data::SqlTypes::INullable {
        todo!()
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlBoolean")]
impl AsRef<crate::System::IComparable> for crate::System::Data::SqlTypes::SqlBoolean {
    fn as_ref(&self) -> &crate::System::IComparable {
        todo!()
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlBoolean")]
impl AsMut<crate::System::IComparable> for crate::System::Data::SqlTypes::SqlBoolean {
    fn as_mut(&mut self) -> &mut crate::System::IComparable {
        todo!()
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlBoolean")]
impl AsRef<crate::System::Xml::Serialization::IXmlSerializable>
for crate::System::Data::SqlTypes::SqlBoolean {
    fn as_ref(&self) -> &crate::System::Xml::Serialization::IXmlSerializable {
        todo!()
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlBoolean")]
impl AsMut<crate::System::Xml::Serialization::IXmlSerializable>
for crate::System::Data::SqlTypes::SqlBoolean {
    fn as_mut(&mut self) -> &mut crate::System::Xml::Serialization::IXmlSerializable {
        todo!()
    }
}
