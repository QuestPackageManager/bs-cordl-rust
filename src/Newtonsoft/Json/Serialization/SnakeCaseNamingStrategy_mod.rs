#[cfg(feature = "Newtonsoft+Json+Serialization+SnakeCaseNamingStrategy")]
#[repr(C)]
#[derive(Debug)]
pub struct SnakeCaseNamingStrategy {
    __cordl_parent: crate::Newtonsoft::Json::Serialization::NamingStrategy,
}
#[cfg(feature = "Newtonsoft+Json+Serialization+SnakeCaseNamingStrategy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Serialization::SnakeCaseNamingStrategy =>
    "Newtonsoft.Json.Serialization"."SnakeCaseNamingStrategy"
);
#[cfg(feature = "Newtonsoft+Json+Serialization+SnakeCaseNamingStrategy")]
impl std::ops::Deref
for crate::Newtonsoft::Json::Serialization::SnakeCaseNamingStrategy {
    type Target = crate::Newtonsoft::Json::Serialization::NamingStrategy;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+SnakeCaseNamingStrategy")]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Serialization::SnakeCaseNamingStrategy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+SnakeCaseNamingStrategy")]
impl crate::Newtonsoft::Json::Serialization::SnakeCaseNamingStrategy {
    pub fn ResolvePropertyName(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ResolvePropertyName", (name))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor__cordl_bool__cordl_bool0(
        &mut self,
        processDictionaryKeys: bool,
        overrideSpecifiedNames: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (processDictionaryKeys, overrideSpecifiedNames))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor__cordl_bool__cordl_bool__cordl_bool1(
        &mut self,
        processDictionaryKeys: bool,
        overrideSpecifiedNames: bool,
        processExtensionDataNames: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    processDictionaryKeys,
                    overrideSpecifiedNames,
                    processExtensionDataNames,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New__cordl_bool__cordl_bool0(
        processDictionaryKeys: bool,
        overrideSpecifiedNames: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (processDictionaryKeys, overrideSpecifiedNames))?;
        Ok(__cordl_object)
    }
    pub fn New__cordl_bool__cordl_bool__cordl_bool1(
        processDictionaryKeys: bool,
        overrideSpecifiedNames: bool,
        processExtensionDataNames: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    processDictionaryKeys,
                    overrideSpecifiedNames,
                    processExtensionDataNames,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_2() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+SnakeCaseNamingStrategy")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Serialization::SnakeCaseNamingStrategy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
