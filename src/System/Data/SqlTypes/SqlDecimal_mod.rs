#[cfg(feature = "System+Data+SqlTypes+SqlDecimal")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SqlDecimal {
    pub _bStatus: u8,
    pub _bLen: u8,
    pub _bPrec: u8,
    pub _bScale: u8,
    pub _data1: u32,
    pub _data2: u32,
    pub _data3: u32,
    pub _data4: u32,
}
#[cfg(feature = "System+Data+SqlTypes+SqlDecimal")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Data::SqlTypes::SqlDecimal =>
    "System.Data.SqlTypes"."SqlDecimal"
);
#[cfg(feature = "System+Data+SqlTypes+SqlDecimal")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Data::SqlTypes::SqlDecimal {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlDecimal")]
impl crate::System::Data::SqlTypes::SqlDecimal {
    pub fn SetToZero(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetToZero",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Data(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<i32>> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<i32> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Data",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetPositive(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetPositive",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetSignBit(
        &mut self,
        fPositive: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetSignBit",
            (fPositive),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_Serialization_IXmlSerializable_ReadXml(
        &mut self,
        reader: *mut crate::System::Xml::XmlReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Xml.Serialization.IXmlSerializable.ReadXml",
            (reader),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToSqlDouble(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlDouble> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlDouble = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToSqlDouble",
            (),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn _ctor_Decimal1(
        &mut self,
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray_u8_u8_u8__cordl_bool4(
        &mut self,
        rglData: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
        bLen: u8,
        bPrec: u8,
        bScale: u8,
        fPositive: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (rglData, bLen, bPrec, bScale, fPositive),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Value",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToDouble(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToDouble",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_IsPositive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsPositive",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn CalculatePrecision(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CalculatePrecision",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Scale(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Scale",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToDecimal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToDecimal",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn FGt10_38_0(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "FGt10_38",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn FGt10_38_Il2CppArray1(
        &mut self,
        rglData: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "FGt10_38",
            (rglData),
        )?;
        Ok(__cordl_ret)
    }
    pub fn AddULong(
        &mut self,
        ulAdd: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddULong",
            (ulAdd),
        )?;
        Ok(__cordl_ret)
    }
    pub fn MultByULong(
        &mut self,
        uiMultiplier: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MultByULong",
            (uiMultiplier),
        )?;
        Ok(__cordl_ret)
    }
    pub fn AdjustScale(
        &mut self,
        digits: i32,
        fRound: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AdjustScale",
            (digits, fRound),
        )?;
        Ok(__cordl_ret)
    }
    pub fn VerifyPrecision(
        &mut self,
        precision: u8,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "VerifyPrecision",
            (precision),
        )?;
        Ok(__cordl_ret)
    }
    pub fn DivByULong(&mut self, iDivisor: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "DivByULong",
            (iDivisor),
        )?;
        Ok(__cordl_ret)
    }
    pub fn LAbsCmp(
        &mut self,
        snumOp: crate::System::Data::SqlTypes::SqlDecimal,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "LAbsCmp",
            (snumOp),
        )?;
        Ok(__cordl_ret)
    }
    pub fn StoreFromWorkingArray(
        &mut self,
        rguiData: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "StoreFromWorkingArray",
            (rguiData),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_IsNull(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsNull",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn CompareTo_Object0(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompareTo",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn CompareTo_SqlDecimal1(
        &mut self,
        value: crate::System::Data::SqlTypes::SqlDecimal,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompareTo",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_Serialization_IXmlSerializable_WriteXml(
        &mut self,
        writer: *mut crate::System::Xml::XmlWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Xml.Serialization.IXmlSerializable.WriteXml",
            (writer),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_Serialization_IXmlSerializable_GetSchema(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::XmlSchema> {
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchema = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Xml.Serialization.IXmlSerializable.GetSchema",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn FZero(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "FZero",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToSqlInt64(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlInt64> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlInt64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToSqlInt64",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn CompareNm(
        &mut self,
        snumOp: crate::System::Data::SqlTypes::SqlDecimal,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::EComparison> {
        let __cordl_ret: crate::System::Data::SqlTypes::EComparison = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompareNm",
            (snumOp),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToSqlMoney(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlMoney> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlMoney = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToSqlMoney",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
