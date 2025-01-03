#[cfg(feature = "System+Data+SqlTypes+SqlMoney")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SqlMoney {
    pub _fNotNull: bool,
    pub _value: i64,
}
#[cfg(feature = "System+Data+SqlTypes+SqlMoney")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Data::SqlTypes::SqlMoney =>
    "System.Data.SqlTypes"."SqlMoney"
);
#[cfg(feature = "System+Data+SqlTypes+SqlMoney")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Data::SqlTypes::SqlMoney {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlMoney")]
impl crate::System::Data::SqlTypes::SqlMoney {
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
    pub fn CompareTo_SqlMoney1(
        &mut self,
        value: crate::System::Data::SqlTypes::SqlMoney,
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
    pub fn GreaterThan(
        x: crate::System::Data::SqlTypes::SqlMoney,
        y: crate::System::Data::SqlTypes::SqlMoney,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlBoolean> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlBoolean = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GreaterThan", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn LessThan(
        x: crate::System::Data::SqlTypes::SqlMoney,
        y: crate::System::Data::SqlTypes::SqlMoney,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlBoolean> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlBoolean = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LessThan", (x, y))?;
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
    pub fn ToDecimal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToDecimal",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDouble(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToDouble",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSqlDecimal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlDecimal> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlDecimal = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToSqlDecimal",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSqlDouble(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlDouble> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlDouble = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToSqlDouble",
            (),
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
    pub fn _ctor_Decimal4(
        &mut self,
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool0(
        &mut self,
        fNull: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (fNull),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_2(
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
    pub fn _ctor_i64_3(
        &mut self,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i64_i32_1(
        &mut self,
        value: i64,
        ignored: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value, ignored),
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
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Value",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition(
        x: crate::System::Data::SqlTypes::SqlMoney,
        y: crate::System::Data::SqlTypes::SqlMoney,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlMoney> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlMoney = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Division(
        x: crate::System::Data::SqlTypes::SqlMoney,
        y: crate::System::Data::SqlTypes::SqlMoney,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlMoney> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlMoney = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Division", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        x: crate::System::Data::SqlTypes::SqlMoney,
        y: crate::System::Data::SqlTypes::SqlMoney,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlBoolean> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlBoolean = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit(
        x: crate::System::Data::SqlTypes::SqlDecimal,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlMoney> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlMoney = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan(
        x: crate::System::Data::SqlTypes::SqlMoney,
        y: crate::System::Data::SqlTypes::SqlMoney,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlBoolean> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlBoolean = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThan", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_Decimal0(
        x: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlMoney> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlMoney = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_SqlByte2(
        x: crate::System::Data::SqlTypes::SqlByte,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlMoney> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlMoney = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_SqlInt16_3(
        x: crate::System::Data::SqlTypes::SqlInt16,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlMoney> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlMoney = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_SqlInt32_4(
        x: crate::System::Data::SqlTypes::SqlInt32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlMoney> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlMoney = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_SqlInt64_5(
        x: crate::System::Data::SqlTypes::SqlInt64,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlMoney> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlMoney = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_i64_1(
        x: i64,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlMoney> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlMoney = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan(
        x: crate::System::Data::SqlTypes::SqlMoney,
        y: crate::System::Data::SqlTypes::SqlMoney,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlBoolean> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlBoolean = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThan", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply(
        x: crate::System::Data::SqlTypes::SqlMoney,
        y: crate::System::Data::SqlTypes::SqlMoney,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlMoney> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlMoney = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction(
        x: crate::System::Data::SqlTypes::SqlMoney,
        y: crate::System::Data::SqlTypes::SqlMoney,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlMoney> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlMoney = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_UnaryNegation(
        x: crate::System::Data::SqlTypes::SqlMoney,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlMoney> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlMoney = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_UnaryNegation", (x))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlMoney")]
impl AsRef<crate::System::Data::SqlTypes::INullable>
for crate::System::Data::SqlTypes::SqlMoney {
    fn as_ref(&self) -> &crate::System::Data::SqlTypes::INullable {
        todo!()
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlMoney")]
impl AsMut<crate::System::Data::SqlTypes::INullable>
for crate::System::Data::SqlTypes::SqlMoney {
    fn as_mut(&mut self) -> &mut crate::System::Data::SqlTypes::INullable {
        todo!()
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlMoney")]
impl AsRef<crate::System::IComparable> for crate::System::Data::SqlTypes::SqlMoney {
    fn as_ref(&self) -> &crate::System::IComparable {
        todo!()
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlMoney")]
impl AsMut<crate::System::IComparable> for crate::System::Data::SqlTypes::SqlMoney {
    fn as_mut(&mut self) -> &mut crate::System::IComparable {
        todo!()
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlMoney")]
impl AsRef<crate::System::Xml::Serialization::IXmlSerializable>
for crate::System::Data::SqlTypes::SqlMoney {
    fn as_ref(&self) -> &crate::System::Xml::Serialization::IXmlSerializable {
        todo!()
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlMoney")]
impl AsMut<crate::System::Xml::Serialization::IXmlSerializable>
for crate::System::Data::SqlTypes::SqlMoney {
    fn as_mut(&mut self) -> &mut crate::System::Xml::Serialization::IXmlSerializable {
        todo!()
    }
}
