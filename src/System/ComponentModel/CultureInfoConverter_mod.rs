#[cfg(feature = "System+ComponentModel+CultureInfoConverter+CultureComparer")]
#[repr(C)]
#[derive(Debug)]
pub struct CultureInfoConverter_CultureComparer {
    __cordl_parent: crate::System::Object,
    pub _converter: *mut crate::System::ComponentModel::CultureInfoConverter,
}
#[cfg(feature = "System+ComponentModel+CultureInfoConverter+CultureComparer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::CultureInfoConverter_CultureComparer =>
    "System.ComponentModel"."CultureInfoConverter/CultureComparer"
);
#[cfg(feature = "System+ComponentModel+CultureInfoConverter+CultureComparer")]
impl std::ops::Deref
for crate::System::ComponentModel::CultureInfoConverter_CultureComparer {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+CultureInfoConverter+CultureComparer")]
impl std::ops::DerefMut
for crate::System::ComponentModel::CultureInfoConverter_CultureComparer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+CultureInfoConverter+CultureComparer")]
impl crate::System::ComponentModel::CultureInfoConverter_CultureComparer {
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
    pub fn New(
        cultureConverter: *mut crate::System::ComponentModel::CultureInfoConverter,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cultureConverter))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        cultureConverter: *mut crate::System::ComponentModel::CultureInfoConverter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cultureConverter))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+ComponentModel+CultureInfoConverter+CultureComparer")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::CultureInfoConverter_CultureComparer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+ComponentModel+CultureInfoConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct CultureInfoConverter {
    __cordl_parent: crate::System::ComponentModel::TypeConverter,
    pub _values: *mut crate::System::ComponentModel::TypeConverter_StandardValuesCollection,
}
#[cfg(feature = "System+ComponentModel+CultureInfoConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::ComponentModel::CultureInfoConverter =>
    "System.ComponentModel"."CultureInfoConverter"
);
#[cfg(feature = "System+ComponentModel+CultureInfoConverter")]
impl std::ops::Deref for crate::System::ComponentModel::CultureInfoConverter {
    type Target = crate::System::ComponentModel::TypeConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+CultureInfoConverter")]
impl std::ops::DerefMut for crate::System::ComponentModel::CultureInfoConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+CultureInfoConverter")]
impl crate::System::ComponentModel::CultureInfoConverter {
    pub const DefaultInvariantCultureString: &'static str = "(Default)";
    #[cfg(feature = "System+ComponentModel+CultureInfoConverter+CultureComparer")]
    pub type CultureComparer = crate::System::ComponentModel::CultureInfoConverter_CultureComparer;
    #[cfg(feature = "System+ComponentModel+CultureInfoConverter+CultureInfoMapper")]
    pub type CultureInfoMapper = crate::System::ComponentModel::CultureInfoConverter_CultureInfoMapper;
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
    pub fn GetCultureName(
        &mut self,
        culture: *mut crate::System::Globalization::CultureInfo,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetCultureName", (culture))?;
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
    pub fn get_DefaultCultureString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_DefaultCultureString", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+ComponentModel+CultureInfoConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::CultureInfoConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+ComponentModel+CultureInfoConverter+CultureInfoMapper")]
#[repr(C)]
#[derive(Debug)]
pub struct CultureInfoConverter_CultureInfoMapper {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+ComponentModel+CultureInfoConverter+CultureInfoMapper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::CultureInfoConverter_CultureInfoMapper =>
    "System.ComponentModel"."CultureInfoConverter/CultureInfoMapper"
);
#[cfg(feature = "System+ComponentModel+CultureInfoConverter+CultureInfoMapper")]
impl std::ops::Deref
for crate::System::ComponentModel::CultureInfoConverter_CultureInfoMapper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+CultureInfoConverter+CultureInfoMapper")]
impl std::ops::DerefMut
for crate::System::ComponentModel::CultureInfoConverter_CultureInfoMapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+CultureInfoConverter+CultureInfoMapper")]
impl crate::System::ComponentModel::CultureInfoConverter_CultureInfoMapper {}
#[cfg(feature = "System+ComponentModel+CultureInfoConverter+CultureInfoMapper")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::CultureInfoConverter_CultureInfoMapper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
