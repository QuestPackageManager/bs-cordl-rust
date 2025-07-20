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
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Resources::ManifestBasedResourceGroveler {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Resources";
    const CLASS_NAME: &'static str = "ManifestBasedResourceGroveler";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Resources::ManifestBasedResourceGroveler as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::System::Resources::UltimateResourceFallbackLocation,
                    >,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
                2usize,
            >("GetNeutralResourcesLanguage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Resources::ManifestBasedResourceGroveler as
                    quest_hook::libil2cpp::Type > ::class(),
                    "GetNeutralResourcesLanguage", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CultureInfo,
        > = unsafe { method.invoke_unchecked((), (a, fallbackLocation))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetNeutralResourcesLanguageAttribute(
        assembly: quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
        cultureName: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        fallbackLocation: quest_hook::libil2cpp::ByRefMut<i16>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Resources::ManifestBasedResourceGroveler as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                    quest_hook::libil2cpp::ByRefMut<i16>,
                ),
                bool,
                3usize,
            >("GetNeutralResourcesLanguageAttribute")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Resources::ManifestBasedResourceGroveler as
                    quest_hook::libil2cpp::Type > ::class(),
                    "GetNeutralResourcesLanguageAttribute", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (assembly, cultureName, fallbackLocation))?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Resources::ManifestBasedResourceGroveler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Resources::ResourceManager_ResourceManagerMediator,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Resources::ManifestBasedResourceGroveler as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (mediator))?
        };
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
