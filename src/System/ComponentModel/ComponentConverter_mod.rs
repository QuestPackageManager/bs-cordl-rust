#[cfg(feature = "System+ComponentModel+ComponentConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct ComponentConverter {
    __cordl_parent: crate::System::ComponentModel::ReferenceConverter,
}
#[cfg(feature = "System+ComponentModel+ComponentConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::ComponentModel::ComponentConverter =>
    "System.ComponentModel"."ComponentConverter"
);
#[cfg(feature = "System+ComponentModel+ComponentConverter")]
impl std::ops::Deref for crate::System::ComponentModel::ComponentConverter {
    type Target = crate::System::ComponentModel::ReferenceConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+ComponentConverter")]
impl std::ops::DerefMut for crate::System::ComponentModel::ComponentConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+ComponentConverter")]
impl crate::System::ComponentModel::ComponentConverter {
    pub fn GetPropertiesSupported(
        &mut self,
        context: *mut crate::System::ComponentModel::ITypeDescriptorContext,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetPropertiesSupported", (context))?;
        Ok(__cordl_ret)
    }
    pub fn GetProperties(
        &mut self,
        context: *mut crate::System::ComponentModel::ITypeDescriptorContext,
        value: *mut crate::System::Object,
        attributes: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Attribute,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::PropertyDescriptorCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::PropertyDescriptorCollection = __cordl_object
            .invoke("GetProperties", (context, value, attributes))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+ComponentModel+ComponentConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::ComponentConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
