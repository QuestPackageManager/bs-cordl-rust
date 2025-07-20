#[cfg(feature = "System+Data+XmlDataLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlDataLoader {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _dataSet: quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
    pub _nodeToSchemaMap: quest_hook::libil2cpp::Gc<
        crate::System::Data::XmlToDatasetMap,
    >,
    pub _nodeToRowMap: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
    pub _childRowsStack: quest_hook::libil2cpp::Gc<crate::System::Collections::Stack>,
    pub _htableExcludedNS: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Hashtable,
    >,
    pub _fIsXdr: bool,
    pub _isDiffgram: bool,
    pub _topMostNode: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
    pub _ignoreSchema: bool,
    pub _dataTable: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    pub _isTableLevel: bool,
    pub _fromInference: bool,
    pub _dataReader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
    pub _XSD_XMLNS_NS: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _XDR_SCHEMA: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _XDRNS: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _SQL_SYNC: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _UPDGNS: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _XSD_SCHEMA: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _XSDNS: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _DFFNS: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _MSDNS: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _DIFFID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _HASCHANGES: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _ROWORDER: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Data+XmlDataLoader")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Data::XmlDataLoader {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Data";
    const CLASS_NAME: &'static str = "XmlDataLoader";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+Data+XmlDataLoader")]
