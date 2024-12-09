#[cfg(feature = "System+ComponentModel+ArrayConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct ArrayConverter {
    __cordl_parent: crate::System::ComponentModel::CollectionConverter,
}
#[cfg(feature = "System+ComponentModel+ArrayConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::ComponentModel::ArrayConverter =>
    "System.ComponentModel"."ArrayConverter"
);
#[cfg(feature = "System+ComponentModel+ArrayConverter")]
impl std::ops::Deref for crate::System::ComponentModel::ArrayConverter {
    type Target = crate::System::ComponentModel::CollectionConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+ArrayConverter")]
impl std::ops::DerefMut for crate::System::ComponentModel::ArrayConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+ArrayConverter")]
impl crate::System::ComponentModel::ArrayConverter {
    #[cfg(feature = "System+ComponentModel+ArrayConverter+ArrayPropertyDescriptor")]
    pub type ArrayPropertyDescriptor = crate::System::ComponentModel::ArrayConverter_ArrayPropertyDescriptor;
    pub fn ConvertTo(
        &mut self,
        context: *mut crate::System::ComponentModel::ITypeDescriptorContext,
        culture: *mut crate::System::Globalization::CultureInfo,
        value: *mut quest_hook::libil2cpp::Il2CppObject,
        destinationType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("ConvertTo", (context, culture, value, destinationType))?;
        Ok(__cordl_ret)
    }
    pub fn GetProperties(
        &mut self,
        context: *mut crate::System::ComponentModel::ITypeDescriptorContext,
        value: *mut quest_hook::libil2cpp::Il2CppObject,
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
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+ComponentModel+ArrayConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::ArrayConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+ComponentModel+ArrayConverter+ArrayPropertyDescriptor")]
#[repr(C)]
#[derive(Debug)]
pub struct ArrayConverter_ArrayPropertyDescriptor {
    __cordl_parent: crate::System::ComponentModel::TypeConverter_SimplePropertyDescriptor,
    pub _index: i32,
}
#[cfg(feature = "System+ComponentModel+ArrayConverter+ArrayPropertyDescriptor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::ArrayConverter_ArrayPropertyDescriptor =>
    "System.ComponentModel"."ArrayConverter/ArrayPropertyDescriptor"
);
#[cfg(feature = "System+ComponentModel+ArrayConverter+ArrayPropertyDescriptor")]
impl std::ops::Deref
for crate::System::ComponentModel::ArrayConverter_ArrayPropertyDescriptor {
    type Target = crate::System::ComponentModel::TypeConverter_SimplePropertyDescriptor;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+ArrayConverter+ArrayPropertyDescriptor")]
impl std::ops::DerefMut
for crate::System::ComponentModel::ArrayConverter_ArrayPropertyDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+ArrayConverter+ArrayPropertyDescriptor")]
impl crate::System::ComponentModel::ArrayConverter_ArrayPropertyDescriptor {
    pub fn GetValue(
        &mut self,
        instance: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("GetValue", (instance))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        arrayType: *mut crate::System::Type,
        elementType: *mut crate::System::Type,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (arrayType, elementType, index))?;
        Ok(__cordl_object)
    }
    pub fn SetValue(
        &mut self,
        instance: *mut quest_hook::libil2cpp::Il2CppObject,
        value: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetValue", (instance, value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        arrayType: *mut crate::System::Type,
        elementType: *mut crate::System::Type,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (arrayType, elementType, index))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+ComponentModel+ArrayConverter+ArrayPropertyDescriptor")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::ArrayConverter_ArrayPropertyDescriptor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
