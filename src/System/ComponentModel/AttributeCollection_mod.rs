#[cfg(feature = "System+ComponentModel+AttributeCollection")]
#[repr(C)]
#[derive(Debug)]
pub struct AttributeCollection {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _attributes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Attribute>,
    >,
    pub _foundAttributeTypes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::System::ComponentModel::AttributeCollection_AttributeEntry,
        >,
    >,
    pub _index: i32,
}
#[cfg(feature = "System+ComponentModel+AttributeCollection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::ComponentModel::AttributeCollection =>
    "System.ComponentModel"."AttributeCollection"
);
#[cfg(feature = "System+ComponentModel+AttributeCollection")]
impl std::ops::Deref for crate::System::ComponentModel::AttributeCollection {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+AttributeCollection")]
impl std::ops::DerefMut for crate::System::ComponentModel::AttributeCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+AttributeCollection")]
impl crate::System::ComponentModel::AttributeCollection {
    #[cfg(feature = "System+ComponentModel+AttributeCollection+AttributeEntry")]
    pub type AttributeEntry = crate::System::ComponentModel::AttributeCollection_AttributeEntry;
    pub fn Contains(
        &mut self,
        attribute: quest_hook::libil2cpp::Gc<crate::System::Attribute>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Contains", (attribute))?;
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
    pub fn GetDefaultAttribute(
        &mut self,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Attribute>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Attribute> = __cordl_object
            .invoke("GetDefaultAttribute", (attributeType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        attributes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Attribute>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (attributes))?;
        Ok(__cordl_object.into())
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
        attributes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Attribute>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (attributes))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Attributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Attribute>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Attribute>,
        > = __cordl_object.invoke("get_Attributes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(
        &mut self,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Attribute>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Attribute> = __cordl_object
            .invoke("get_Item", (attributeType))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+ComponentModel+AttributeCollection")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::AttributeCollection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+ComponentModel+AttributeCollection")]
impl AsRef<crate::System::Collections::ICollection>
for crate::System::ComponentModel::AttributeCollection {
    fn as_ref(&self) -> &crate::System::Collections::ICollection {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+ComponentModel+AttributeCollection")]
impl AsMut<crate::System::Collections::ICollection>
for crate::System::ComponentModel::AttributeCollection {
    fn as_mut(&mut self) -> &mut crate::System::Collections::ICollection {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+ComponentModel+AttributeCollection")]
impl AsRef<crate::System::Collections::IEnumerable>
for crate::System::ComponentModel::AttributeCollection {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+ComponentModel+AttributeCollection")]
impl AsMut<crate::System::Collections::IEnumerable>
for crate::System::ComponentModel::AttributeCollection {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+ComponentModel+AttributeCollection+AttributeEntry")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct AttributeCollection_AttributeEntry {
    pub _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub index: i32,
}
#[cfg(feature = "System+ComponentModel+AttributeCollection+AttributeEntry")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::AttributeCollection_AttributeEntry =>
    "System.ComponentModel"."AttributeCollection/AttributeEntry"
);
#[cfg(feature = "System+ComponentModel+AttributeCollection+AttributeEntry")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::ComponentModel::AttributeCollection_AttributeEntry {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+ComponentModel+AttributeCollection+AttributeEntry")]
impl crate::System::ComponentModel::AttributeCollection_AttributeEntry {}