impl std::ops::Deref for crate::System::Data::XmlDataLoader {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+XmlDataLoader")]
impl std::ops::DerefMut for crate::System::Data::XmlDataLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+XmlDataLoader")]
impl crate::System::Data::XmlDataLoader {
    pub fn AttachRows(
        &mut self,
        parentRow: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        parentElement: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
                    quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("AttachRows")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type >
                    ::class(), "AttachRows", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (parentRow, parentElement))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CountNonNSAttributes(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>),
                i32,
                1usize,
            >("CountNonNSAttributes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type >
                    ::class(), "CountNonNSAttributes", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (node))? };
        Ok(__cordl_ret.into())
    }
    pub fn FColumnElement(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>),
                bool,
                1usize,
            >("FColumnElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type >
                    ::class(), "FColumnElement", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (e))? };
        Ok(__cordl_ret.into())
    }
    pub fn FExcludedNamespace(
        &mut self,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                bool,
                1usize,
            >("FExcludedNamespace")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type >
                    ::class(), "FExcludedNamespace", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (ns))? };
        Ok(__cordl_ret.into())
    }
    pub fn FIgnoreNamespace_XmlNode0(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>),
                bool,
                1usize,
            >("FIgnoreNamespace")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type >
                    ::class(), "FIgnoreNamespace", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (node))? };
        Ok(__cordl_ret.into())
    }
    pub fn FIgnoreNamespace_XmlReader1(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>),
                bool,
                1usize,
            >("FIgnoreNamespace")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type >
                    ::class(), "FIgnoreNamespace", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (node))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetInitialTextFromNodes(
        &mut self,
        n: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<
                    quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
                >),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GetInitialTextFromNodes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type >
                    ::class(), "GetInitialTextFromNodes", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (n))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetRowFromElement(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>),
                quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
                1usize,
            >("GetRowFromElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type >
                    ::class(), "GetRowFromElement", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow> = unsafe {
            method.invoke_unchecked(self, (e))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTextOnlyColumn(
        &mut self,
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>),
                quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
                1usize,
            >("GetTextOnlyColumn")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type >
                    ::class(), "GetTextOnlyColumn", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn> = unsafe {
            method.invoke_unchecked(self, (row))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetValueForTextOnlyColums(
        &mut self,
        n: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GetValueForTextOnlyColums")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type >
                    ::class(), "GetValueForTextOnlyColums", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (n))? };
        Ok(__cordl_ret.into())
    }
    pub fn InitNameTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("InitNameTable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type >
                    ::class(), "InitNameTable", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsTextLikeNode(
        &mut self,
        n: crate::System::Xml::XmlNodeType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::Xml::XmlNodeType),
                bool,
                1usize,
            >("IsTextLikeNode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type >
                    ::class(), "IsTextLikeNode", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (n))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsTextOnly(
        &mut self,
        c: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>),
                bool,
                1usize,
            >("IsTextOnly")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type >
                    ::class(), "IsTextOnly", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (c))? };
        Ok(__cordl_ret.into())
    }
    pub fn LoadColumn(
        &mut self,
        column: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
        foundColumns: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("LoadColumn")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type >
                    ::class(), "LoadColumn", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (column, foundColumns))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadData_XmlDocument0(
        &mut self,
        xdoc: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocument>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocument>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("LoadData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type >
                    ::class(), "LoadData", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (xdoc))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadData_XmlReader1(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("LoadData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type >
                    ::class(), "LoadData", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (reader))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadRowData(
        &mut self,
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        rowElement: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
                    quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("LoadRowData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type >
                    ::class(), "LoadRowData", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (row, rowElement))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadRows(
        &mut self,
        parentRow: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        parentElement: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
                    quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("LoadRows")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type >
                    ::class(), "LoadRows", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (parentRow, parentElement))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadTable(
        &mut self,
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        isNested: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("LoadTable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type >
                    ::class(), "LoadTable", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (table, isNested))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadTopMostTable(
        &mut self,
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("LoadTopMostTable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type >
                    ::class(), "LoadTopMostTable", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (table))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New_DataSet_XmlElement__cordl_bool1(
        dataset: quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
        IsXdr: bool,
        topNode: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
        ignoreSchema: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dataset, IsXdr, topNode, ignoreSchema))?;
        Ok(__cordl_object.into())
    }
    pub fn New_DataSet__cordl_bool0(
        dataset: quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
        IsXdr: bool,
        ignoreSchema: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dataset, IsXdr, ignoreSchema))?;
        Ok(__cordl_object.into())
    }
    pub fn New_DataTable_XmlElement__cordl_bool3(
        datatable: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        IsXdr: bool,
        topNode: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
        ignoreSchema: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (datatable, IsXdr, topNode, ignoreSchema))?;
        Ok(__cordl_object.into())
    }
    pub fn New_DataTable__cordl_bool2(
        datatable: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        IsXdr: bool,
        ignoreSchema: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (datatable, IsXdr, ignoreSchema))?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessXsdSchema(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("ProcessXsdSchema")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type >
                    ::class(), "ProcessXsdSchema", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRowValueFromXmlText(
        &mut self,
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        col: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
        xmlText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
                    quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetRowValueFromXmlText")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type >
                    ::class(), "SetRowValueFromXmlText", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (row, col, xmlText))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_DataSet_XmlElement__cordl_bool1(
        &mut self,
        dataset: quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
        IsXdr: bool,
        topNode: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
        ignoreSchema: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
                    bool,
                    quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (dataset, IsXdr, topNode, ignoreSchema))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_DataSet__cordl_bool0(
        &mut self,
        dataset: quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
        IsXdr: bool,
        ignoreSchema: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>, bool, bool),
                quest_hook::libil2cpp::Void,
                3usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (dataset, IsXdr, ignoreSchema))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_DataTable_XmlElement__cordl_bool3(
        &mut self,
        datatable: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        IsXdr: bool,
        topNode: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
        ignoreSchema: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
                    bool,
                    quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (datatable, IsXdr, topNode, ignoreSchema))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_DataTable__cordl_bool2(
        &mut self,
        datatable: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        IsXdr: bool,
        ignoreSchema: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>, bool, bool),
                quest_hook::libil2cpp::Void,
                3usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (datatable, IsXdr, ignoreSchema))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_FromInference(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_FromInference")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type >
                    ::class(), "get_FromInference", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_FromInference(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_FromInference")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::XmlDataLoader as quest_hook::libil2cpp::Type >
                    ::class(), "set_FromInference", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Data+XmlDataLoader")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::XmlDataLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
