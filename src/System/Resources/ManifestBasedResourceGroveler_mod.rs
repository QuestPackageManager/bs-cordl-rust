#[cfg(feature = "System+Resources+ManifestBasedResourceGroveler")]
#[repr(C)]
#[derive(Debug)]
pub struct ManifestBasedResourceGroveler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _mediator: quest_hook::libil2cpp::Gc<
        crate::System::Resources::ResourceManager_ResourceManagerMediator,
    >,
}
#[cfg(feature = "System+Resources+ManifestBasedResourceGroveler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Resources::ManifestBasedResourceGroveler
    => "System.Resources"."ManifestBasedResourceGroveler"
);
#[cfg(feature = "System+Resources+ManifestBasedResourceGroveler")]
impl std::ops::Deref for crate::System::Resources::ManifestBasedResourceGroveler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Resources+ManifestBasedResourceGroveler")]
impl std::ops::DerefMut for crate::System::Resources::ManifestBasedResourceGroveler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Resources+ManifestBasedResourceGroveler")]
impl crate::System::Resources::ManifestBasedResourceGroveler {
    pub fn GetNeutralResourcesLanguage(
        a: quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
        fallbackLocation: quest_hook::libil2cpp::ByRefMut<
            crate::System::Resources::UltimateResourceFallbackLocation,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CultureInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNeutralResourcesLanguage", (a, fallbackLocation))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNeutralResourcesLanguageAttribute(
        assembly: quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
        cultureName: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
        fallbackLocation: quest_hook::libil2cpp::ByRefMut<i16>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetNeutralResourcesLanguageAttribute",
                (assembly, cultureName, fallbackLocation),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        mediator: quest_hook::libil2cpp::Gc<
            crate::System::Resources::ResourceManager_ResourceManagerMediator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (mediator))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        mediator: quest_hook::libil2cpp::Gc<
            crate::System::Resources::ResourceManager_ResourceManagerMediator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (mediator))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Resources+ManifestBasedResourceGroveler")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Resources::ManifestBasedResourceGroveler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Resources+ManifestBasedResourceGroveler")]
impl AsRef<crate::System::Resources::IResourceGroveler>
for crate::System::Resources::ManifestBasedResourceGroveler {
    fn as_ref(&self) -> &crate::System::Resources::IResourceGroveler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Resources+ManifestBasedResourceGroveler")]
impl AsMut<crate::System::Resources::IResourceGroveler>
for crate::System::Resources::ManifestBasedResourceGroveler {
    fn as_mut(&mut self) -> &mut crate::System::Resources::IResourceGroveler {
        unsafe { std::mem::transmute(self) }
    }
}
