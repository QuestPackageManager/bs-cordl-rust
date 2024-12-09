#[cfg(feature = "System+Xml+Schema+XmlSchemaObjectTable")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSchemaObjectTable {
    __cordl_parent: crate::System::Object,
    pub table: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::Xml::XmlQualifiedName,
        *mut crate::System::Xml::Schema::XmlSchemaObject,
    >,
    pub entries: *mut crate::System::Collections::Generic::List_1<
        crate::System::Xml::Schema::XmlSchemaObjectTable_XmlSchemaObjectEntry,
    >,
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaObjectTable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XmlSchemaObjectTable =>
    "System.Xml.Schema"."XmlSchemaObjectTable"
);
#[cfg(feature = "System+Xml+Schema+XmlSchemaObjectTable")]
impl std::ops::Deref for crate::System::Xml::Schema::XmlSchemaObjectTable {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaObjectTable")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XmlSchemaObjectTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaObjectTable")]
impl crate::System::Xml::Schema::XmlSchemaObjectTable {
    #[cfg(feature = "System+Xml+Schema+XmlSchemaObjectTable+EnumeratorType")]
    pub type EnumeratorType = crate::System::Xml::Schema::XmlSchemaObjectTable_EnumeratorType;
    #[cfg(feature = "System+Xml+Schema+XmlSchemaObjectTable+ValuesCollection")]
    pub type ValuesCollection = crate::System::Xml::Schema::XmlSchemaObjectTable_ValuesCollection;
    #[cfg(feature = "System+Xml+Schema+XmlSchemaObjectTable+XSODictionaryEnumerator")]
    pub type XSODictionaryEnumerator = crate::System::Xml::Schema::XmlSchemaObjectTable_XSODictionaryEnumerator;
    #[cfg(feature = "System+Xml+Schema+XmlSchemaObjectTable+XSOEnumerator")]
    pub type XSOEnumerator = crate::System::Xml::Schema::XmlSchemaObjectTable_XSOEnumerator;
    #[cfg(feature = "System+Xml+Schema+XmlSchemaObjectTable+XmlSchemaObjectEntry")]
    pub type XmlSchemaObjectEntry = crate::System::Xml::Schema::XmlSchemaObjectTable_XmlSchemaObjectEntry;
    pub fn Add(
        &mut self,
        name: *mut crate::System::Xml::XmlQualifiedName,
        value: *mut crate::System::Xml::Schema::XmlSchemaObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret)
    }
    pub fn Contains(
        &mut self,
        name: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Contains", (name))?;
        Ok(__cordl_ret)
    }
    pub fn FindIndexByValue(
        &mut self,
        xso: *mut crate::System::Xml::Schema::XmlSchemaObject,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("FindIndexByValue", (xso))?;
        Ok(__cordl_ret)
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::IDictionaryEnumerator,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IDictionaryEnumerator = __cordl_object
            .invoke("GetEnumerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn Insert(
        &mut self,
        name: *mut crate::System::Xml::XmlQualifiedName,
        value: *mut crate::System::Xml::Schema::XmlSchemaObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Insert", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Remove(
        &mut self,
        name: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Remove", (name))?;
        Ok(__cordl_ret)
    }
    pub fn Replace(
        &mut self,
        name: *mut crate::System::Xml::XmlQualifiedName,
        value: *mut crate::System::Xml::Schema::XmlSchemaObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Replace", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Item(
        &mut self,
        name: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaObject,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaObject = __cordl_object
            .invoke("get_Item", (name))?;
        Ok(__cordl_ret)
    }
    pub fn get_Values(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::ICollection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ICollection = __cordl_object
            .invoke("get_Values", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaObjectTable")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XmlSchemaObjectTable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaObjectTable+EnumeratorType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmlSchemaObjectTable_EnumeratorType {
    DictionaryEntry = 2i32,
    Keys = 0i32,
    Values = 1i32,
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaObjectTable+EnumeratorType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Schema::XmlSchemaObjectTable_EnumeratorType => "System.Xml.Schema"
    ."XmlSchemaObjectTable/EnumeratorType"
);
#[cfg(feature = "System+Xml+Schema+XmlSchemaObjectTable+ValuesCollection")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSchemaObjectTable_ValuesCollection {
    __cordl_parent: crate::System::Object,
    pub entries: *mut crate::System::Collections::Generic::List_1<
        crate::System::Xml::Schema::XmlSchemaObjectTable_XmlSchemaObjectEntry,
    >,
    pub _cordl_size: i32,
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaObjectTable+ValuesCollection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Schema::XmlSchemaObjectTable_ValuesCollection => "System.Xml.Schema"
    ."XmlSchemaObjectTable/ValuesCollection"
);
#[cfg(feature = "System+Xml+Schema+XmlSchemaObjectTable+ValuesCollection")]
impl std::ops::Deref
for crate::System::Xml::Schema::XmlSchemaObjectTable_ValuesCollection {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaObjectTable+ValuesCollection")]
impl std::ops::DerefMut
for crate::System::Xml::Schema::XmlSchemaObjectTable_ValuesCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaObjectTable+ValuesCollection")]
impl crate::System::Xml::Schema::XmlSchemaObjectTable_ValuesCollection {
    pub fn CopyTo(
        &mut self,
        array: *mut crate::System::Array,
        arrayIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyTo", (array, arrayIndex))?;
        Ok(__cordl_ret)
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("GetEnumerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        entries: *mut crate::System::Collections::Generic::List_1<
            crate::System::Xml::Schema::XmlSchemaObjectTable_XmlSchemaObjectEntry,
        >,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (entries, _cordl_size))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        entries: *mut crate::System::Collections::Generic::List_1<
            crate::System::Xml::Schema::XmlSchemaObjectTable_XmlSchemaObjectEntry,
        >,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (entries, _cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsSynchronized(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsSynchronized", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SyncRoot(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_SyncRoot", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaObjectTable+ValuesCollection")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XmlSchemaObjectTable_ValuesCollection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaObjectTable+XSODictionaryEnumerator")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSchemaObjectTable_XSODictionaryEnumerator {
    __cordl_parent: crate::System::Xml::Schema::XmlSchemaObjectTable_XSOEnumerator,
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaObjectTable+XSODictionaryEnumerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Schema::XmlSchemaObjectTable_XSODictionaryEnumerator =>
    "System.Xml.Schema"."XmlSchemaObjectTable/XSODictionaryEnumerator"
);
#[cfg(feature = "System+Xml+Schema+XmlSchemaObjectTable+XSODictionaryEnumerator")]
impl std::ops::Deref
for crate::System::Xml::Schema::XmlSchemaObjectTable_XSODictionaryEnumerator {
    type Target = crate::System::Xml::Schema::XmlSchemaObjectTable_XSOEnumerator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaObjectTable+XSODictionaryEnumerator")]
impl std::ops::DerefMut
for crate::System::Xml::Schema::XmlSchemaObjectTable_XSODictionaryEnumerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaObjectTable+XSODictionaryEnumerator")]
impl crate::System::Xml::Schema::XmlSchemaObjectTable_XSODictionaryEnumerator {
    pub fn New(
        entries: *mut crate::System::Collections::Generic::List_1<
            crate::System::Xml::Schema::XmlSchemaObjectTable_XmlSchemaObjectEntry,
        >,
        _cordl_size: i32,
        enumType: crate::System::Xml::Schema::XmlSchemaObjectTable_EnumeratorType,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (entries, _cordl_size, enumType))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        entries: *mut crate::System::Collections::Generic::List_1<
            crate::System::Xml::Schema::XmlSchemaObjectTable_XmlSchemaObjectEntry,
        >,
        _cordl_size: i32,
        enumType: crate::System::Xml::Schema::XmlSchemaObjectTable_EnumeratorType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (entries, _cordl_size, enumType))?;
        Ok(__cordl_ret)
    }
    pub fn get_Entry(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Collections::DictionaryEntry> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Collections::DictionaryEntry = __cordl_object
            .invoke("get_Entry", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Key(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_Key", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_Value", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaObjectTable+XSODictionaryEnumerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XmlSchemaObjectTable_XSODictionaryEnumerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaObjectTable+XSOEnumerator")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSchemaObjectTable_XSOEnumerator {
    __cordl_parent: crate::System::Object,
    pub entries: *mut crate::System::Collections::Generic::List_1<
        crate::System::Xml::Schema::XmlSchemaObjectTable_XmlSchemaObjectEntry,
    >,
    pub enumType: crate::System::Xml::Schema::XmlSchemaObjectTable_EnumeratorType,
    pub currentIndex: i32,
    pub _cordl_size: i32,
    pub currentKey: *mut crate::System::Xml::XmlQualifiedName,
    pub currentValue: *mut crate::System::Xml::Schema::XmlSchemaObject,
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaObjectTable+XSOEnumerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Schema::XmlSchemaObjectTable_XSOEnumerator => "System.Xml.Schema"
    ."XmlSchemaObjectTable/XSOEnumerator"
);
#[cfg(feature = "System+Xml+Schema+XmlSchemaObjectTable+XSOEnumerator")]
impl std::ops::Deref for crate::System::Xml::Schema::XmlSchemaObjectTable_XSOEnumerator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaObjectTable+XSOEnumerator")]
impl std::ops::DerefMut
for crate::System::Xml::Schema::XmlSchemaObjectTable_XSOEnumerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaObjectTable+XSOEnumerator")]
impl crate::System::Xml::Schema::XmlSchemaObjectTable_XSOEnumerator {
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveNext", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        entries: *mut crate::System::Collections::Generic::List_1<
            crate::System::Xml::Schema::XmlSchemaObjectTable_XmlSchemaObjectEntry,
        >,
        _cordl_size: i32,
        enumType: crate::System::Xml::Schema::XmlSchemaObjectTable_EnumeratorType,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (entries, _cordl_size, enumType))?;
        Ok(__cordl_object)
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        entries: *mut crate::System::Collections::Generic::List_1<
            crate::System::Xml::Schema::XmlSchemaObjectTable_XmlSchemaObjectEntry,
        >,
        _cordl_size: i32,
        enumType: crate::System::Xml::Schema::XmlSchemaObjectTable_EnumeratorType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (entries, _cordl_size, enumType))?;
        Ok(__cordl_ret)
    }
    pub fn get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_Current", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaObjectTable+XSOEnumerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XmlSchemaObjectTable_XSOEnumerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaObjectTable+XmlSchemaObjectEntry")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct XmlSchemaObjectTable_XmlSchemaObjectEntry {
    pub qname: *mut crate::System::Xml::XmlQualifiedName,
    pub xso: *mut crate::System::Xml::Schema::XmlSchemaObject,
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaObjectTable+XmlSchemaObjectEntry")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Schema::XmlSchemaObjectTable_XmlSchemaObjectEntry =>
    "System.Xml.Schema"."XmlSchemaObjectTable/XmlSchemaObjectEntry"
);
#[cfg(feature = "System+Xml+Schema+XmlSchemaObjectTable+XmlSchemaObjectEntry")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Xml::Schema::XmlSchemaObjectTable_XmlSchemaObjectEntry {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaObjectTable+XmlSchemaObjectEntry")]
impl crate::System::Xml::Schema::XmlSchemaObjectTable_XmlSchemaObjectEntry {
    pub fn _ctor(
        &mut self,
        name: *mut crate::System::Xml::XmlQualifiedName,
        value: *mut crate::System::Xml::Schema::XmlSchemaObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (name, value),
        )?;
        Ok(__cordl_ret)
    }
}
