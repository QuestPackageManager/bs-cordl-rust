#[cfg(feature = "System+Data+PrimaryKeyTypeConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct PrimaryKeyTypeConverter {
    __cordl_parent: crate::System::ComponentModel::ReferenceConverter,
}
#[cfg(feature = "System+Data+PrimaryKeyTypeConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::PrimaryKeyTypeConverter =>
    "System.Data"."PrimaryKeyTypeConverter"
);
#[cfg(feature = "System+Data+PrimaryKeyTypeConverter")]
impl std::ops::Deref for crate::System::Data::PrimaryKeyTypeConverter {
    type Target = crate::System::ComponentModel::ReferenceConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+PrimaryKeyTypeConverter")]
impl std::ops::DerefMut for crate::System::Data::PrimaryKeyTypeConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+PrimaryKeyTypeConverter")]
impl crate::System::Data::PrimaryKeyTypeConverter {
    pub fn CanConvertTo(
        &mut self,
        context: *mut crate::System::ComponentModel::ITypeDescriptorContext,
        destinationType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CanConvertTo", (context, destinationType))?;
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
#[cfg(feature = "System+Data+PrimaryKeyTypeConverter")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::PrimaryKeyTypeConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
