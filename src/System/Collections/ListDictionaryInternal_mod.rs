#[cfg(feature = "System+Collections+ListDictionaryInternal")]
#[repr(C)]
#[derive(Debug)]
pub struct ListDictionaryInternal {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub head: quest_hook::libil2cpp::Gc<
        crate::System::Collections::ListDictionaryInternal_DictionaryNode,
    >,
    pub version: i32,
    pub count: i32,
    pub _syncRoot: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Collections+ListDictionaryInternal")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Collections::ListDictionaryInternal =>
    "System.Collections"."ListDictionaryInternal"
);
#[cfg(feature = "System+Collections+ListDictionaryInternal")]
impl std::ops::Deref for crate::System::Collections::ListDictionaryInternal {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+ListDictionaryInternal")]
impl std::ops::DerefMut for crate::System::Collections::ListDictionaryInternal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+ListDictionaryInternal")]
impl crate::System::Collections::ListDictionaryInternal {
    #[cfg(feature = "System+Collections+ListDictionaryInternal+DictionaryNode")]
    pub type DictionaryNode = crate::System::Collections::ListDictionaryInternal_DictionaryNode;
    #[cfg(feature = "System+Collections+ListDictionaryInternal+NodeEnumerator")]
    pub type NodeEnumerator = crate::System::Collections::ListDictionaryInternal_NodeEnumerator;
    #[cfg(feature = "System+Collections+ListDictionaryInternal+NodeKeyValueCollection")]
    pub type NodeKeyValueCollection = crate::System::Collections::ListDictionaryInternal_NodeKeyValueCollection;
    pub fn Add(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (key, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Contains(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Contains", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyTo(
        &mut self,
        array: quest_hook::libil2cpp::Gc<crate::System::Array>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyTo", (array, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionaryEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IDictionaryEnumerator,
        > = __cordl_object.invoke("GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Remove(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Remove", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("System.Collections.IEnumerable.GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsFixedSize(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsFixedSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsReadOnly(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsReadOnly", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsSynchronized(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsSynchronized", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_Item", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Keys(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ICollection,
        > = __cordl_object.invoke("get_Keys", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SyncRoot(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_SyncRoot", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Values(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ICollection,
        > = __cordl_object.invoke("get_Values", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Item(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Item", (key, value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Collections+ListDictionaryInternal")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Collections::ListDictionaryInternal {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Collections+ListDictionaryInternal")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection>>
for crate::System::Collections::ListDictionaryInternal {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+ListDictionaryInternal")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection>>
for crate::System::Collections::ListDictionaryInternal {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+ListDictionaryInternal")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>>
for crate::System::Collections::ListDictionaryInternal {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+ListDictionaryInternal")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>>
for crate::System::Collections::ListDictionaryInternal {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+ListDictionaryInternal")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>>
for crate::System::Collections::ListDictionaryInternal {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+ListDictionaryInternal")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>>
for crate::System::Collections::ListDictionaryInternal {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+ListDictionaryInternal+DictionaryNode")]
#[repr(C)]
#[derive(Debug)]
pub struct ListDictionaryInternal_DictionaryNode {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub next: quest_hook::libil2cpp::Gc<
        crate::System::Collections::ListDictionaryInternal_DictionaryNode,
    >,
}
#[cfg(feature = "System+Collections+ListDictionaryInternal+DictionaryNode")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Collections::ListDictionaryInternal_DictionaryNode => "System.Collections"
    ."ListDictionaryInternal/DictionaryNode"
);
#[cfg(feature = "System+Collections+ListDictionaryInternal+DictionaryNode")]
impl std::ops::Deref
for crate::System::Collections::ListDictionaryInternal_DictionaryNode {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+ListDictionaryInternal+DictionaryNode")]
impl std::ops::DerefMut
for crate::System::Collections::ListDictionaryInternal_DictionaryNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+ListDictionaryInternal+DictionaryNode")]
impl crate::System::Collections::ListDictionaryInternal_DictionaryNode {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Collections+ListDictionaryInternal+DictionaryNode")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Collections::ListDictionaryInternal_DictionaryNode {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Collections+ListDictionaryInternal+NodeEnumerator")]
#[repr(C)]
#[derive(Debug)]
pub struct ListDictionaryInternal_NodeEnumerator {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub list: quest_hook::libil2cpp::Gc<
        crate::System::Collections::ListDictionaryInternal,
    >,
    pub current: quest_hook::libil2cpp::Gc<
        crate::System::Collections::ListDictionaryInternal_DictionaryNode,
    >,
    pub version: i32,
    pub start: bool,
}
#[cfg(feature = "System+Collections+ListDictionaryInternal+NodeEnumerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Collections::ListDictionaryInternal_NodeEnumerator => "System.Collections"
    ."ListDictionaryInternal/NodeEnumerator"
);
#[cfg(feature = "System+Collections+ListDictionaryInternal+NodeEnumerator")]
impl std::ops::Deref
for crate::System::Collections::ListDictionaryInternal_NodeEnumerator {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+ListDictionaryInternal+NodeEnumerator")]
impl std::ops::DerefMut
for crate::System::Collections::ListDictionaryInternal_NodeEnumerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+ListDictionaryInternal+NodeEnumerator")]
impl crate::System::Collections::ListDictionaryInternal_NodeEnumerator {
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveNext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        list: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ListDictionaryInternal,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (list))?;
        Ok(__cordl_object.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        list: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ListDictionaryInternal,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (list))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_Current", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Entry(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Collections::DictionaryEntry> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Collections::DictionaryEntry = __cordl_object
            .invoke("get_Entry", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Key(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_Key", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_Value", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Collections+ListDictionaryInternal+NodeEnumerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Collections::ListDictionaryInternal_NodeEnumerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Collections+ListDictionaryInternal+NodeEnumerator")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionaryEnumerator>>
for crate::System::Collections::ListDictionaryInternal_NodeEnumerator {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionaryEnumerator> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+ListDictionaryInternal+NodeEnumerator")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionaryEnumerator>>
for crate::System::Collections::ListDictionaryInternal_NodeEnumerator {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::System::Collections::IDictionaryEnumerator,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+ListDictionaryInternal+NodeEnumerator")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>>
for crate::System::Collections::ListDictionaryInternal_NodeEnumerator {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+ListDictionaryInternal+NodeEnumerator")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>>
for crate::System::Collections::ListDictionaryInternal_NodeEnumerator {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+ListDictionaryInternal+NodeKeyValueCollection")]
#[repr(C)]
#[derive(Debug)]
pub struct ListDictionaryInternal_NodeKeyValueCollection {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub list: quest_hook::libil2cpp::Gc<
        crate::System::Collections::ListDictionaryInternal,
    >,
    pub isKeys: bool,
}
#[cfg(feature = "System+Collections+ListDictionaryInternal+NodeKeyValueCollection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Collections::ListDictionaryInternal_NodeKeyValueCollection =>
    "System.Collections"."ListDictionaryInternal/NodeKeyValueCollection"
);
#[cfg(feature = "System+Collections+ListDictionaryInternal+NodeKeyValueCollection")]
impl std::ops::Deref
for crate::System::Collections::ListDictionaryInternal_NodeKeyValueCollection {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+ListDictionaryInternal+NodeKeyValueCollection")]
impl std::ops::DerefMut
for crate::System::Collections::ListDictionaryInternal_NodeKeyValueCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+ListDictionaryInternal+NodeKeyValueCollection")]
impl crate::System::Collections::ListDictionaryInternal_NodeKeyValueCollection {
    #[cfg(
        feature = "System+Collections+ListDictionaryInternal+NodeKeyValueCollection+NodeKeyValueEnumerator"
    )]
    pub type NodeKeyValueEnumerator = crate::System::Collections::NodeKeyValueCollection_ListDictionaryInternal_NodeKeyValueEnumerator;
    pub fn New(
        list: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ListDictionaryInternal,
        >,
        isKeys: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (list, isKeys))?;
        Ok(__cordl_object.into())
    }
    pub fn System_Collections_ICollection_CopyTo(
        &mut self,
        array: quest_hook::libil2cpp::Gc<crate::System::Array>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Collections.ICollection.CopyTo", (array, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_ICollection_get_Count(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("System.Collections.ICollection.get_Count", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_ICollection_get_IsSynchronized(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.Collections.ICollection.get_IsSynchronized", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_ICollection_get_SyncRoot(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("System.Collections.ICollection.get_SyncRoot", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("System.Collections.IEnumerable.GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        list: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ListDictionaryInternal,
        >,
        isKeys: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (list, isKeys))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Collections+ListDictionaryInternal+NodeKeyValueCollection")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Collections::ListDictionaryInternal_NodeKeyValueCollection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Collections+ListDictionaryInternal+NodeKeyValueCollection")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection>>
for crate::System::Collections::ListDictionaryInternal_NodeKeyValueCollection {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+ListDictionaryInternal+NodeKeyValueCollection")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection>>
for crate::System::Collections::ListDictionaryInternal_NodeKeyValueCollection {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+ListDictionaryInternal+NodeKeyValueCollection")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>>
for crate::System::Collections::ListDictionaryInternal_NodeKeyValueCollection {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+ListDictionaryInternal+NodeKeyValueCollection")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>>
for crate::System::Collections::ListDictionaryInternal_NodeKeyValueCollection {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "System+Collections+ListDictionaryInternal+NodeKeyValueCollection+NodeKeyValueEnumerator"
)]
#[repr(C)]
#[derive(Debug)]
pub struct NodeKeyValueCollection_ListDictionaryInternal_NodeKeyValueEnumerator {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub list: quest_hook::libil2cpp::Gc<
        crate::System::Collections::ListDictionaryInternal,
    >,
    pub current: quest_hook::libil2cpp::Gc<
        crate::System::Collections::ListDictionaryInternal_DictionaryNode,
    >,
    pub version: i32,
    pub isKeys: bool,
    pub start: bool,
}
#[cfg(
    feature = "System+Collections+ListDictionaryInternal+NodeKeyValueCollection+NodeKeyValueEnumerator"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Collections::NodeKeyValueCollection_ListDictionaryInternal_NodeKeyValueEnumerator
    => "System.Collections"
    ."ListDictionaryInternal/NodeKeyValueCollection/NodeKeyValueEnumerator"
);
#[cfg(
    feature = "System+Collections+ListDictionaryInternal+NodeKeyValueCollection+NodeKeyValueEnumerator"
)]
impl std::ops::Deref
for crate::System::Collections::NodeKeyValueCollection_ListDictionaryInternal_NodeKeyValueEnumerator {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Collections+ListDictionaryInternal+NodeKeyValueCollection+NodeKeyValueEnumerator"
)]
impl std::ops::DerefMut
for crate::System::Collections::NodeKeyValueCollection_ListDictionaryInternal_NodeKeyValueEnumerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Collections+ListDictionaryInternal+NodeKeyValueCollection+NodeKeyValueEnumerator"
)]
impl crate::System::Collections::NodeKeyValueCollection_ListDictionaryInternal_NodeKeyValueEnumerator {
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveNext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        list: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ListDictionaryInternal,
        >,
        isKeys: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (list, isKeys))?;
        Ok(__cordl_object.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        list: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ListDictionaryInternal,
        >,
        isKeys: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (list, isKeys))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_Current", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "System+Collections+ListDictionaryInternal+NodeKeyValueCollection+NodeKeyValueEnumerator"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Collections::NodeKeyValueCollection_ListDictionaryInternal_NodeKeyValueEnumerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "System+Collections+ListDictionaryInternal+NodeKeyValueCollection+NodeKeyValueEnumerator"
)]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>>
for crate::System::Collections::NodeKeyValueCollection_ListDictionaryInternal_NodeKeyValueEnumerator {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "System+Collections+ListDictionaryInternal+NodeKeyValueCollection+NodeKeyValueEnumerator"
)]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>>
for crate::System::Collections::NodeKeyValueCollection_ListDictionaryInternal_NodeKeyValueEnumerator {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator> {
        unsafe { std::mem::transmute(self) }
    }
}
