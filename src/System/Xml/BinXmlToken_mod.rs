#[cfg(feature = "System+Xml+BinXmlToken")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinXmlToken {
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::BinXmlToken => "System.Xml"
    ."BinXmlToken"
);