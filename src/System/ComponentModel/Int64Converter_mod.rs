#[cfg(feature = "System+ComponentModel+Int64Converter")]
#[repr(C)]
#[derive(Debug)]
pub struct Int64Converter {
    __cordl_parent: crate::System::ComponentModel::BaseNumberConverter,
}
#[cfg(feature = "System+ComponentModel+Int64Converter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::ComponentModel::Int64Converter =>
    "System.ComponentModel"."Int64Converter"
);
#[cfg(feature = "System+ComponentModel+Int64Converter")]
impl std::ops::Deref for crate::System::ComponentModel::Int64Converter {
    type Target = crate::System::ComponentModel::BaseNumberConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+Int64Converter")]
impl std::ops::DerefMut for crate::System::ComponentModel::Int64Converter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+Int64Converter")]
impl crate::System::ComponentModel::Int64Converter {
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
    pub fn ToString(
        &mut self,
        value: *mut crate::System::Object,
        formatInfo: *mut crate::System::Globalization::NumberFormatInfo,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", (value, formatInfo))?;
        Ok(__cordl_ret)
    }
    pub fn FromString_i32_0(
        &mut self,
        value: *mut crate::System::String,
        radix: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("FromString", (value, radix))?;
        Ok(__cordl_ret)
    }
    pub fn FromString_NumberFormatInfo1(
        &mut self,
        value: *mut crate::System::String,
        formatInfo: *mut crate::System::Globalization::NumberFormatInfo,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("FromString", (value, formatInfo))?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+ComponentModel+Int64Converter")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::Int64Converter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
