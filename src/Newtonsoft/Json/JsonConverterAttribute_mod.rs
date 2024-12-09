#[cfg(feature = "Newtonsoft+Json+JsonConverterAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonConverterAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _converterType: *mut crate::System::Type,
    pub _ConverterParameters_k__BackingField: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppObject,
    >,
}
#[cfg(feature = "Newtonsoft+Json+JsonConverterAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::JsonConverterAttribute =>
    "Newtonsoft.Json"."JsonConverterAttribute"
);
#[cfg(feature = "Newtonsoft+Json+JsonConverterAttribute")]
impl std::ops::Deref for crate::Newtonsoft::Json::JsonConverterAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+JsonConverterAttribute")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::JsonConverterAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+JsonConverterAttribute")]
impl crate::Newtonsoft::Json::JsonConverterAttribute {
    pub fn New_Il2CppArray1(
        converterType: *mut crate::System::Type,
        converterParameters: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (converterType, converterParameters))?;
        Ok(__cordl_object)
    }
    pub fn New_Type0(
        converterType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (converterType))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_Il2CppArray1(
        &mut self,
        converterType: *mut crate::System::Type,
        converterParameters: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (converterType, converterParameters))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Type0(
        &mut self,
        converterType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (converterType))?;
        Ok(__cordl_ret)
    }
    pub fn get_ConverterParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_ConverterParameters", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ConverterType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_ConverterType", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Newtonsoft+Json+JsonConverterAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::JsonConverterAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
