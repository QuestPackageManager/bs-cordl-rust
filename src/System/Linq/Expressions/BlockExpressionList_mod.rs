#[cfg(feature = "System+Linq+Expressions+BlockExpressionList")]
#[repr(C)]
#[derive(Debug)]
pub struct BlockExpressionList {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _block: quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::BlockExpression,
    >,
    pub _arg0: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
}
#[cfg(feature = "System+Linq+Expressions+BlockExpressionList")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Expressions::BlockExpressionList
    => "System.Linq.Expressions"."BlockExpressionList"
);
#[cfg(feature = "System+Linq+Expressions+BlockExpressionList")]
impl std::ops::Deref for crate::System::Linq::Expressions::BlockExpressionList {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+BlockExpressionList")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::BlockExpressionList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+BlockExpressionList")]
impl crate::System::Linq::Expressions::BlockExpressionList {
    pub fn Add(
        &mut self,
        item: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (item))?;
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
        item: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Contains", (item))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyTo(
        &mut self,
        array: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Linq::Expressions::Expression,
            >,
        >,
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
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                *mut crate::System::Linq::Expressions::Expression,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                *mut crate::System::Linq::Expressions::Expression,
            >,
        > = __cordl_object.invoke("GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf(
        &mut self,
        item: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("IndexOf", (item))?;
        Ok(__cordl_ret.into())
    }
    pub fn Insert(
        &mut self,
        index: i32,
        item: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Insert", (index, item))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        provider: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BlockExpression,
        >,
        arg0: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (provider, arg0))?;
        Ok(__cordl_object.into())
    }
    pub fn Remove(
        &mut self,
        item: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Remove", (item))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveAt(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveAt", (index))?;
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
        provider: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BlockExpression,
        >,
        arg0: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (provider, arg0))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsReadOnly(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsReadOnly", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        > = __cordl_object.invoke("get_Item", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Item(
        &mut self,
        index: i32,
        value: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Item", (index, value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+BlockExpressionList")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::BlockExpressionList {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+BlockExpressionList")]
impl AsRef<
    crate::System::Collections::Generic::ICollection_1<
        *mut crate::System::Linq::Expressions::Expression,
    >,
> for crate::System::Linq::Expressions::BlockExpressionList {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::ICollection_1<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+Expressions+BlockExpressionList")]
impl AsMut<
    crate::System::Collections::Generic::ICollection_1<
        *mut crate::System::Linq::Expressions::Expression,
    >,
> for crate::System::Linq::Expressions::BlockExpressionList {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::ICollection_1<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+Expressions+BlockExpressionList")]
impl AsRef<
    crate::System::Collections::Generic::IEnumerable_1<
        *mut crate::System::Linq::Expressions::Expression,
    >,
> for crate::System::Linq::Expressions::BlockExpressionList {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEnumerable_1<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+Expressions+BlockExpressionList")]
impl AsMut<
    crate::System::Collections::Generic::IEnumerable_1<
        *mut crate::System::Linq::Expressions::Expression,
    >,
> for crate::System::Linq::Expressions::BlockExpressionList {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerable_1<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+Expressions+BlockExpressionList")]
impl AsRef<
    crate::System::Collections::Generic::IList_1<
        *mut crate::System::Linq::Expressions::Expression,
    >,
> for crate::System::Linq::Expressions::BlockExpressionList {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IList_1<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+Expressions+BlockExpressionList")]
impl AsMut<
    crate::System::Collections::Generic::IList_1<
        *mut crate::System::Linq::Expressions::Expression,
    >,
> for crate::System::Linq::Expressions::BlockExpressionList {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IList_1<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+Expressions+BlockExpressionList")]
impl AsRef<crate::System::Collections::IEnumerable>
for crate::System::Linq::Expressions::BlockExpressionList {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+Expressions+BlockExpressionList")]
impl AsMut<crate::System::Collections::IEnumerable>
for crate::System::Linq::Expressions::BlockExpressionList {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
