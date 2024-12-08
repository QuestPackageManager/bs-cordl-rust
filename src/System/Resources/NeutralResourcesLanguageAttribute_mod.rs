#[cfg(feature = "System+Resources+NeutralResourcesLanguageAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct NeutralResourcesLanguageAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _CultureName_k__BackingField: *mut crate::System::String,
    pub _Location_k__BackingField: crate::System::Resources::UltimateResourceFallbackLocation,
}
#[cfg(feature = "System+Resources+NeutralResourcesLanguageAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Resources::NeutralResourcesLanguageAttribute => "System.Resources"
    ."NeutralResourcesLanguageAttribute"
);
#[cfg(feature = "System+Resources+NeutralResourcesLanguageAttribute")]
impl std::ops::Deref for crate::System::Resources::NeutralResourcesLanguageAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Resources+NeutralResourcesLanguageAttribute")]
impl std::ops::DerefMut for crate::System::Resources::NeutralResourcesLanguageAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Resources+NeutralResourcesLanguageAttribute")]
impl crate::System::Resources::NeutralResourcesLanguageAttribute {
    pub fn get_CultureName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_CultureName", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        cultureName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cultureName))?;
        Ok(__cordl_ret)
    }
    pub fn get_Location(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Resources::UltimateResourceFallbackLocation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Resources::UltimateResourceFallbackLocation = __cordl_object
            .invoke("get_Location", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        cultureName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cultureName))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Resources+NeutralResourcesLanguageAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Resources::NeutralResourcesLanguageAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
