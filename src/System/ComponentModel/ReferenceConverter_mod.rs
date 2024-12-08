#[cfg(feature = "System+ComponentModel+ReferenceConverter+ReferenceComparer")]
#[repr(C)]
#[derive(Debug)]
pub struct ReferenceConverter_ReferenceComparer {
    __cordl_parent: crate::System::Object,
    pub _converter: *mut crate::System::ComponentModel::ReferenceConverter,
}
#[cfg(feature = "System+ComponentModel+ReferenceConverter+ReferenceComparer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::ReferenceConverter_ReferenceComparer =>
    "System.ComponentModel"."ReferenceConverter/ReferenceComparer"
);
#[cfg(feature = "System+ComponentModel+ReferenceConverter+ReferenceComparer")]
impl std::ops::Deref
for crate::System::ComponentModel::ReferenceConverter_ReferenceComparer {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+ReferenceConverter+ReferenceComparer")]
impl std::ops::DerefMut
for crate::System::ComponentModel::ReferenceConverter_ReferenceComparer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+ReferenceConverter+ReferenceComparer")]
impl crate::System::ComponentModel::ReferenceConverter_ReferenceComparer {
    pub fn Compare(
        &mut self,
        item1: *mut crate::System::Object,
        item2: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Compare", (item1, item2))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        converter: *mut crate::System::ComponentModel::ReferenceConverter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (converter))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        converter: *mut crate::System::ComponentModel::ReferenceConverter,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (converter))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+ComponentModel+ReferenceConverter+ReferenceComparer")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::ReferenceConverter_ReferenceComparer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+ComponentModel+ReferenceConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct ReferenceConverter {
    __cordl_parent: crate::System::ComponentModel::TypeConverter,
    pub _type: *mut crate::System::Type,
}
#[cfg(feature = "System+ComponentModel+ReferenceConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::ComponentModel::ReferenceConverter =>
    "System.ComponentModel"."ReferenceConverter"
);
#[cfg(feature = "System+ComponentModel+ReferenceConverter")]
impl std::ops::Deref for crate::System::ComponentModel::ReferenceConverter {
    type Target = crate::System::ComponentModel::TypeConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+ReferenceConverter")]
impl std::ops::DerefMut for crate::System::ComponentModel::ReferenceConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+ReferenceConverter")]
impl crate::System::ComponentModel::ReferenceConverter {
    #[cfg(feature = "System+ComponentModel+ReferenceConverter+ReferenceComparer")]
    pub type ReferenceComparer = crate::System::ComponentModel::ReferenceConverter_ReferenceComparer;
    pub fn GetStandardValuesSupported(
        &mut self,
        context: *mut crate::System::ComponentModel::ITypeDescriptorContext,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetStandardValuesSupported", (context))?;
        Ok(__cordl_ret)
    }
    pub fn ConvertTo(
        &mut self,
        context: *mut crate::System::ComponentModel::ITypeDescriptorContext,
        culture: *mut crate::System::Globalization::CultureInfo,
        value: *mut crate::System::Object,
        destinationType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ConvertTo", (context, culture, value, destinationType))?;
        Ok(__cordl_ret)
    }
    pub fn CanConvertFrom(
        &mut self,
        context: *mut crate::System::ComponentModel::ITypeDescriptorContext,
        sourceType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CanConvertFrom", (context, sourceType))?;
        Ok(__cordl_ret)
    }
    pub fn GetStandardValues(
        &mut self,
        context: *mut crate::System::ComponentModel::ITypeDescriptorContext,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::TypeConverter_StandardValuesCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::TypeConverter_StandardValuesCollection = __cordl_object
            .invoke("GetStandardValues", (context))?;
        Ok(__cordl_ret)
    }
    pub fn IsValueAllowed(
        &mut self,
        context: *mut crate::System::ComponentModel::ITypeDescriptorContext,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsValueAllowed", (context, value))?;
        Ok(__cordl_ret)
    }
    pub fn GetStandardValuesExclusive(
        &mut self,
        context: *mut crate::System::ComponentModel::ITypeDescriptorContext,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetStandardValuesExclusive", (context))?;
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
    pub fn ConvertFrom(
        &mut self,
        context: *mut crate::System::ComponentModel::ITypeDescriptorContext,
        culture: *mut crate::System::Globalization::CultureInfo,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ConvertFrom", (context, culture, value))?;
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
#[cfg(feature = "System+ComponentModel+ReferenceConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::ReferenceConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
