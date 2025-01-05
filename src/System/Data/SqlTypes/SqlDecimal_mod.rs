#[cfg(feature = "System+Data+SqlTypes+SqlDecimal")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
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
    pub fn AddULong(
        &mut self,
        ulAdd: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddULong",
            (ulAdd),
        )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn BGetPrecUI4(value: u32) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BGetPrecUI4", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BGetPrecUI8(dwlVal: u64) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BGetPrecUI8", (dwlVal))?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculatePrecision(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CalculatePrecision",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ChFromDigit(uiDigit: u32) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ChFromDigit", (uiDigit))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckValidPrecScale(
        bPrec: u8,
        bScale: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckValidPrecScale", (bPrec, bScale))?;
        Ok(__cordl_ret.into())
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
    pub fn CompareTo_SqlDecimal1(
        &mut self,
        value: crate::System::Data::SqlTypes::SqlDecimal,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompareTo",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn DWL(lo: u32, hi: u32) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DWL", (lo, hi))?;
        Ok(__cordl_ret.into())
    }
    pub fn DivByULong(&mut self, iDivisor: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "DivByULong",
            (iDivisor),
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
    pub fn FGt10_38_0(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "FGt10_38",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn FGt10_38_Il2CppArray1(
        &mut self,
        rglData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "FGt10_38",
            (rglData),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn FZero(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "FZero",
            (),
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
        x: crate::System::Data::SqlTypes::SqlDecimal,
        y: crate::System::Data::SqlTypes::SqlDecimal,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlBoolean> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlBoolean = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GreaterThan", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn HI(x: u64) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HI", (x))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn LO(x: u64) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LO", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn LessThan(
        x: crate::System::Data::SqlTypes::SqlDecimal,
        y: crate::System::Data::SqlTypes::SqlDecimal,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlBoolean> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlBoolean = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LessThan", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn MpDiv(
        rgulU: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        ciulU: i32,
        rgulD: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        ciulD: i32,
        rgulQ: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        ciulQ: quest_hook::libil2cpp::ByRefMut<i32>,
        rgulR: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        ciulR: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MpDiv", (rgulU, ciulU, rgulD, ciulD, rgulQ, ciulQ, rgulR, ciulR))?;
        Ok(__cordl_ret.into())
    }
    pub fn MpDiv1(
        rgulU: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        ciulU: quest_hook::libil2cpp::ByRefMut<i32>,
        iulD: u32,
        iulR: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MpDiv1", (rgulU, ciulU, iulD, iulR))?;
        Ok(__cordl_ret.into())
    }
    pub fn MpMove(
        rgulS: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        ciulS: i32,
        rgulD: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        ciulD: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MpMove", (rgulS, ciulS, rgulD, ciulD))?;
        Ok(__cordl_ret.into())
    }
    pub fn MpMul1(
        piulD: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        ciulD: quest_hook::libil2cpp::ByRefMut<i32>,
        iulX: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MpMul1", (piulD, ciulD, iulX))?;
        Ok(__cordl_ret.into())
    }
    pub fn MpNormalize(
        rgulU: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        ciulU: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MpNormalize", (rgulU, ciulU))?;
        Ok(__cordl_ret.into())
    }
    pub fn MpSet(
        rgulD: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        ciulD: quest_hook::libil2cpp::ByRefMut<i32>,
        iulN: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MpSet", (rgulD, ciulD, iulN))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn Parse(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlDecimal> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlDecimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Parse", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPositive(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetPositive",
            (),
        )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn SetToZero(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetToZero",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn StoreFromWorkingArray(
        &mut self,
        rguiData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "StoreFromWorkingArray",
            (rguiData),
        )?;
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
    pub fn ToSqlInt64(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlInt64> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlInt64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToSqlInt64",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSqlMoney(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlMoney> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlMoney = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToSqlMoney",
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
    pub fn VerifyPrecision(
        &mut self,
        precision: u8,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "VerifyPrecision",
            (precision),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ZeroToMaxLen(
        rgulData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        cUI4sCur: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ZeroToMaxLen", (rgulData, cUI4sCur))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray_u8_u8_u8__cordl_bool4(
        &mut self,
        rglData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
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
    pub fn get_Data(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Data", ())?;
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
    pub fn get_IsPositive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsPositive",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Scale(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Scale",
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
        x: crate::System::Data::SqlTypes::SqlDecimal,
        y: crate::System::Data::SqlTypes::SqlDecimal,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlDecimal> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlDecimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Division(
        x: crate::System::Data::SqlTypes::SqlDecimal,
        y: crate::System::Data::SqlTypes::SqlDecimal,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlDecimal> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlDecimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Division", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        x: crate::System::Data::SqlTypes::SqlDecimal,
        y: crate::System::Data::SqlTypes::SqlDecimal,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlBoolean> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlBoolean = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan(
        x: crate::System::Data::SqlTypes::SqlDecimal,
        y: crate::System::Data::SqlTypes::SqlDecimal,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlBoolean> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlBoolean = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThan", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_Decimal0(
        x: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlDecimal> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlDecimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_SqlByte2(
        x: crate::System::Data::SqlTypes::SqlByte,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlDecimal> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlDecimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_SqlInt16_3(
        x: crate::System::Data::SqlTypes::SqlInt16,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlDecimal> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlDecimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_SqlInt32_4(
        x: crate::System::Data::SqlTypes::SqlInt32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlDecimal> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlDecimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_SqlInt64_5(
        x: crate::System::Data::SqlTypes::SqlInt64,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlDecimal> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlDecimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_SqlMoney6(
        x: crate::System::Data::SqlTypes::SqlMoney,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlDecimal> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlDecimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_i64_1(
        x: i64,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlDecimal> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlDecimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan(
        x: crate::System::Data::SqlTypes::SqlDecimal,
        y: crate::System::Data::SqlTypes::SqlDecimal,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlBoolean> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlBoolean = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThan", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply(
        x: crate::System::Data::SqlTypes::SqlDecimal,
        y: crate::System::Data::SqlTypes::SqlDecimal,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlDecimal> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlDecimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction(
        x: crate::System::Data::SqlTypes::SqlDecimal,
        y: crate::System::Data::SqlTypes::SqlDecimal,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlDecimal> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlDecimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_UnaryNegation(
        x: crate::System::Data::SqlTypes::SqlDecimal,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlDecimal> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlDecimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_UnaryNegation", (x))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlDecimal")]
impl AsRef<crate::System::Data::SqlTypes::INullable>
for crate::System::Data::SqlTypes::SqlDecimal {
    fn as_ref(&self) -> &crate::System::Data::SqlTypes::INullable {
        todo!()
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlDecimal")]
impl AsMut<crate::System::Data::SqlTypes::INullable>
for crate::System::Data::SqlTypes::SqlDecimal {
    fn as_mut(&mut self) -> &mut crate::System::Data::SqlTypes::INullable {
        todo!()
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlDecimal")]
impl AsRef<crate::System::IComparable> for crate::System::Data::SqlTypes::SqlDecimal {
    fn as_ref(&self) -> &crate::System::IComparable {
        todo!()
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlDecimal")]
impl AsMut<crate::System::IComparable> for crate::System::Data::SqlTypes::SqlDecimal {
    fn as_mut(&mut self) -> &mut crate::System::IComparable {
        todo!()
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlDecimal")]
impl AsRef<crate::System::Xml::Serialization::IXmlSerializable>
for crate::System::Data::SqlTypes::SqlDecimal {
    fn as_ref(&self) -> &crate::System::Xml::Serialization::IXmlSerializable {
        todo!()
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlDecimal")]
impl AsMut<crate::System::Xml::Serialization::IXmlSerializable>
for crate::System::Data::SqlTypes::SqlDecimal {
    fn as_mut(&mut self) -> &mut crate::System::Xml::Serialization::IXmlSerializable {
        todo!()
    }
}
