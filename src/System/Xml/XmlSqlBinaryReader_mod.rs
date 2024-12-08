#[cfg(feature = "System+Xml+XmlSqlBinaryReader+AttrInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct XmlSqlBinaryReader_AttrInfo {
    pub name: crate::System::Xml::XmlSqlBinaryReader_QName,
    pub val: *mut crate::System::String,
    pub contentPos: i32,
    pub hashCode: i32,
    pub prevHash: i32,
}
#[cfg(feature = "System+Xml+XmlSqlBinaryReader+AttrInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlSqlBinaryReader_AttrInfo =>
    "System.Xml"."XmlSqlBinaryReader/AttrInfo"
);
#[cfg(feature = "System+Xml+XmlSqlBinaryReader+AttrInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Xml::XmlSqlBinaryReader_AttrInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Xml+XmlSqlBinaryReader+AttrInfo")]
impl crate::System::Xml::XmlSqlBinaryReader_AttrInfo {
    pub fn AdjustPosition(
        &mut self,
        adj: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AdjustPosition",
            (adj),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetLocalnameAndNamespaceUri(
        &mut self,
        localname: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
        namespaceUri: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetLocalnameAndNamespaceUri",
            (localname, namespaceUri),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetLocalnameAndNamespaceUriAndHash(
        &mut self,
        hasher: *mut crate::System::Xml::SecureStringHasher,
        localname: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
        namespaceUri: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetLocalnameAndNamespaceUriAndHash",
            (hasher, localname, namespaceUri),
        )?;
        Ok(__cordl_ret)
    }
    pub fn MatchHashNS(
        &mut self,
        hash: i32,
        localname: *mut crate::System::String,
        namespaceUri: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MatchHashNS",
            (hash, localname, namespaceUri),
        )?;
        Ok(__cordl_ret)
    }
    pub fn MatchNS(
        &mut self,
        localname: *mut crate::System::String,
        namespaceUri: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MatchNS",
            (localname, namespaceUri),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Set_String0(
        &mut self,
        n: crate::System::Xml::XmlSqlBinaryReader_QName,
        v: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Set",
            (n, v),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Set_i32_1(
        &mut self,
        n: crate::System::Xml::XmlSqlBinaryReader_QName,
        pos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Set",
            (n, pos),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+XmlSqlBinaryReader+ElemInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct XmlSqlBinaryReader_ElemInfo {
    pub name: crate::System::Xml::XmlSqlBinaryReader_QName,
    pub xmlLang: *mut crate::System::String,
    pub xmlSpace: crate::System::Xml::XmlSpace,
    pub xmlspacePreserve: bool,
    pub nsdecls: *mut crate::System::Xml::XmlSqlBinaryReader_NamespaceDecl,
}
#[cfg(feature = "System+Xml+XmlSqlBinaryReader+ElemInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlSqlBinaryReader_ElemInfo =>
    "System.Xml"."XmlSqlBinaryReader/ElemInfo"
);
#[cfg(feature = "System+Xml+XmlSqlBinaryReader+ElemInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Xml::XmlSqlBinaryReader_ElemInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Xml+XmlSqlBinaryReader+ElemInfo")]
impl crate::System::Xml::XmlSqlBinaryReader_ElemInfo {
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::XmlSqlBinaryReader_NamespaceDecl,
    > {
        let __cordl_ret: *mut crate::System::Xml::XmlSqlBinaryReader_NamespaceDecl = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Clear",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Set(
        &mut self,
        name: crate::System::Xml::XmlSqlBinaryReader_QName,
        xmlspacePreserve: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Set",
            (name, xmlspacePreserve),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+XmlSqlBinaryReader+NamespaceDecl")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSqlBinaryReader_NamespaceDecl {
    __cordl_parent: crate::System::Object,
    pub prefix: *mut crate::System::String,
    pub uri: *mut crate::System::String,
    pub scopeLink: *mut crate::System::Xml::XmlSqlBinaryReader_NamespaceDecl,
    pub prevLink: *mut crate::System::Xml::XmlSqlBinaryReader_NamespaceDecl,
    pub scope: i32,
    pub implied: bool,
}
#[cfg(feature = "System+Xml+XmlSqlBinaryReader+NamespaceDecl")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlSqlBinaryReader_NamespaceDecl =>
    "System.Xml"."XmlSqlBinaryReader/NamespaceDecl"
);
#[cfg(feature = "System+Xml+XmlSqlBinaryReader+NamespaceDecl")]
impl std::ops::Deref for crate::System::Xml::XmlSqlBinaryReader_NamespaceDecl {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlSqlBinaryReader+NamespaceDecl")]
impl std::ops::DerefMut for crate::System::Xml::XmlSqlBinaryReader_NamespaceDecl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlSqlBinaryReader+NamespaceDecl")]
impl crate::System::Xml::XmlSqlBinaryReader_NamespaceDecl {
    pub fn New(
        prefix: *mut crate::System::String,
        nsuri: *mut crate::System::String,
        nextInScope: *mut crate::System::Xml::XmlSqlBinaryReader_NamespaceDecl,
        prevDecl: *mut crate::System::Xml::XmlSqlBinaryReader_NamespaceDecl,
        scope: i32,
        implied: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (prefix, nsuri, nextInScope, prevDecl, scope, implied),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        prefix: *mut crate::System::String,
        nsuri: *mut crate::System::String,
        nextInScope: *mut crate::System::Xml::XmlSqlBinaryReader_NamespaceDecl,
        prevDecl: *mut crate::System::Xml::XmlSqlBinaryReader_NamespaceDecl,
        scope: i32,
        implied: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (prefix, nsuri, nextInScope, prevDecl, scope, implied))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+XmlSqlBinaryReader+NamespaceDecl")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::XmlSqlBinaryReader_NamespaceDecl {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+XmlSqlBinaryReader+NestedBinXml")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSqlBinaryReader_NestedBinXml {
    __cordl_parent: crate::System::Object,
    pub symbolTables: crate::System::Xml::XmlSqlBinaryReader_SymbolTables,
    pub docState: i32,
    pub next: *mut crate::System::Xml::XmlSqlBinaryReader_NestedBinXml,
}
#[cfg(feature = "System+Xml+XmlSqlBinaryReader+NestedBinXml")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlSqlBinaryReader_NestedBinXml =>
    "System.Xml"."XmlSqlBinaryReader/NestedBinXml"
);
#[cfg(feature = "System+Xml+XmlSqlBinaryReader+NestedBinXml")]
impl std::ops::Deref for crate::System::Xml::XmlSqlBinaryReader_NestedBinXml {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlSqlBinaryReader+NestedBinXml")]
impl std::ops::DerefMut for crate::System::Xml::XmlSqlBinaryReader_NestedBinXml {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlSqlBinaryReader+NestedBinXml")]
impl crate::System::Xml::XmlSqlBinaryReader_NestedBinXml {
    pub fn New(
        symbolTables: crate::System::Xml::XmlSqlBinaryReader_SymbolTables,
        docState: i32,
        next: *mut crate::System::Xml::XmlSqlBinaryReader_NestedBinXml,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (symbolTables, docState, next))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        symbolTables: crate::System::Xml::XmlSqlBinaryReader_SymbolTables,
        docState: i32,
        next: *mut crate::System::Xml::XmlSqlBinaryReader_NestedBinXml,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (symbolTables, docState, next))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+XmlSqlBinaryReader+NestedBinXml")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::XmlSqlBinaryReader_NestedBinXml {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+XmlSqlBinaryReader+QName")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct XmlSqlBinaryReader_QName {
    pub prefix: *mut crate::System::String,
    pub localname: *mut crate::System::String,
    pub namespaceUri: *mut crate::System::String,
}
#[cfg(feature = "System+Xml+XmlSqlBinaryReader+QName")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlSqlBinaryReader_QName =>
    "System.Xml"."XmlSqlBinaryReader/QName"
);
#[cfg(feature = "System+Xml+XmlSqlBinaryReader+QName")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Xml::XmlSqlBinaryReader_QName {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Xml+XmlSqlBinaryReader+QName")]
impl crate::System::Xml::XmlSqlBinaryReader_QName {
    pub fn CheckPrefixNS(
        &mut self,
        prefix: *mut crate::System::String,
        namespaceUri: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CheckPrefixNS",
            (prefix, namespaceUri),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Clear",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals(
        &mut self,
        other: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
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
    pub fn GetNSHashCode(
        &mut self,
        hasher: *mut crate::System::Xml::SecureStringHasher,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetNSHashCode",
            (hasher),
        )?;
        Ok(__cordl_ret)
    }
    pub fn MatchNs(
        &mut self,
        lname: *mut crate::System::String,
        nsUri: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MatchNs",
            (lname, nsUri),
        )?;
        Ok(__cordl_ret)
    }
    pub fn MatchPrefix(
        &mut self,
        prefix: *mut crate::System::String,
        lname: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MatchPrefix",
            (prefix, lname),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Set(
        &mut self,
        prefix: *mut crate::System::String,
        lname: *mut crate::System::String,
        nsUri: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Set",
            (prefix, lname, nsUri),
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
    pub fn _ctor(
        &mut self,
        prefix: *mut crate::System::String,
        lname: *mut crate::System::String,
        nsUri: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (prefix, lname, nsUri),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+XmlSqlBinaryReader+ScanState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmlSqlBinaryReader_ScanState {
    Attr = 2i32,
    AttrVal = 3i32,
    AttrValPseudoValue = 4i32,
    Closed = 8i32,
    Doc = 0i32,
    _cordl_EOF = 7i32,
    Error = 6i32,
    Init = 5i32,
    XmlText = 1i32,
}
#[cfg(feature = "System+Xml+XmlSqlBinaryReader+ScanState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlSqlBinaryReader_ScanState =>
    "System.Xml"."XmlSqlBinaryReader/ScanState"
);
#[cfg(feature = "System+Xml+XmlSqlBinaryReader+SymbolTables")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct XmlSqlBinaryReader_SymbolTables {
    pub symtable: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    pub symCount: i32,
    pub qnametable: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::System::Xml::XmlSqlBinaryReader_QName,
    >,
    pub qnameCount: i32,
}
#[cfg(feature = "System+Xml+XmlSqlBinaryReader+SymbolTables")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlSqlBinaryReader_SymbolTables =>
    "System.Xml"."XmlSqlBinaryReader/SymbolTables"
);
#[cfg(feature = "System+Xml+XmlSqlBinaryReader+SymbolTables")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Xml::XmlSqlBinaryReader_SymbolTables {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Xml+XmlSqlBinaryReader+SymbolTables")]
impl crate::System::Xml::XmlSqlBinaryReader_SymbolTables {
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Init",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+XmlSqlBinaryReader")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSqlBinaryReader {
    __cordl_parent: crate::System::Xml::XmlReader,
    pub inStrm: *mut crate::System::IO::Stream,
    pub data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub pos: i32,
    pub mark: i32,
    pub end: i32,
    pub offset: i64,
    pub eof: bool,
    pub sniffed: bool,
    pub isEmpty: bool,
    pub docState: i32,
    pub symbolTables: crate::System::Xml::XmlSqlBinaryReader_SymbolTables,
    pub xnt: *mut crate::System::Xml::XmlNameTable,
    pub xntFromSettings: bool,
    pub xml: *mut crate::System::String,
    pub xmlns: *mut crate::System::String,
    pub nsxmlns: *mut crate::System::String,
    pub baseUri: *mut crate::System::String,
    pub state: crate::System::Xml::XmlSqlBinaryReader_ScanState,
    pub nodetype: crate::System::Xml::XmlNodeType,
    pub token: crate::System::Xml::BinXmlToken,
    pub attrIndex: i32,
    pub qnameOther: crate::System::Xml::XmlSqlBinaryReader_QName,
    pub qnameElement: crate::System::Xml::XmlSqlBinaryReader_QName,
    pub parentNodeType: crate::System::Xml::XmlNodeType,
    pub elementStack: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::System::Xml::XmlSqlBinaryReader_ElemInfo,
    >,
    pub elemDepth: i32,
    pub attributes: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::System::Xml::XmlSqlBinaryReader_AttrInfo,
    >,
    pub attrHashTbl: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub attrCount: i32,
    pub posAfterAttrs: i32,
    pub xmlspacePreserve: bool,
    pub tokLen: i32,
    pub tokDataPos: i32,
    pub hasTypedValue: bool,
    pub valueType: *mut crate::System::Type,
    pub stringValue: *mut crate::System::String,
    pub namespaces: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut crate::System::Xml::XmlSqlBinaryReader_NamespaceDecl,
    >,
    pub prevNameInfo: *mut crate::System::Xml::XmlSqlBinaryReader_NestedBinXml,
    pub textXmlReader: *mut crate::System::Xml::XmlReader,
    pub closeInput: bool,
    pub checkCharacters: bool,
    pub ignoreWhitespace: bool,
    pub ignorePIs: bool,
    pub ignoreComments: bool,
    pub dtdProcessing: crate::System::Xml::DtdProcessing,
    pub hasher: *mut crate::System::Xml::SecureStringHasher,
    pub xmlCharType: crate::System::Xml::XmlCharType,
    pub unicode: *mut crate::System::Text::Encoding,
    pub version: u8,
}
#[cfg(feature = "System+Xml+XmlSqlBinaryReader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlSqlBinaryReader => "System.Xml"
    ."XmlSqlBinaryReader"
);
#[cfg(feature = "System+Xml+XmlSqlBinaryReader")]
impl std::ops::Deref for crate::System::Xml::XmlSqlBinaryReader {
    type Target = crate::System::Xml::XmlReader;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlSqlBinaryReader")]
impl std::ops::DerefMut for crate::System::Xml::XmlSqlBinaryReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlSqlBinaryReader")]
impl crate::System::Xml::XmlSqlBinaryReader {
    #[cfg(feature = "System+Xml+XmlSqlBinaryReader+AttrInfo")]
    pub type AttrInfo = crate::System::Xml::XmlSqlBinaryReader_AttrInfo;
    #[cfg(feature = "System+Xml+XmlSqlBinaryReader+SymbolTables")]
    pub type SymbolTables = crate::System::Xml::XmlSqlBinaryReader_SymbolTables;
    #[cfg(feature = "System+Xml+XmlSqlBinaryReader+NestedBinXml")]
    pub type NestedBinXml = crate::System::Xml::XmlSqlBinaryReader_NestedBinXml;
    #[cfg(feature = "System+Xml+XmlSqlBinaryReader+ScanState")]
    pub type ScanState = crate::System::Xml::XmlSqlBinaryReader_ScanState;
    #[cfg(feature = "System+Xml+XmlSqlBinaryReader+ElemInfo")]
    pub type ElemInfo = crate::System::Xml::XmlSqlBinaryReader_ElemInfo;
    #[cfg(feature = "System+Xml+XmlSqlBinaryReader+NamespaceDecl")]
    pub type NamespaceDecl = crate::System::Xml::XmlSqlBinaryReader_NamespaceDecl;
    #[cfg(feature = "System+Xml+XmlSqlBinaryReader+QName")]
    pub type QName = crate::System::Xml::XmlSqlBinaryReader_QName;
    pub fn AddInitNamespace(
        &mut self,
        prefix: *mut crate::System::String,
        uri: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddInitNamespace", (prefix, uri))?;
        Ok(__cordl_ret)
    }
    pub fn AddName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddName", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddQName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddQName", ())?;
        Ok(__cordl_ret)
    }
    pub fn CDATAValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("CDATAValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn CheckAllowContent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckAllowContent", ())?;
        Ok(__cordl_ret)
    }
    pub fn CheckText(
        &mut self,
        attr: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::XmlNodeType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::XmlNodeType = __cordl_object
            .invoke("CheckText", (attr))?;
        Ok(__cordl_ret)
    }
    pub fn CheckTextIsWS(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::XmlNodeType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::XmlNodeType = __cordl_object
            .invoke("CheckTextIsWS", ())?;
        Ok(__cordl_ret)
    }
    pub fn CheckValueTokenBounds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckValueTokenBounds", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClearAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearAttributes", ())?;
        Ok(__cordl_ret)
    }
    pub fn Close(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Close", ())?;
        Ok(__cordl_ret)
    }
    pub fn Fill(
        &mut self,
        require: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Fill", (require))?;
        Ok(__cordl_ret)
    }
    pub fn FillAllowEOF(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("FillAllowEOF", ())?;
        Ok(__cordl_ret)
    }
    pub fn Fill_(
        &mut self,
        require: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Fill_", (require))?;
        Ok(__cordl_ret)
    }
    pub fn FinishCDATA(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinishCDATA", ())?;
        Ok(__cordl_ret)
    }
    pub fn FinishEndElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinishEndElement", ())?;
        Ok(__cordl_ret)
    }
    pub fn GenerateImpliedXmlnsAttrs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateImpliedXmlnsAttrs", ())?;
        Ok(__cordl_ret)
    }
    pub fn GenerateTokenTypeMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateTokenTypeMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetAttributeText(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetAttributeText", (i))?;
        Ok(__cordl_ret)
    }
    pub fn GetAttribute_String1(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetAttribute", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetAttribute_String_String0(
        &mut self,
        name: *mut crate::System::String,
        ns: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetAttribute", (name, ns))?;
        Ok(__cordl_ret)
    }
    pub fn GetAttribute_i32_2(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetAttribute", (i))?;
        Ok(__cordl_ret)
    }
    pub fn GetDouble(&mut self, offset: i32) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("GetDouble", (offset))?;
        Ok(__cordl_ret)
    }
    pub fn GetInt16(&mut self, pos: i32) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i16 = __cordl_object.invoke("GetInt16", (pos))?;
        Ok(__cordl_ret)
    }
    pub fn GetInt32(&mut self, pos: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetInt32", (pos))?;
        Ok(__cordl_ret)
    }
    pub fn GetInt64(&mut self, pos: i32) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("GetInt64", (pos))?;
        Ok(__cordl_ret)
    }
    pub fn GetSingle(&mut self, offset: i32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetSingle", (offset))?;
        Ok(__cordl_ret)
    }
    pub fn GetString(
        &mut self,
        pos: i32,
        cch: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetString", (pos, cch))?;
        Ok(__cordl_ret)
    }
    pub fn GetStringAligned(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        cch: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetStringAligned", (data, offset, cch))?;
        Ok(__cordl_ret)
    }
    pub fn GetUInt16(&mut self, pos: i32) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u16 = __cordl_object.invoke("GetUInt16", (pos))?;
        Ok(__cordl_ret)
    }
    pub fn GetUInt32(&mut self, pos: i32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("GetUInt32", (pos))?;
        Ok(__cordl_ret)
    }
    pub fn GetUInt64(&mut self, pos: i32) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object.invoke("GetUInt64", (pos))?;
        Ok(__cordl_ret)
    }
    pub fn GetValueType(
        &mut self,
        token: crate::System::Xml::BinXmlToken,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("GetValueType", (token))?;
        Ok(__cordl_ret)
    }
    pub fn GetXsdKatmaiTokenLength(
        &mut self,
        token: crate::System::Xml::BinXmlToken,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetXsdKatmaiTokenLength", (token))?;
        Ok(__cordl_ret)
    }
    pub fn GrowAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GrowAttributes", ())?;
        Ok(__cordl_ret)
    }
    pub fn GrowElements(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GrowElements", ())?;
        Ok(__cordl_ret)
    }
    pub fn HashCheckForDuplicateAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HashCheckForDuplicateAttributes", ())?;
        Ok(__cordl_ret)
    }
    pub fn ImplReadCDATA(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ImplReadCDATA", ())?;
        Ok(__cordl_ret)
    }
    pub fn ImplReadComment(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ImplReadComment", ())?;
        Ok(__cordl_ret)
    }
    pub fn ImplReadData(
        &mut self,
        tokenType: crate::System::Xml::BinXmlToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ImplReadData", (tokenType))?;
        Ok(__cordl_ret)
    }
    pub fn ImplReadDoctype(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ImplReadDoctype", ())?;
        Ok(__cordl_ret)
    }
    pub fn ImplReadElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ImplReadElement", ())?;
        Ok(__cordl_ret)
    }
    pub fn ImplReadEndElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ImplReadEndElement", ())?;
        Ok(__cordl_ret)
    }
    pub fn ImplReadEndNest(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ImplReadEndNest", ())?;
        Ok(__cordl_ret)
    }
    pub fn ImplReadNest(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ImplReadNest", ())?;
        Ok(__cordl_ret)
    }
    pub fn ImplReadPI(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ImplReadPI", ())?;
        Ok(__cordl_ret)
    }
    pub fn ImplReadXmlText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ImplReadXmlText", ())?;
        Ok(__cordl_ret)
    }
    pub fn LocateAttribute_String0(
        &mut self,
        name: *mut crate::System::String,
        ns: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("LocateAttribute", (name, ns))?;
        Ok(__cordl_ret)
    }
    pub fn LocateAttribute_String1(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("LocateAttribute", (name))?;
        Ok(__cordl_ret)
    }
    pub fn LookupNamespace(
        &mut self,
        prefix: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("LookupNamespace", (prefix))?;
        Ok(__cordl_ret)
    }
    pub fn MoveToAttribute_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveToAttribute", (name))?;
        Ok(__cordl_ret)
    }
    pub fn MoveToAttribute_i32_1(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MoveToAttribute", (i))?;
        Ok(__cordl_ret)
    }
    pub fn MoveToElement(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveToElement", ())?;
        Ok(__cordl_ret)
    }
    pub fn MoveToFirstAttribute(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveToFirstAttribute", ())?;
        Ok(__cordl_ret)
    }
    pub fn MoveToNextAttribute(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveToNextAttribute", ())?;
        Ok(__cordl_ret)
    }
    pub fn NameFlush(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NameFlush", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        stream: *mut crate::System::IO::Stream,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        len: i32,
        baseUri: *mut crate::System::String,
        closeInput: bool,
        settings: *mut crate::System::Xml::XmlReaderSettings,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (stream, data, len, baseUri, closeInput, settings))?;
        Ok(__cordl_object)
    }
    pub fn NextToken(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::BinXmlToken> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::BinXmlToken = __cordl_object
            .invoke("NextToken", ())?;
        Ok(__cordl_ret)
    }
    pub fn NextToken1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::BinXmlToken> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::BinXmlToken = __cordl_object
            .invoke("NextToken1", ())?;
        Ok(__cordl_ret)
    }
    pub fn NextToken2(
        &mut self,
        token: crate::System::Xml::BinXmlToken,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::BinXmlToken> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::BinXmlToken = __cordl_object
            .invoke("NextToken2", (token))?;
        Ok(__cordl_ret)
    }
    pub fn ParseMB32_(&mut self, b: u8) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ParseMB32_", (b))?;
        Ok(__cordl_ret)
    }
    pub fn ParseMB32_0(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ParseMB32", ())?;
        Ok(__cordl_ret)
    }
    pub fn ParseMB32_i32_1(&mut self, pos: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ParseMB32", (pos))?;
        Ok(__cordl_ret)
    }
    pub fn ParseMB64(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ParseMB64", ())?;
        Ok(__cordl_ret)
    }
    pub fn ParseText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ParseText", ())?;
        Ok(__cordl_ret)
    }
    pub fn PeekNextToken(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::BinXmlToken> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::BinXmlToken = __cordl_object
            .invoke("PeekNextToken", ())?;
        Ok(__cordl_ret)
    }
    pub fn PeekToken(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::BinXmlToken> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::BinXmlToken = __cordl_object
            .invoke("PeekToken", ())?;
        Ok(__cordl_ret)
    }
    pub fn PopNamespaces(
        &mut self,
        firstInScopeChain: *mut crate::System::Xml::XmlSqlBinaryReader_NamespaceDecl,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopNamespaces", (firstInScopeChain))?;
        Ok(__cordl_ret)
    }
    pub fn PositionOnAttribute(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PositionOnAttribute", (i))?;
        Ok(__cordl_ret)
    }
    pub fn PushNamespace(
        &mut self,
        prefix: *mut crate::System::String,
        ns: *mut crate::System::String,
        implied: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PushNamespace", (prefix, ns, implied))?;
        Ok(__cordl_ret)
    }
    pub fn ReScanOverValue(
        &mut self,
        token: crate::System::Xml::BinXmlToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReScanOverValue", (token))?;
        Ok(__cordl_ret)
    }
    pub fn Read(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Read", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadAttributeValue(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ReadAttributeValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadByte(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("ReadByte", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadDoc(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ReadDoc", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadInit(
        &mut self,
        skipXmlDecl: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ReadInit", (skipXmlDecl))?;
        Ok(__cordl_ret)
    }
    pub fn ReadNameRef(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ReadNameRef", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadQNameRef(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ReadQNameRef", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadToken(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::BinXmlToken> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::BinXmlToken = __cordl_object
            .invoke("ReadToken", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadUShort(&mut self) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u16 = __cordl_object.invoke("ReadUShort", ())?;
        Ok(__cordl_ret)
    }
    pub fn RescanNextToken(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::BinXmlToken> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::BinXmlToken = __cordl_object
            .invoke("RescanNextToken", ())?;
        Ok(__cordl_ret)
    }
    pub fn ResolveEntity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResolveEntity", ())?;
        Ok(__cordl_ret)
    }
    pub fn ScanAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScanAttributes", ())?;
        Ok(__cordl_ret)
    }
    pub fn ScanOverAnyValue(
        &mut self,
        token: crate::System::Xml::BinXmlToken,
        attr: bool,
        checkChars: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::XmlNodeType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::XmlNodeType = __cordl_object
            .invoke("ScanOverAnyValue", (token, attr, checkChars))?;
        Ok(__cordl_ret)
    }
    pub fn ScanOverValue(
        &mut self,
        token: crate::System::Xml::BinXmlToken,
        attr: bool,
        checkChars: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::XmlNodeType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::XmlNodeType = __cordl_object
            .invoke("ScanOverValue", (token, attr, checkChars))?;
        Ok(__cordl_ret)
    }
    pub fn ScanText(
        &mut self,
        start: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ScanText", (start))?;
        Ok(__cordl_ret)
    }
    pub fn SimpleCheckForDuplicateAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SimpleCheckForDuplicateAttributes", ())?;
        Ok(__cordl_ret)
    }
    pub fn SkipExtn(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SkipExtn", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IXmlNamespaceResolver_GetNamespacesInScope(
        &mut self,
        scope: crate::System::Xml::XmlNamespaceScope,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IDictionary_2<
            *mut crate::System::String,
            *mut crate::System::String,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IDictionary_2<
            *mut crate::System::String,
            *mut crate::System::String,
        > = __cordl_object
            .invoke("System.Xml.IXmlNamespaceResolver.GetNamespacesInScope", (scope))?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IXmlNamespaceResolver_LookupPrefix(
        &mut self,
        namespaceName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("System.Xml.IXmlNamespaceResolver.LookupPrefix", (namespaceName))?;
        Ok(__cordl_ret)
    }
    pub fn ThrowNotSupported(
        &mut self,
        res: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Exception = __cordl_object
            .invoke("ThrowNotSupported", (res))?;
        Ok(__cordl_ret)
    }
    pub fn ThrowUnexpectedToken(
        &mut self,
        token: crate::System::Xml::BinXmlToken,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Exception = __cordl_object
            .invoke("ThrowUnexpectedToken", (token))?;
        Ok(__cordl_ret)
    }
    pub fn ThrowXmlException_String0(
        &mut self,
        res: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Exception = __cordl_object
            .invoke("ThrowXmlException", (res))?;
        Ok(__cordl_ret)
    }
    pub fn ThrowXmlException_String_String1(
        &mut self,
        res: *mut crate::System::String,
        arg1: *mut crate::System::String,
        arg2: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Exception = __cordl_object
            .invoke("ThrowXmlException", (res, arg1, arg2))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateFromTextReader_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateFromTextReader", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateFromTextReader__cordl_bool1(
        &mut self,
        needUpdate: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("UpdateFromTextReader", (needUpdate))?;
        Ok(__cordl_ret)
    }
    pub fn ValueAsDateTimeString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ValueAsDateTimeString", ())?;
        Ok(__cordl_ret)
    }
    pub fn ValueAsDecimal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Decimal = __cordl_object
            .invoke("ValueAsDecimal", ())?;
        Ok(__cordl_ret)
    }
    pub fn ValueAsDouble(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("ValueAsDouble", ())?;
        Ok(__cordl_ret)
    }
    pub fn ValueAsLong(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("ValueAsLong", ())?;
        Ok(__cordl_ret)
    }
    pub fn ValueAsString(
        &mut self,
        token: crate::System::Xml::BinXmlToken,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ValueAsString", (token))?;
        Ok(__cordl_ret)
    }
    pub fn ValueAsULong(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object.invoke("ValueAsULong", ())?;
        Ok(__cordl_ret)
    }
    pub fn VerifyVersion(
        &mut self,
        requiredVersion: i32,
        token: crate::System::Xml::BinXmlToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("VerifyVersion", (requiredVersion, token))?;
        Ok(__cordl_ret)
    }
    pub fn XmlDeclValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("XmlDeclValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn XsdKatmaiTimeScaleToValueLength(
        &mut self,
        scale: u8,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("XsdKatmaiTimeScaleToValueLength", (scale))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        stream: *mut crate::System::IO::Stream,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        len: i32,
        baseUri: *mut crate::System::String,
        closeInput: bool,
        settings: *mut crate::System::Xml::XmlReaderSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (stream, data, len, baseUri, closeInput, settings))?;
        Ok(__cordl_ret)
    }
    pub fn get_AttributeCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_AttributeCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_BaseURI(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_BaseURI", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Depth(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Depth", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_EOF(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_EOF", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsEmptyElement(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsEmptyElement", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LocalName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_LocalName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_NameTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlNameTable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlNameTable = __cordl_object
            .invoke("get_NameTable", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_NamespaceURI(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_NamespaceURI", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_NodeType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::XmlNodeType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::XmlNodeType = __cordl_object
            .invoke("get_NodeType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Prefix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Prefix", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ReadState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::ReadState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::ReadState = __cordl_object
            .invoke("get_ReadState", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Settings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlReaderSettings> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlReaderSettings = __cordl_object
            .invoke("get_Settings", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Value", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ValueType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_ValueType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_XmlLang(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_XmlLang", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_XmlSpace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::XmlSpace> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::XmlSpace = __cordl_object
            .invoke("get_XmlSpace", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+XmlSqlBinaryReader")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::XmlSqlBinaryReader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
