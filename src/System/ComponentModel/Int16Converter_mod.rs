#[cfg(feature = "System+ComponentModel+Int16Converter")]
#[repr(C)]
#[derive(Debug)]
pub struct Int16Converter {
    __cordl_parent: crate::System::ComponentModel::BaseNumberConverter,
}
#[cfg(feature = "System+ComponentModel+Int16Converter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::ComponentModel::Int16Converter =>
    "System.ComponentModel"."Int16Converter"
);
#[cfg(feature = "System+ComponentModel+Int16Converter")]
impl std::ops::Deref for crate::System::ComponentModel::Int16Converter {
    type Target = crate::System::ComponentModel::BaseNumberConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+Int16Converter")]
impl std::ops::DerefMut for crate::System::ComponentModel::Int16Converter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+Int16Converter")]
impl crate::System::ComponentModel::Int16Converter {
    pub fn FromString_NumberFormatInfo1(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppString,
        formatInfo: *mut crate::System::Globalization::NumberFormatInfo,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("FromString", (value, formatInfo))?;
        Ok(__cordl_ret)
    }
    pub fn FromString_i32_0(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppString,
        radix: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("FromString", (value, radix))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ToString(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppObject,
        formatInfo: *mut crate::System::Globalization::NumberFormatInfo,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("ToString", (value, formatInfo))?;
        Ok(__cordl_ret)
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
    pub fn get_TargetType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_TargetType", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+ComponentModel+Int16Converter")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::Int16Converter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
