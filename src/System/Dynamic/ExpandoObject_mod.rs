#[cfg(feature = "System+Dynamic+ExpandoObject")]
#[repr(C)]
#[derive(Debug)]
pub struct ExpandoObject {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub LockObject: *mut quest_hook::libil2cpp::Il2CppObject,
    pub _data: *mut crate::System::Dynamic::ExpandoObject_ExpandoData,
    pub _count: i32,
    pub _propertyChanged: *mut crate::System::ComponentModel::PropertyChangedEventHandler,
}
#[cfg(feature = "System+Dynamic+ExpandoObject")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Dynamic::ExpandoObject =>
    "System.Dynamic"."ExpandoObject"
);
#[cfg(feature = "System+Dynamic+ExpandoObject")]
impl std::ops::Deref for crate::System::Dynamic::ExpandoObject {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+ExpandoObject")]
impl std::ops::DerefMut for crate::System::Dynamic::ExpandoObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+ExpandoObject")]
impl crate::System::Dynamic::ExpandoObject {
    #[cfg(feature = "System+Dynamic+ExpandoObject+ExpandoData")]
    pub type ExpandoData = crate::System::Dynamic::ExpandoObject_ExpandoData;
    #[cfg(feature = "System+Dynamic+ExpandoObject+KeyCollection")]
    pub type KeyCollection = crate::System::Dynamic::ExpandoObject_KeyCollection;
    #[cfg(feature = "System+Dynamic+ExpandoObject+KeyCollectionDebugView")]
    pub type KeyCollectionDebugView = crate::System::Dynamic::ExpandoObject_KeyCollectionDebugView;
    #[cfg(feature = "System+Dynamic+ExpandoObject+MetaExpando")]
    pub type MetaExpando = crate::System::Dynamic::ExpandoObject_MetaExpando;
    #[cfg(feature = "System+Dynamic+ExpandoObject+ValueCollection")]
    pub type ValueCollection = crate::System::Dynamic::ExpandoObject_ValueCollection;
    #[cfg(feature = "System+Dynamic+ExpandoObject+ValueCollectionDebugView")]
    pub type ValueCollectionDebugView = crate::System::Dynamic::ExpandoObject_ValueCollectionDebugView;
    #[cfg(feature = "System+Dynamic+ExpandoObject+_GetExpandoEnumerator_d__51")]
    pub type _GetExpandoEnumerator_d__51 = crate::System::Dynamic::ExpandoObject__GetExpandoEnumerator_d__51;
    pub fn ExpandoContainsKey(
        &mut self,
        key: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ExpandoContainsKey", (key))?;
        Ok(__cordl_ret)
    }
    pub fn GetExpandoEnumerator(
        &mut self,
        data: *mut crate::System::Dynamic::ExpandoObject_ExpandoData,
        version: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerator_1<
            crate::System::Collections::Generic::KeyValuePair_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerator_1<
            crate::System::Collections::Generic::KeyValuePair_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        > = __cordl_object.invoke("GetExpandoEnumerator", (data, version))?;
        Ok(__cordl_ret)
    }
    pub fn IsDeletedMember(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsDeletedMember", (index))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn PromoteClass(
        &mut self,
        oldClass: *mut quest_hook::libil2cpp::Il2CppObject,
        newClass: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PromoteClass", (oldClass, newClass))?;
        Ok(__cordl_ret)
    }
    pub fn PromoteClassCore(
        &mut self,
        oldClass: *mut crate::System::Dynamic::ExpandoClass,
        newClass: *mut crate::System::Dynamic::ExpandoClass,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Dynamic::ExpandoObject_ExpandoData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Dynamic::ExpandoObject_ExpandoData = __cordl_object
            .invoke("PromoteClassCore", (oldClass, newClass))?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_Generic_ICollection_System_Collections_Generic_KeyValuePair_System_String_System_Object___Add(
        &mut self,
        item: crate::System::Collections::Generic::KeyValuePair_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Collections.Generic.ICollection<System.Collections.Generic.KeyValuePair<System.String,System.Object>>.Add",
                (item),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_Generic_ICollection_System_Collections_Generic_KeyValuePair_System_String_System_Object___Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Collections.Generic.ICollection<System.Collections.Generic.KeyValuePair<System.String,System.Object>>.Clear",
                (),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_Generic_ICollection_System_Collections_Generic_KeyValuePair_System_String_System_Object___Contains(
        &mut self,
        item: crate::System::Collections::Generic::KeyValuePair_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "System.Collections.Generic.ICollection<System.Collections.Generic.KeyValuePair<System.String,System.Object>>.Contains",
                (item),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_Generic_ICollection_System_Collections_Generic_KeyValuePair_System_String_System_Object___CopyTo(
        &mut self,
        array: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::System::Collections::Generic::KeyValuePair_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
        arrayIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Collections.Generic.ICollection<System.Collections.Generic.KeyValuePair<System.String,System.Object>>.CopyTo",
                (array, arrayIndex),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_Generic_ICollection_System_Collections_Generic_KeyValuePair_System_String_System_Object___Remove(
        &mut self,
        item: crate::System::Collections::Generic::KeyValuePair_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "System.Collections.Generic.ICollection<System.Collections.Generic.KeyValuePair<System.String,System.Object>>.Remove",
                (item),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_Generic_ICollection_System_Collections_Generic_KeyValuePair_System_String_System_Object___get_Count(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "System.Collections.Generic.ICollection<System.Collections.Generic.KeyValuePair<System.String,System.Object>>.get_Count",
                (),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_Generic_ICollection_System_Collections_Generic_KeyValuePair_System_String_System_Object___get_IsReadOnly(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "System.Collections.Generic.ICollection<System.Collections.Generic.KeyValuePair<System.String,System.Object>>.get_IsReadOnly",
                (),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_Generic_IDictionary_System_String_System_Object__Add(
        &mut self,
        key: *mut quest_hook::libil2cpp::Il2CppString,
        value: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Collections.Generic.IDictionary<System.String,System.Object>.Add",
                (key, value),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_Generic_IDictionary_System_String_System_Object__ContainsKey(
        &mut self,
        key: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "System.Collections.Generic.IDictionary<System.String,System.Object>.ContainsKey",
                (key),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_Generic_IDictionary_System_String_System_Object__Remove(
        &mut self,
        key: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "System.Collections.Generic.IDictionary<System.String,System.Object>.Remove",
                (key),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_Generic_IDictionary_System_String_System_Object__TryGetValue(
        &mut self,
        key: *mut quest_hook::libil2cpp::Il2CppString,
        value: quest_hook::libil2cpp::ByRefMut<*mut quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "System.Collections.Generic.IDictionary<System.String,System.Object>.TryGetValue",
                (key, value),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_Generic_IDictionary_System_String_System_Object__get_Item(
        &mut self,
        key: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke(
                "System.Collections.Generic.IDictionary<System.String,System.Object>.get_Item",
                (key),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_Generic_IDictionary_System_String_System_Object__get_Keys(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::ICollection_1<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::ICollection_1<
            *mut quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object
            .invoke(
                "System.Collections.Generic.IDictionary<System.String,System.Object>.get_Keys",
                (),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_Generic_IDictionary_System_String_System_Object__get_Values(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::ICollection_1<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::ICollection_1<
            *mut quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object
            .invoke(
                "System.Collections.Generic.IDictionary<System.String,System.Object>.get_Values",
                (),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_Generic_IDictionary_System_String_System_Object__set_Item(
        &mut self,
        key: *mut quest_hook::libil2cpp::Il2CppString,
        value: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Collections.Generic.IDictionary<System.String,System.Object>.set_Item",
                (key, value),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_Generic_IEnumerable_System_Collections_Generic_KeyValuePair_System_String_System_Object___GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerator_1<
            crate::System::Collections::Generic::KeyValuePair_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerator_1<
            crate::System::Collections::Generic::KeyValuePair_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        > = __cordl_object
            .invoke(
                "System.Collections.Generic.IEnumerable<System.Collections.Generic.KeyValuePair<System.String,System.Object>>.GetEnumerator",
                (),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("System.Collections.IEnumerable.GetEnumerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Dynamic_IDynamicMetaObjectProvider_GetMetaObject(
        &mut self,
        parameter: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Dynamic::DynamicMetaObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Dynamic::DynamicMetaObject = __cordl_object
            .invoke(
                "System.Dynamic.IDynamicMetaObjectProvider.GetMetaObject",
                (parameter),
            )?;
        Ok(__cordl_ret)
    }
    pub fn TryAddMember(
        &mut self,
        key: *mut quest_hook::libil2cpp::Il2CppString,
        value: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TryAddMember", (key, value))?;
        Ok(__cordl_ret)
    }
    pub fn TryDeleteValue(
        &mut self,
        indexClass: *mut quest_hook::libil2cpp::Il2CppObject,
        index: i32,
        name: *mut quest_hook::libil2cpp::Il2CppString,
        ignoreCase: bool,
        deleteValue: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "TryDeleteValue",
                (indexClass, index, name, ignoreCase, deleteValue),
            )?;
        Ok(__cordl_ret)
    }
    pub fn TryGetValue(
        &mut self,
        indexClass: *mut quest_hook::libil2cpp::Il2CppObject,
        index: i32,
        name: *mut quest_hook::libil2cpp::Il2CppString,
        ignoreCase: bool,
        value: quest_hook::libil2cpp::ByRefMut<*mut quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetValue", (indexClass, index, name, ignoreCase, value))?;
        Ok(__cordl_ret)
    }
    pub fn TryGetValueForKey(
        &mut self,
        key: *mut quest_hook::libil2cpp::Il2CppString,
        value: quest_hook::libil2cpp::ByRefMut<*mut quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetValueForKey", (key, value))?;
        Ok(__cordl_ret)
    }
    pub fn TrySetValue(
        &mut self,
        indexClass: *mut quest_hook::libil2cpp::Il2CppObject,
        index: i32,
        value: *mut quest_hook::libil2cpp::Il2CppObject,
        name: *mut quest_hook::libil2cpp::Il2CppString,
        ignoreCase: bool,
        add: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TrySetValue", (indexClass, index, value, name, ignoreCase, add))?;
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
    pub fn get_Class(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Dynamic::ExpandoClass> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Dynamic::ExpandoClass = __cordl_object
            .invoke("get_Class", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Dynamic+ExpandoObject")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Dynamic::ExpandoObject {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Dynamic+ExpandoObject+ExpandoData")]
#[repr(C)]
#[derive(Debug)]
pub struct ExpandoObject_ExpandoData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Class: *mut crate::System::Dynamic::ExpandoClass,
    pub _dataArray: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppObject,
    >,
    pub _version: i32,
}
#[cfg(feature = "System+Dynamic+ExpandoObject+ExpandoData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Dynamic::ExpandoObject_ExpandoData =>
    "System.Dynamic"."ExpandoObject/ExpandoData"
);
#[cfg(feature = "System+Dynamic+ExpandoObject+ExpandoData")]
impl std::ops::Deref for crate::System::Dynamic::ExpandoObject_ExpandoData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+ExpandoObject+ExpandoData")]
impl std::ops::DerefMut for crate::System::Dynamic::ExpandoObject_ExpandoData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+ExpandoObject+ExpandoData")]
impl crate::System::Dynamic::ExpandoObject_ExpandoData {
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_ExpandoClass_Il2CppArray_i32_1(
        klass: *mut crate::System::Dynamic::ExpandoClass,
        data: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
        version: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (klass, data, version))?;
        Ok(__cordl_object)
    }
    pub fn UpdateClass(
        &mut self,
        newClass: *mut crate::System::Dynamic::ExpandoClass,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Dynamic::ExpandoObject_ExpandoData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Dynamic::ExpandoObject_ExpandoData = __cordl_object
            .invoke("UpdateClass", (newClass))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ExpandoClass_Il2CppArray_i32_1(
        &mut self,
        klass: *mut crate::System::Dynamic::ExpandoClass,
        data: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
        version: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (klass, data, version))?;
        Ok(__cordl_ret)
    }
    pub fn get_Item(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("get_Item", (index))?;
        Ok(__cordl_ret)
    }
    pub fn get_Length(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Length", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Version(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Version", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Item(
        &mut self,
        index: i32,
        value: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Item", (index, value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Dynamic+ExpandoObject+ExpandoData")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Dynamic::ExpandoObject_ExpandoData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Dynamic+ExpandoObject+KeyCollection")]
#[repr(C)]
#[derive(Debug)]
pub struct ExpandoObject_KeyCollection {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _expando: *mut crate::System::Dynamic::ExpandoObject,
    pub _expandoVersion: i32,
    pub _expandoCount: i32,
    pub _expandoData: *mut crate::System::Dynamic::ExpandoObject_ExpandoData,
}
#[cfg(feature = "System+Dynamic+ExpandoObject+KeyCollection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Dynamic::ExpandoObject_KeyCollection =>
    "System.Dynamic"."ExpandoObject/KeyCollection"
);
#[cfg(feature = "System+Dynamic+ExpandoObject+KeyCollection")]
impl std::ops::Deref for crate::System::Dynamic::ExpandoObject_KeyCollection {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+ExpandoObject+KeyCollection")]
impl std::ops::DerefMut for crate::System::Dynamic::ExpandoObject_KeyCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+ExpandoObject+KeyCollection")]
impl crate::System::Dynamic::ExpandoObject_KeyCollection {
    #[cfg(feature = "System+Dynamic+ExpandoObject+KeyCollection+_GetEnumerator_d__15")]
    pub type _GetEnumerator_d__15 = crate::System::Dynamic::KeyCollection_ExpandoObject__GetEnumerator_d__15;
    pub fn Add(
        &mut self,
        item: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (item))?;
        Ok(__cordl_ret)
    }
    pub fn CheckVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckVersion", ())?;
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
        item: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Contains", (item))?;
        Ok(__cordl_ret)
    }
    pub fn CopyTo(
        &mut self,
        array: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
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
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerator_1<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerator_1<
            *mut quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetEnumerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        expando: *mut crate::System::Dynamic::ExpandoObject,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (expando))?;
        Ok(__cordl_object)
    }
    pub fn Remove(
        &mut self,
        item: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Remove", (item))?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("System.Collections.IEnumerable.GetEnumerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        expando: *mut crate::System::Dynamic::ExpandoObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (expando))?;
        Ok(__cordl_ret)
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsReadOnly(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsReadOnly", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Dynamic+ExpandoObject+KeyCollection")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Dynamic::ExpandoObject_KeyCollection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Dynamic+ExpandoObject+KeyCollectionDebugView")]
#[repr(C)]
#[derive(Debug)]
pub struct ExpandoObject_KeyCollectionDebugView {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Dynamic+ExpandoObject+KeyCollectionDebugView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Dynamic::ExpandoObject_KeyCollectionDebugView => "System.Dynamic"
    ."ExpandoObject/KeyCollectionDebugView"
);
#[cfg(feature = "System+Dynamic+ExpandoObject+KeyCollectionDebugView")]
impl std::ops::Deref for crate::System::Dynamic::ExpandoObject_KeyCollectionDebugView {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+ExpandoObject+KeyCollectionDebugView")]
impl std::ops::DerefMut
for crate::System::Dynamic::ExpandoObject_KeyCollectionDebugView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+ExpandoObject+KeyCollectionDebugView")]
impl crate::System::Dynamic::ExpandoObject_KeyCollectionDebugView {}
#[cfg(feature = "System+Dynamic+ExpandoObject+KeyCollectionDebugView")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Dynamic::ExpandoObject_KeyCollectionDebugView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Dynamic+ExpandoObject+MetaExpando")]
#[repr(C)]
#[derive(Debug)]
pub struct ExpandoObject_MetaExpando {
    __cordl_parent: crate::System::Dynamic::DynamicMetaObject,
}
#[cfg(feature = "System+Dynamic+ExpandoObject+MetaExpando")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Dynamic::ExpandoObject_MetaExpando =>
    "System.Dynamic"."ExpandoObject/MetaExpando"
);
#[cfg(feature = "System+Dynamic+ExpandoObject+MetaExpando")]
impl std::ops::Deref for crate::System::Dynamic::ExpandoObject_MetaExpando {
    type Target = crate::System::Dynamic::DynamicMetaObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+ExpandoObject+MetaExpando")]
impl std::ops::DerefMut for crate::System::Dynamic::ExpandoObject_MetaExpando {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+ExpandoObject+MetaExpando")]
impl crate::System::Dynamic::ExpandoObject_MetaExpando {
    #[cfg(
        feature = "System+Dynamic+ExpandoObject+MetaExpando+_GetDynamicMemberNames_d__6"
    )]
    pub type _GetDynamicMemberNames_d__6 = crate::System::Dynamic::MetaExpando_ExpandoObject__GetDynamicMemberNames_d__6;
    #[cfg(feature = "System+Dynamic+ExpandoObject+MetaExpando+__c__DisplayClass3_0")]
    pub type __c__DisplayClass3_0 = crate::System::Dynamic::MetaExpando_ExpandoObject___c__DisplayClass3_0;
    pub fn AddDynamicTestAndDefer(
        &mut self,
        binder: *mut crate::System::Dynamic::DynamicMetaObjectBinder,
        klass: *mut crate::System::Dynamic::ExpandoClass,
        originalClass: *mut crate::System::Dynamic::ExpandoClass,
        succeeds: *mut crate::System::Dynamic::DynamicMetaObject,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Dynamic::DynamicMetaObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Dynamic::DynamicMetaObject = __cordl_object
            .invoke("AddDynamicTestAndDefer", (binder, klass, originalClass, succeeds))?;
        Ok(__cordl_ret)
    }
    pub fn BindDeleteMember(
        &mut self,
        binder: *mut crate::System::Dynamic::DeleteMemberBinder,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Dynamic::DynamicMetaObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Dynamic::DynamicMetaObject = __cordl_object
            .invoke("BindDeleteMember", (binder))?;
        Ok(__cordl_ret)
    }
    pub fn BindGetMember(
        &mut self,
        binder: *mut crate::System::Dynamic::GetMemberBinder,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Dynamic::DynamicMetaObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Dynamic::DynamicMetaObject = __cordl_object
            .invoke("BindGetMember", (binder))?;
        Ok(__cordl_ret)
    }
    pub fn BindGetOrInvokeMember(
        &mut self,
        binder: *mut crate::System::Dynamic::DynamicMetaObjectBinder,
        name: *mut quest_hook::libil2cpp::Il2CppString,
        ignoreCase: bool,
        fallback: *mut crate::System::Dynamic::DynamicMetaObject,
        fallbackInvoke: *mut crate::System::Func_2<
            *mut crate::System::Dynamic::DynamicMetaObject,
            *mut crate::System::Dynamic::DynamicMetaObject,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Dynamic::DynamicMetaObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Dynamic::DynamicMetaObject = __cordl_object
            .invoke(
                "BindGetOrInvokeMember",
                (binder, name, ignoreCase, fallback, fallbackInvoke),
            )?;
        Ok(__cordl_ret)
    }
    pub fn BindInvokeMember(
        &mut self,
        binder: *mut crate::System::Dynamic::InvokeMemberBinder,
        args: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Dynamic::DynamicMetaObject,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Dynamic::DynamicMetaObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Dynamic::DynamicMetaObject = __cordl_object
            .invoke("BindInvokeMember", (binder, args))?;
        Ok(__cordl_ret)
    }
    pub fn BindSetMember(
        &mut self,
        binder: *mut crate::System::Dynamic::SetMemberBinder,
        value: *mut crate::System::Dynamic::DynamicMetaObject,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Dynamic::DynamicMetaObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Dynamic::DynamicMetaObject = __cordl_object
            .invoke("BindSetMember", (binder, value))?;
        Ok(__cordl_ret)
    }
    pub fn GetClassEnsureIndex(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
        caseInsensitive: bool,
        obj: *mut crate::System::Dynamic::ExpandoObject,
        klass: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Dynamic::ExpandoClass,
        >,
        index: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Dynamic::ExpandoClass> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Dynamic::ExpandoClass = __cordl_object
            .invoke("GetClassEnsureIndex", (name, caseInsensitive, obj, klass, index))?;
        Ok(__cordl_ret)
    }
    pub fn GetDynamicMemberNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetDynamicMemberNames", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetLimitedSelf(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("GetLimitedSelf", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetRestrictions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Dynamic::BindingRestrictions,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Dynamic::BindingRestrictions = __cordl_object
            .invoke("GetRestrictions", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        expression: *mut crate::System::Linq::Expressions::Expression,
        value: *mut crate::System::Dynamic::ExpandoObject,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (expression, value))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        expression: *mut crate::System::Linq::Expressions::Expression,
        value: *mut crate::System::Dynamic::ExpandoObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (expression, value))?;
        Ok(__cordl_ret)
    }
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Dynamic::ExpandoObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Dynamic::ExpandoObject = __cordl_object
            .invoke("get_Value", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Dynamic+ExpandoObject+MetaExpando")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Dynamic::ExpandoObject_MetaExpando {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Dynamic+ExpandoObject+ValueCollection")]
#[repr(C)]
#[derive(Debug)]
pub struct ExpandoObject_ValueCollection {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _expando: *mut crate::System::Dynamic::ExpandoObject,
    pub _expandoVersion: i32,
    pub _expandoCount: i32,
    pub _expandoData: *mut crate::System::Dynamic::ExpandoObject_ExpandoData,
}
#[cfg(feature = "System+Dynamic+ExpandoObject+ValueCollection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Dynamic::ExpandoObject_ValueCollection
    => "System.Dynamic"."ExpandoObject/ValueCollection"
);
#[cfg(feature = "System+Dynamic+ExpandoObject+ValueCollection")]
impl std::ops::Deref for crate::System::Dynamic::ExpandoObject_ValueCollection {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+ExpandoObject+ValueCollection")]
impl std::ops::DerefMut for crate::System::Dynamic::ExpandoObject_ValueCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+ExpandoObject+ValueCollection")]
impl crate::System::Dynamic::ExpandoObject_ValueCollection {
    #[cfg(feature = "System+Dynamic+ExpandoObject+ValueCollection+_GetEnumerator_d__15")]
    pub type _GetEnumerator_d__15 = crate::System::Dynamic::ValueCollection_ExpandoObject__GetEnumerator_d__15;
    pub fn Add(
        &mut self,
        item: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (item))?;
        Ok(__cordl_ret)
    }
    pub fn CheckVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckVersion", ())?;
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
        item: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Contains", (item))?;
        Ok(__cordl_ret)
    }
    pub fn CopyTo(
        &mut self,
        array: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
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
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerator_1<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerator_1<
            *mut quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetEnumerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        expando: *mut crate::System::Dynamic::ExpandoObject,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (expando))?;
        Ok(__cordl_object)
    }
    pub fn Remove(
        &mut self,
        item: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Remove", (item))?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("System.Collections.IEnumerable.GetEnumerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        expando: *mut crate::System::Dynamic::ExpandoObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (expando))?;
        Ok(__cordl_ret)
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsReadOnly(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsReadOnly", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Dynamic+ExpandoObject+ValueCollection")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Dynamic::ExpandoObject_ValueCollection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Dynamic+ExpandoObject+ValueCollectionDebugView")]
#[repr(C)]
#[derive(Debug)]
pub struct ExpandoObject_ValueCollectionDebugView {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Dynamic+ExpandoObject+ValueCollectionDebugView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Dynamic::ExpandoObject_ValueCollectionDebugView => "System.Dynamic"
    ."ExpandoObject/ValueCollectionDebugView"
);
#[cfg(feature = "System+Dynamic+ExpandoObject+ValueCollectionDebugView")]
impl std::ops::Deref for crate::System::Dynamic::ExpandoObject_ValueCollectionDebugView {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+ExpandoObject+ValueCollectionDebugView")]
impl std::ops::DerefMut
for crate::System::Dynamic::ExpandoObject_ValueCollectionDebugView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+ExpandoObject+ValueCollectionDebugView")]
impl crate::System::Dynamic::ExpandoObject_ValueCollectionDebugView {}
#[cfg(feature = "System+Dynamic+ExpandoObject+ValueCollectionDebugView")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Dynamic::ExpandoObject_ValueCollectionDebugView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
