#[cfg(feature = "System+Data+SqlTypes+SqlDouble")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SqlDouble {
    pub m_fNotNull: bool,
    pub m_value: f64,
}
#[cfg(feature = "System+Data+SqlTypes+SqlDouble")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Data::SqlTypes::SqlDouble =>
    "System.Data.SqlTypes"."SqlDouble"
);
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
    pub fn CompareTo_SqlDouble1(
        &mut self,
        value: crate::System::Data::SqlTypes::SqlDouble,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompareTo",
            (value),
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
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
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
    pub fn ToSqlSingle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlSingle> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlSingle = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToSqlSingle",
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
    pub fn _ctor_f64_1(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value),
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
    pub fn get_Value(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Value",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
