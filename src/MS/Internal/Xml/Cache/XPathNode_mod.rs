#[cfg(feature = "MS+Internal+Xml+Cache+XPathNode")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct XPathNode {
    pub _info: quest_hook::libil2cpp::Gc<
        crate::MS::Internal::Xml::Cache::XPathNodeInfoAtom,
    >,
    pub _idxSibling: u16,
    pub _idxParent: u16,
    pub _idxSimilar: u16,
    pub _posOffset: u16,
    pub _props: u32,
    pub _value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "MS+Internal+Xml+Cache+XPathNode")]
unsafe impl quest_hook::libil2cpp::Type for crate::MS::Internal::Xml::Cache::XPathNode {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "MS.Internal.Xml.Cache";
    const CLASS_NAME: &'static str = "XPathNode";
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
#[cfg(feature = "MS+Internal+Xml+Cache+XPathNode")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::MS::Internal::Xml::Cache::XPathNode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "MS+Internal+Xml+Cache+XPathNode")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::MS::Internal::Xml::Cache::XPathNode {
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
#[cfg(feature = "MS+Internal+Xml+Cache+XPathNode")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::MS::Internal::Xml::Cache::XPathNode {
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
#[cfg(feature = "MS+Internal+Xml+Cache+XPathNode")]
unsafe impl quest_hook::libil2cpp::Return
for crate::MS::Internal::Xml::Cache::XPathNode {
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
#[cfg(feature = "MS+Internal+Xml+Cache+XPathNode")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::MS::Internal::Xml::Cache::XPathNode {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "MS+Internal+Xml+Cache+XPathNode")]
impl crate::MS::Internal::Xml::Cache::XPathNode {
    pub fn GetParent(
        &mut self,
        pageNode: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::MS::Internal::Xml::Cache::XPathNode,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetParent",
            (pageNode),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSibling(
        &mut self,
        pageNode: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::MS::Internal::Xml::Cache::XPathNode,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetSibling",
            (pageNode),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CollapsedLinePosition(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_CollapsedLinePosition",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Document(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XPath::XPathDocument>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::XPath::XPathDocument,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Document", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasCollapsedText(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_HasCollapsedText",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasNamespaceDecls(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_HasNamespaceDecls",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasSibling(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_HasSibling",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsText(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsText",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsXmlNamespaceNode(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsXmlNamespaceNode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LineNumber(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_LineNumber",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LinePosition(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_LinePosition",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LocalName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_LocalName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NamespaceUri(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_NamespaceUri", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NodeType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::XPath::XPathNodeType> {
        let __cordl_ret: crate::System::Xml::XPath::XPathNodeType = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_NodeType",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PageInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::Cache::XPathNodePageInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::MS::Internal::Xml::Cache::XPathNodePageInfo,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_PageInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Prefix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Prefix", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Value", ())?;
        Ok(__cordl_ret.into())
    }
}
