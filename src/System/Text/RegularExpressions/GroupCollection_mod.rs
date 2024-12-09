#[cfg(feature = "System+Text+RegularExpressions+GroupCollection")]
#[repr(C)]
#[derive(Debug)]
pub struct GroupCollection {
    __cordl_parent: crate::System::Object,
    pub _match: *mut crate::System::Text::RegularExpressions::Match,
    pub _captureMap: *mut crate::System::Collections::Hashtable,
    pub _groups: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Text::RegularExpressions::Group,
    >,
}
#[cfg(feature = "System+Text+RegularExpressions+GroupCollection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Text::RegularExpressions::GroupCollection =>
    "System.Text.RegularExpressions"."GroupCollection"
);
#[cfg(feature = "System+Text+RegularExpressions+GroupCollection")]
impl std::ops::Deref for crate::System::Text::RegularExpressions::GroupCollection {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+GroupCollection")]
impl std::ops::DerefMut for crate::System::Text::RegularExpressions::GroupCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+GroupCollection")]
impl crate::System::Text::RegularExpressions::GroupCollection {
    #[cfg(feature = "System+Text+RegularExpressions+GroupCollection+Enumerator")]
    pub type Enumerator = crate::System::Text::RegularExpressions::GroupCollection_Enumerator;
    pub fn CopyTo_Array0(
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
    pub fn CopyTo_Il2CppArray1(
        &mut self,
        array: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Text::RegularExpressions::Group,
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
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("GetEnumerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetGroup(
        &mut self,
        groupnum: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Text::RegularExpressions::Group,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::RegularExpressions::Group = __cordl_object
            .invoke("GetGroup", (groupnum))?;
        Ok(__cordl_ret)
    }
    pub fn GetGroupImpl(
        &mut self,
        groupnum: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Text::RegularExpressions::Group,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::RegularExpressions::Group = __cordl_object
            .invoke("GetGroupImpl", (groupnum))?;
        Ok(__cordl_ret)
    }
    pub fn New_1() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_Match_Hashtable0(
        _cordl_match: *mut crate::System::Text::RegularExpressions::Match,
        caps: *mut crate::System::Collections::Hashtable,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_match, caps))?;
        Ok(__cordl_object)
    }
    pub fn System_Collections_Generic_ICollection_System_Text_RegularExpressions_Group__Add(
        &mut self,
        item: *mut crate::System::Text::RegularExpressions::Group,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Collections.Generic.ICollection<System.Text.RegularExpressions.Group>.Add",
                (item),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_Generic_ICollection_System_Text_RegularExpressions_Group__Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Collections.Generic.ICollection<System.Text.RegularExpressions.Group>.Clear",
                (),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_Generic_ICollection_System_Text_RegularExpressions_Group__Contains(
        &mut self,
        item: *mut crate::System::Text::RegularExpressions::Group,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "System.Collections.Generic.ICollection<System.Text.RegularExpressions.Group>.Contains",
                (item),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_Generic_ICollection_System_Text_RegularExpressions_Group__Remove(
        &mut self,
        item: *mut crate::System::Text::RegularExpressions::Group,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "System.Collections.Generic.ICollection<System.Text.RegularExpressions.Group>.Remove",
                (item),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_Generic_IEnumerable_System_Text_RegularExpressions_Group__GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerator_1<
            *mut crate::System::Text::RegularExpressions::Group,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerator_1<
            *mut crate::System::Text::RegularExpressions::Group,
        > = __cordl_object
            .invoke(
                "System.Collections.Generic.IEnumerable<System.Text.RegularExpressions.Group>.GetEnumerator",
                (),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_Generic_IList_System_Text_RegularExpressions_Group__IndexOf(
        &mut self,
        item: *mut crate::System::Text::RegularExpressions::Group,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "System.Collections.Generic.IList<System.Text.RegularExpressions.Group>.IndexOf",
                (item),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_Generic_IList_System_Text_RegularExpressions_Group__Insert(
        &mut self,
        index: i32,
        item: *mut crate::System::Text::RegularExpressions::Group,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Collections.Generic.IList<System.Text.RegularExpressions.Group>.Insert",
                (index, item),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_Generic_IList_System_Text_RegularExpressions_Group__RemoveAt(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Collections.Generic.IList<System.Text.RegularExpressions.Group>.RemoveAt",
                (index),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_Generic_IList_System_Text_RegularExpressions_Group__get_Item(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Text::RegularExpressions::Group,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::RegularExpressions::Group = __cordl_object
            .invoke(
                "System.Collections.Generic.IList<System.Text.RegularExpressions.Group>.get_Item",
                (index),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_Generic_IList_System_Text_RegularExpressions_Group__set_Item(
        &mut self,
        index: i32,
        value: *mut crate::System::Text::RegularExpressions::Group,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Collections.Generic.IList<System.Text.RegularExpressions.Group>.set_Item",
                (index, value),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IList_Add(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("System.Collections.IList.Add", (value))?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IList_Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Collections.IList.Clear", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IList_Contains(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.Collections.IList.Contains", (value))?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IList_IndexOf(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("System.Collections.IList.IndexOf", (value))?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IList_Insert(
        &mut self,
        index: i32,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Collections.IList.Insert", (index, value))?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IList_Remove(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Collections.IList.Remove", (value))?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IList_RemoveAt(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Collections.IList.RemoveAt", (index))?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IList_get_IsFixedSize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.Collections.IList.get_IsFixedSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IList_get_Item(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("System.Collections.IList.get_Item", (index))?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IList_set_Item(
        &mut self,
        index: i32,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Collections.IList.set_Item", (index, value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Match_Hashtable0(
        &mut self,
        _cordl_match: *mut crate::System::Text::RegularExpressions::Match,
        caps: *mut crate::System::Collections::Hashtable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_match, caps))?;
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
    pub fn get_IsSynchronized(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsSynchronized", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Item_String1(
        &mut self,
        groupname: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Text::RegularExpressions::Group,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::RegularExpressions::Group = __cordl_object
            .invoke("get_Item", (groupname))?;
        Ok(__cordl_ret)
    }
    pub fn get_Item_i32_0(
        &mut self,
        groupnum: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Text::RegularExpressions::Group,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::RegularExpressions::Group = __cordl_object
            .invoke("get_Item", (groupnum))?;
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
#[cfg(feature = "System+Text+RegularExpressions+GroupCollection")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Text::RegularExpressions::GroupCollection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Text+RegularExpressions+GroupCollection+Enumerator")]
#[repr(C)]
#[derive(Debug)]
pub struct GroupCollection_Enumerator {
    __cordl_parent: crate::System::Object,
    pub _collection: *mut crate::System::Text::RegularExpressions::GroupCollection,
    pub _index: i32,
}
#[cfg(feature = "System+Text+RegularExpressions+GroupCollection+Enumerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Text::RegularExpressions::GroupCollection_Enumerator =>
    "System.Text.RegularExpressions"."GroupCollection/Enumerator"
);
#[cfg(feature = "System+Text+RegularExpressions+GroupCollection+Enumerator")]
impl std::ops::Deref
for crate::System::Text::RegularExpressions::GroupCollection_Enumerator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+GroupCollection+Enumerator")]
impl std::ops::DerefMut
for crate::System::Text::RegularExpressions::GroupCollection_Enumerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+GroupCollection+Enumerator")]
impl crate::System::Text::RegularExpressions::GroupCollection_Enumerator {
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveNext", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        collection: *mut crate::System::Text::RegularExpressions::GroupCollection,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (collection))?;
        Ok(__cordl_object)
    }
    pub fn System_Collections_IEnumerator_Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Collections.IEnumerator.Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IEnumerator_get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("System.Collections.IEnumerator.get_Current", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_IDisposable_Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.IDisposable.Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        collection: *mut crate::System::Text::RegularExpressions::GroupCollection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (collection))?;
        Ok(__cordl_ret)
    }
    pub fn get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Text::RegularExpressions::Group,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::RegularExpressions::Group = __cordl_object
            .invoke("get_Current", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Text+RegularExpressions+GroupCollection+Enumerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Text::RegularExpressions::GroupCollection_Enumerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
