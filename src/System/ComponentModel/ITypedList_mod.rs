#[cfg(feature = "System+ComponentModel+ITypedList")]
#[repr(C)]
#[derive(Debug)]
pub struct ITypedList {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+ComponentModel+ITypedList")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::ComponentModel::ITypedList =>
    "System.ComponentModel"."ITypedList"
);
#[cfg(feature = "System+ComponentModel+ITypedList")]
impl std::ops::Deref for crate::System::ComponentModel::ITypedList {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+ITypedList")]
impl std::ops::DerefMut for crate::System::ComponentModel::ITypedList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+ITypedList")]
impl crate::System::ComponentModel::ITypedList {
    pub fn GetItemProperties(
        &mut self,
        listAccessors: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::ComponentModel::PropertyDescriptor,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::PropertyDescriptorCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::PropertyDescriptorCollection = __cordl_object
            .invoke("GetItemProperties", (listAccessors))?;
        Ok(__cordl_ret)
    }
    pub fn GetListName(
        &mut self,
        listAccessors: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::ComponentModel::PropertyDescriptor,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetListName", (listAccessors))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+ComponentModel+ITypedList")]
impl quest_hook::libil2cpp::ObjectType for crate::System::ComponentModel::ITypedList {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}