#[cfg(feature = "UnityEngine+UIElements+SortColumnDescriptions")]
#[repr(C)]
#[derive(Debug)]
pub struct SortColumnDescriptions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Descriptions: *mut crate::System::Collections::Generic::IList_1<
        *mut crate::UnityEngine::UIElements::SortColumnDescription,
    >,
    pub changed: *mut crate::System::Action,
}
#[cfg(feature = "UnityEngine+UIElements+SortColumnDescriptions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::SortColumnDescriptions
    => "UnityEngine.UIElements"."SortColumnDescriptions"
);
#[cfg(feature = "UnityEngine+UIElements+SortColumnDescriptions")]
impl std::ops::Deref for crate::UnityEngine::UIElements::SortColumnDescriptions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+SortColumnDescriptions")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::SortColumnDescriptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+SortColumnDescriptions")]
impl crate::UnityEngine::UIElements::SortColumnDescriptions {
    #[cfg(feature = "UnityEngine+UIElements+SortColumnDescriptions+UxmlObjectFactory_1")]
    pub type UxmlObjectFactory_1<T: quest_hook::libil2cpp::Type> = crate::UnityEngine::UIElements::SortColumnDescriptions_UxmlObjectFactory_1<
        T,
    >;
    #[cfg(feature = "UnityEngine+UIElements+SortColumnDescriptions+UxmlObjectTraits_1")]
    pub type UxmlObjectTraits_1<T: quest_hook::libil2cpp::Type> = crate::UnityEngine::UIElements::SortColumnDescriptions_UxmlObjectTraits_1<
        T,
    >;
    pub fn Add(
        &mut self,
        item: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::SortColumnDescription,
        >,
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
        item: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::SortColumnDescription,
        >,
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
                *mut crate::UnityEngine::UIElements::SortColumnDescription,
            >,
        >,
        arrayIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyTo", (array, arrayIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                *mut crate::UnityEngine::UIElements::SortColumnDescription,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                *mut crate::UnityEngine::UIElements::SortColumnDescription,
            >,
        > = __cordl_object.invoke("GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Insert(
        &mut self,
        index: i32,
        desc: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::SortColumnDescription,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Insert", (index, desc))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDescriptionChanged(
        &mut self,
        desc: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::SortColumnDescription,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDescriptionChanged", (desc))?;
        Ok(__cordl_ret.into())
    }
    pub fn Remove(
        &mut self,
        desc: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::SortColumnDescription,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Remove", (desc))?;
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
    pub fn add_changed(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_changed", (value))?;
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
    pub fn remove_changed(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_changed", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+SortColumnDescriptions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::SortColumnDescriptions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+SortColumnDescriptions")]
impl AsRef<
    crate::System::Collections::Generic::ICollection_1<
        *mut crate::UnityEngine::UIElements::SortColumnDescription,
    >,
> for crate::UnityEngine::UIElements::SortColumnDescriptions {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::ICollection_1<
        *mut crate::UnityEngine::UIElements::SortColumnDescription,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+SortColumnDescriptions")]
impl AsMut<
    crate::System::Collections::Generic::ICollection_1<
        *mut crate::UnityEngine::UIElements::SortColumnDescription,
    >,
> for crate::UnityEngine::UIElements::SortColumnDescriptions {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::ICollection_1<
        *mut crate::UnityEngine::UIElements::SortColumnDescription,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+SortColumnDescriptions")]
impl AsRef<
    crate::System::Collections::Generic::IEnumerable_1<
        *mut crate::UnityEngine::UIElements::SortColumnDescription,
    >,
> for crate::UnityEngine::UIElements::SortColumnDescriptions {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEnumerable_1<
        *mut crate::UnityEngine::UIElements::SortColumnDescription,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+SortColumnDescriptions")]
impl AsMut<
    crate::System::Collections::Generic::IEnumerable_1<
        *mut crate::UnityEngine::UIElements::SortColumnDescription,
    >,
> for crate::UnityEngine::UIElements::SortColumnDescriptions {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerable_1<
        *mut crate::UnityEngine::UIElements::SortColumnDescription,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+SortColumnDescriptions")]
impl AsRef<crate::System::Collections::IEnumerable>
for crate::UnityEngine::UIElements::SortColumnDescriptions {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+SortColumnDescriptions")]
impl AsMut<crate::System::Collections::IEnumerable>
for crate::UnityEngine::UIElements::SortColumnDescriptions {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+SortColumnDescriptions+UxmlObjectFactory_1")]
#[repr(C)]
#[derive(Debug)]
pub struct SortColumnDescriptions_UxmlObjectFactory_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::UnityEngine::UIElements::UxmlObjectFactory_2<
        T,
        *mut crate::UnityEngine::UIElements::SortColumnDescriptions_UxmlObjectTraits_1<T>,
    >,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "UnityEngine+UIElements+SortColumnDescriptions+UxmlObjectFactory_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::SortColumnDescriptions_UxmlObjectFactory_1 < T > =>
    "UnityEngine.UIElements"."SortColumnDescriptions/UxmlObjectFactory`1" < T >
);
#[cfg(feature = "UnityEngine+UIElements+SortColumnDescriptions+UxmlObjectFactory_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::UIElements::SortColumnDescriptions_UxmlObjectFactory_1<T> {
    type Target = crate::UnityEngine::UIElements::UxmlObjectFactory_2<
        T,
        *mut crate::UnityEngine::UIElements::SortColumnDescriptions_UxmlObjectTraits_1<T>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+SortColumnDescriptions+UxmlObjectFactory_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::UnityEngine::UIElements::SortColumnDescriptions_UxmlObjectFactory_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+SortColumnDescriptions+UxmlObjectFactory_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::UnityEngine::UIElements::SortColumnDescriptions_UxmlObjectFactory_1<T> {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+SortColumnDescriptions+UxmlObjectFactory_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::SortColumnDescriptions_UxmlObjectFactory_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+SortColumnDescriptions+UxmlObjectTraits_1")]
#[repr(C)]
#[derive(Debug)]
pub struct SortColumnDescriptions_UxmlObjectTraits_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::UnityEngine::UIElements::UxmlObjectTraits_1<T>,
    pub m_SortColumnDescriptions: *mut crate::UnityEngine::UIElements::UxmlObjectListAttributeDescription_1<
        *mut crate::UnityEngine::UIElements::SortColumnDescription,
    >,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "UnityEngine+UIElements+SortColumnDescriptions+UxmlObjectTraits_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::SortColumnDescriptions_UxmlObjectTraits_1 < T > =>
    "UnityEngine.UIElements"."SortColumnDescriptions/UxmlObjectTraits`1" < T >
);
#[cfg(feature = "UnityEngine+UIElements+SortColumnDescriptions+UxmlObjectTraits_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::UIElements::SortColumnDescriptions_UxmlObjectTraits_1<T> {
    type Target = crate::UnityEngine::UIElements::UxmlObjectTraits_1<T>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+SortColumnDescriptions+UxmlObjectTraits_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::UnityEngine::UIElements::SortColumnDescriptions_UxmlObjectTraits_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+SortColumnDescriptions+UxmlObjectTraits_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::UnityEngine::UIElements::SortColumnDescriptions_UxmlObjectTraits_1<T> {
    pub fn Init(
        &mut self,
        obj: quest_hook::libil2cpp::ByRefMut<T>,
        bag: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IUxmlAttributes>,
        cc: crate::UnityEngine::UIElements::CreationContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (obj, bag, cc))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+SortColumnDescriptions+UxmlObjectTraits_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::SortColumnDescriptions_UxmlObjectTraits_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
