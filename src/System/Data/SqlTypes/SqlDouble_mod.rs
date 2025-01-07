#[cfg(feature = "System+Data+SqlTypes+SqlDouble")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SqlDouble {
    pub m_fNotNull: bool,
    pub m_value: f64,
}
#[cfg(feature = "System+Data+SqlTypes+SqlDouble")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Data::SqlTypes::SqlDouble {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Data.SqlTypes";
    const CLASS_NAME: &'static str = "SqlDouble";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Data::SqlTypes::SqlDouble {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Data::SqlTypes::SqlDouble {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Data::SqlTypes::SqlDouble {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
unsafe impl quest_hook::libil2cpp::Return for crate::System::Data::SqlTypes::SqlDouble {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlDouble")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Data::SqlTypes::SqlDouble {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlDouble")]
impl crate::System::Data::SqlTypes::SqlDouble {
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
    pub fn CompareTo_SqlDouble1(
        &mut self,
        value: crate::System::Data::SqlTypes::SqlDouble,
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
        x: crate::System::Data::SqlTypes::SqlDouble,
        y: crate::System::Data::SqlTypes::SqlDouble,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlBoolean> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlBoolean = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GreaterThan", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn LessThan(
        x: crate::System::Data::SqlTypes::SqlDouble,
        y: crate::System::Data::SqlTypes::SqlDouble,
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
    pub fn ToSqlSingle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlSingle> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlSingle = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToSqlSingle",
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
    pub fn _ctor_f64_1(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value),
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
    pub fn get_Value(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Value",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition(
        x: crate::System::Data::SqlTypes::SqlDouble,
        y: crate::System::Data::SqlTypes::SqlDouble,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlDouble> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlDouble = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Division(
        x: crate::System::Data::SqlTypes::SqlDouble,
        y: crate::System::Data::SqlTypes::SqlDouble,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlDouble> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlDouble = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Division", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        x: crate::System::Data::SqlTypes::SqlDouble,
        y: crate::System::Data::SqlTypes::SqlDouble,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlBoolean> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlBoolean = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan(
        x: crate::System::Data::SqlTypes::SqlDouble,
        y: crate::System::Data::SqlTypes::SqlDouble,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlBoolean> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlBoolean = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThan", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_SqlByte1(
        x: crate::System::Data::SqlTypes::SqlByte,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlDouble> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlDouble = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_SqlDecimal7(
        x: crate::System::Data::SqlTypes::SqlDecimal,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlDouble> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlDouble = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_SqlInt16_2(
        x: crate::System::Data::SqlTypes::SqlInt16,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlDouble> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlDouble = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_SqlInt32_3(
        x: crate::System::Data::SqlTypes::SqlInt32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlDouble> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlDouble = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_SqlInt64_4(
        x: crate::System::Data::SqlTypes::SqlInt64,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlDouble> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlDouble = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_SqlMoney6(
        x: crate::System::Data::SqlTypes::SqlMoney,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlDouble> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlDouble = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_SqlSingle5(
        x: crate::System::Data::SqlTypes::SqlSingle,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlDouble> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlDouble = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_f64_0(
        x: f64,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlDouble> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlDouble = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan(
        x: crate::System::Data::SqlTypes::SqlDouble,
        y: crate::System::Data::SqlTypes::SqlDouble,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlBoolean> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlBoolean = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThan", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply(
        x: crate::System::Data::SqlTypes::SqlDouble,
        y: crate::System::Data::SqlTypes::SqlDouble,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlDouble> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlDouble = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction(
        x: crate::System::Data::SqlTypes::SqlDouble,
        y: crate::System::Data::SqlTypes::SqlDouble,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlDouble> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlDouble = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_UnaryNegation(
        x: crate::System::Data::SqlTypes::SqlDouble,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlDouble> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlDouble = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_UnaryNegation", (x))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlDouble")]
impl AsRef<crate::System::Data::SqlTypes::INullable>
for crate::System::Data::SqlTypes::SqlDouble {
    fn as_ref(&self) -> &crate::System::Data::SqlTypes::INullable {
        todo!()
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlDouble")]
impl AsMut<crate::System::Data::SqlTypes::INullable>
for crate::System::Data::SqlTypes::SqlDouble {
    fn as_mut(&mut self) -> &mut crate::System::Data::SqlTypes::INullable {
        todo!()
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlDouble")]
impl AsRef<crate::System::IComparable> for crate::System::Data::SqlTypes::SqlDouble {
    fn as_ref(&self) -> &crate::System::IComparable {
        todo!()
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlDouble")]
impl AsMut<crate::System::IComparable> for crate::System::Data::SqlTypes::SqlDouble {
    fn as_mut(&mut self) -> &mut crate::System::IComparable {
        todo!()
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlDouble")]
impl AsRef<crate::System::Xml::Serialization::IXmlSerializable>
for crate::System::Data::SqlTypes::SqlDouble {
    fn as_ref(&self) -> &crate::System::Xml::Serialization::IXmlSerializable {
        todo!()
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlDouble")]
impl AsMut<crate::System::Xml::Serialization::IXmlSerializable>
for crate::System::Data::SqlTypes::SqlDouble {
    fn as_mut(&mut self) -> &mut crate::System::Xml::Serialization::IXmlSerializable {
        todo!()
    }
}
