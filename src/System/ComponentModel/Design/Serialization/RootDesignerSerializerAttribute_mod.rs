#[cfg(
    feature = "System+ComponentModel+Design+Serialization+RootDesignerSerializerAttribute"
)]
#[repr(C)]
#[derive(Debug)]
pub struct RootDesignerSerializerAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _typeId: *mut quest_hook::libil2cpp::Il2CppString,
    pub _Reloadable_k__BackingField: bool,
    pub _SerializerTypeName_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
    pub _SerializerBaseTypeName_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(
    feature = "System+ComponentModel+Design+Serialization+RootDesignerSerializerAttribute"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::Design::Serialization::RootDesignerSerializerAttribute =>
    "System.ComponentModel.Design.Serialization"."RootDesignerSerializerAttribute"
);
#[cfg(
    feature = "System+ComponentModel+Design+Serialization+RootDesignerSerializerAttribute"
)]
impl std::ops::Deref
for crate::System::ComponentModel::Design::Serialization::RootDesignerSerializerAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+ComponentModel+Design+Serialization+RootDesignerSerializerAttribute"
)]
impl std::ops::DerefMut
for crate::System::ComponentModel::Design::Serialization::RootDesignerSerializerAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+ComponentModel+Design+Serialization+RootDesignerSerializerAttribute"
)]
impl crate::System::ComponentModel::Design::Serialization::RootDesignerSerializerAttribute {
    pub fn New(
        serializerTypeName: *mut quest_hook::libil2cpp::Il2CppString,
        baseSerializerTypeName: *mut quest_hook::libil2cpp::Il2CppString,
        reloadable: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (serializerTypeName, baseSerializerTypeName, reloadable),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        serializerTypeName: *mut quest_hook::libil2cpp::Il2CppString,
        baseSerializerTypeName: *mut quest_hook::libil2cpp::Il2CppString,
        reloadable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (serializerTypeName, baseSerializerTypeName, reloadable))?;
        Ok(__cordl_ret)
    }
    pub fn get_SerializerBaseTypeName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_SerializerBaseTypeName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TypeId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("get_TypeId", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "System+ComponentModel+Design+Serialization+RootDesignerSerializerAttribute"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::Design::Serialization::RootDesignerSerializerAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
