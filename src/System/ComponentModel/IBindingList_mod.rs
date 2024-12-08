#[cfg(feature = "System+ComponentModel+IBindingList")]
#[repr(C)]
#[derive(Debug)]
pub struct IBindingList {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+ComponentModel+IBindingList")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::ComponentModel::IBindingList =>
    "System.ComponentModel"."IBindingList"
);
#[cfg(feature = "System+ComponentModel+IBindingList")]
impl std::ops::Deref for crate::System::ComponentModel::IBindingList {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+IBindingList")]
impl std::ops::DerefMut for crate::System::ComponentModel::IBindingList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+IBindingList")]
impl crate::System::ComponentModel::IBindingList {
    pub fn get_AllowEdit(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_AllowEdit", ())?;
        Ok(__cordl_ret)
    }
    pub fn Find(
        &mut self,
        property: *mut crate::System::ComponentModel::PropertyDescriptor,
        key: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Find", (property, key))?;
        Ok(__cordl_ret)
    }
    pub fn get_SupportsSearching(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_SupportsSearching", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SupportsSorting(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_SupportsSorting", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsSorted(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsSorted", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddNew(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("AddNew", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AllowRemove(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_AllowRemove", ())?;
        Ok(__cordl_ret)
    }
    pub fn RemoveSort(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveSort", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddIndex(
        &mut self,
        property: *mut crate::System::ComponentModel::PropertyDescriptor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddIndex", (property))?;
        Ok(__cordl_ret)
    }
    pub fn ApplySort(
        &mut self,
        property: *mut crate::System::ComponentModel::PropertyDescriptor,
        direction: crate::System::ComponentModel::ListSortDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplySort", (property, direction))?;
        Ok(__cordl_ret)
    }
    pub fn get_AllowNew(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_AllowNew", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SortProperty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::PropertyDescriptor,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::PropertyDescriptor = __cordl_object
            .invoke("get_SortProperty", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SortDirection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ComponentModel::ListSortDirection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::ComponentModel::ListSortDirection = __cordl_object
            .invoke("get_SortDirection", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SupportsChangeNotification(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_SupportsChangeNotification", ())?;
        Ok(__cordl_ret)
    }
    pub fn RemoveIndex(
        &mut self,
        property: *mut crate::System::ComponentModel::PropertyDescriptor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveIndex", (property))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+ComponentModel+IBindingList")]
impl quest_hook::libil2cpp::ObjectType for crate::System::ComponentModel::IBindingList {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
