#[cfg(feature = "System+ComponentModel+ExpandableObjectConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct ExpandableObjectConverter {
    __cordl_parent: crate::System::ComponentModel::TypeConverter,
}
#[cfg(feature = "System+ComponentModel+ExpandableObjectConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::ExpandableObjectConverter => "System.ComponentModel"
    ."ExpandableObjectConverter"
);
#[cfg(feature = "System+ComponentModel+ExpandableObjectConverter")]
impl std::ops::Deref for crate::System::ComponentModel::ExpandableObjectConverter {
    type Target = crate::System::ComponentModel::TypeConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+ExpandableObjectConverter")]
impl std::ops::DerefMut for crate::System::ComponentModel::ExpandableObjectConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+ExpandableObjectConverter")]
impl crate::System::ComponentModel::ExpandableObjectConverter {
    pub fn GetProperties(
        &mut self,
        context: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::ITypeDescriptorContext,
        >,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        attributes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Attribute>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptorCollection,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptorCollection,
        > = __cordl_object.invoke("GetProperties", (context, value, attributes))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertiesSupported(
        &mut self,
        context: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::ITypeDescriptorContext,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetPropertiesSupported", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
}
#[cfg(feature = "System+ComponentModel+ExpandableObjectConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::ExpandableObjectConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
