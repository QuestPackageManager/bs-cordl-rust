#[cfg(feature = "System+Xml+BinXmlToken")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BinXmlToken {
    #[default]
    Attr = 246i32,
    CData = 242i32,
    Comment = 243i32,
    DocType = 252i32,
    _cordl_EOF = -1i32,
    Element = 248i32,
    Encoding = 253i32,
    EndAttrs = 245i32,
    EndCData = 241i32,
    EndElem = 247i32,
    EndNest = 235i32,
    Error = 0i32,
    Extn = 234i32,
    Name = 240i32,
    Nest = 236i32,
    NmFlush = 233i32,
    NotImpl = -2i32,
    PI = 244i32,
    Public = 250i32,
    QName = 239i32,
    SQL_BIGINT = 8i32,
    SQL_BINARY = 12i32,
    SQL_BIT = 6i32,
    SQL_CHAR = 13i32,
    SQL_DATETIME = 18i32,
    SQL_DECIMAL = 10i32,
    SQL_FLOAT = 4i32,
    SQL_IMAGE = 23i32,
    SQL_INT = 2i32,
    SQL_MONEY = 5i32,
    SQL_NCHAR = 14i32,
    SQL_NTEXT = 24i32,
    SQL_NUMERIC = 11i32,
    SQL_NVARCHAR = 17i32,
    SQL_REAL = 3i32,
    SQL_SMALLDATETIME = 19i32,
    SQL_SMALLINT = 1i32,
    SQL_SMALLMONEY = 20i32,
    SQL_TEXT = 22i32,
    SQL_TINYINT = 7i32,
    SQL_UDT = 27i32,
    SQL_UUID = 9i32,
    SQL_VARBINARY = 15i32,
    SQL_VARCHAR = 16i32,
    Subset = 249i32,
    System = 251i32,
    XSD_BASE64 = 133i32,
    XSD_BINHEX = 132i32,
    XSD_BOOLEAN = 134i32,
    XSD_BYTE = 136i32,
    XSD_DATE = 131i32,
    XSD_DATETIME = 130i32,
    XSD_DECIMAL = 135i32,
    XSD_KATMAI_DATE = 127i32,
    XSD_KATMAI_DATEOFFSET = 124i32,
    XSD_KATMAI_DATETIME = 126i32,
    XSD_KATMAI_DATETIMEOFFSET = 123i32,
    XSD_KATMAI_TIME = 125i32,
    XSD_KATMAI_TIMEOFFSET = 122i32,
    XSD_QNAME = 140i32,
    XSD_TIME = 129i32,
    XSD_UNSIGNEDINT = 138i32,
    XSD_UNSIGNEDLONG = 139i32,
    XSD_UNSIGNEDSHORT = 137i32,
    XmlDecl = 254i32,
    XmlText = 237i32,
}
#[cfg(feature = "System+Xml+BinXmlToken")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Xml::BinXmlToken {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Xml";
    const CLASS_NAME: &'static str = "BinXmlToken";
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
#[cfg(feature = "System+Xml+BinXmlToken")]
unsafe impl quest_hook::libil2cpp::Argument for crate::System::Xml::BinXmlToken {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Xml+BinXmlToken")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::System::Xml::BinXmlToken {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "System+Xml+BinXmlToken")]
unsafe impl quest_hook::libil2cpp::Returned for crate::System::Xml::BinXmlToken {
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
#[cfg(feature = "System+Xml+BinXmlToken")]
unsafe impl quest_hook::libil2cpp::Return for crate::System::Xml::BinXmlToken {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
