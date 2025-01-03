#[cfg(feature = "System+ComponentModel+AmbientValueAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct AmbientValueAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _Value_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppObject,
    >,
}
#[cfg(feature = "System+ComponentModel+AmbientValueAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::ComponentModel::AmbientValueAttribute =>
    "System.ComponentModel"."AmbientValueAttribute"
);
#[cfg(feature = "System+ComponentModel+AmbientValueAttribute")]
impl std::ops::Deref for crate::System::ComponentModel::AmbientValueAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+AmbientValueAttribute")]
impl std::ops::DerefMut for crate::System::ComponentModel::AmbientValueAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+AmbientValueAttribute")]
impl crate::System::ComponentModel::AmbientValueAttribute {
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
#[cfg(feature = "System+ComponentModel+AmbientValueAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::AmbientValueAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
