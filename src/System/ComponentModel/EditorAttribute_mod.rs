#[cfg(feature = "System+ComponentModel+EditorAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct EditorAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _typeId: *mut crate::System::String,
    pub _EditorBaseTypeName_k__BackingField: *mut crate::System::String,
    pub _EditorTypeName_k__BackingField: *mut crate::System::String,
}
#[cfg(feature = "System+ComponentModel+EditorAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::ComponentModel::EditorAttribute =>
    "System.ComponentModel"."EditorAttribute"
);
#[cfg(feature = "System+ComponentModel+EditorAttribute")]
impl std::ops::Deref for crate::System::ComponentModel::EditorAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+EditorAttribute")]
impl std::ops::DerefMut for crate::System::ComponentModel::EditorAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+EditorAttribute")]
impl crate::System::ComponentModel::EditorAttribute {
    pub fn Equals(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        typeName: *mut crate::System::String,
        baseTypeName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (typeName, baseTypeName))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        typeName: *mut crate::System::String,
        baseTypeName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (typeName, baseTypeName))?;
        Ok(__cordl_ret)
    }
    pub fn get_EditorBaseTypeName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_EditorBaseTypeName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_EditorTypeName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_EditorTypeName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TypeId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_TypeId", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+ComponentModel+EditorAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::EditorAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
