#[cfg(feature = "BTSCharacterMaterialSwitcher")]
#[repr(C)]
#[derive(Debug)]
pub struct BTSCharacterMaterialSwitcher {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _rendererMaterialsPairs: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::BTSCharacterMaterialSwitcher_RendererMaterialsPairs,
            >,
        >,
    >,
}
#[cfg(feature = "BTSCharacterMaterialSwitcher")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BTSCharacterMaterialSwitcher =>
    ""."BTSCharacterMaterialSwitcher"
);
#[cfg(feature = "BTSCharacterMaterialSwitcher")]
impl std::ops::Deref for crate::GlobalNamespace::BTSCharacterMaterialSwitcher {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BTSCharacterMaterialSwitcher")]
impl std::ops::DerefMut for crate::GlobalNamespace::BTSCharacterMaterialSwitcher {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BTSCharacterMaterialSwitcher")]
impl crate::GlobalNamespace::BTSCharacterMaterialSwitcher {
    #[cfg(feature = "BTSCharacterMaterialSwitcher+MaterialPairs")]
    pub type MaterialPairs = crate::GlobalNamespace::BTSCharacterMaterialSwitcher_MaterialPairs;
    #[cfg(feature = "BTSCharacterMaterialSwitcher+RendererMaterialsPairs")]
    pub type RendererMaterialsPairs = crate::GlobalNamespace::BTSCharacterMaterialSwitcher_RendererMaterialsPairs;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SwapMaterials(
        &mut self,
        alternative: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SwapMaterials", (alternative))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BTSCharacterMaterialSwitcher")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BTSCharacterMaterialSwitcher {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BTSCharacterMaterialSwitcher+MaterialPairs")]
#[repr(C)]
#[derive(Debug)]
pub struct BTSCharacterMaterialSwitcher_MaterialPairs {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub materialIndex: i32,
    pub defaultMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub alternativeMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
}
#[cfg(feature = "BTSCharacterMaterialSwitcher+MaterialPairs")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BTSCharacterMaterialSwitcher_MaterialPairs => ""
    ."BTSCharacterMaterialSwitcher/MaterialPairs"
);
#[cfg(feature = "BTSCharacterMaterialSwitcher+MaterialPairs")]
impl std::ops::Deref
for crate::GlobalNamespace::BTSCharacterMaterialSwitcher_MaterialPairs {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BTSCharacterMaterialSwitcher+MaterialPairs")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BTSCharacterMaterialSwitcher_MaterialPairs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BTSCharacterMaterialSwitcher+MaterialPairs")]
impl crate::GlobalNamespace::BTSCharacterMaterialSwitcher_MaterialPairs {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BTSCharacterMaterialSwitcher+MaterialPairs")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BTSCharacterMaterialSwitcher_MaterialPairs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BTSCharacterMaterialSwitcher+RendererMaterialsPairs")]
#[repr(C)]
#[derive(Debug)]
pub struct BTSCharacterMaterialSwitcher_RendererMaterialsPairs {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _renderer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
    pub _materialPairs: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::BTSCharacterMaterialSwitcher_MaterialPairs,
            >,
        >,
    >,
}
#[cfg(feature = "BTSCharacterMaterialSwitcher+RendererMaterialsPairs")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BTSCharacterMaterialSwitcher_RendererMaterialsPairs => ""
    ."BTSCharacterMaterialSwitcher/RendererMaterialsPairs"
);
#[cfg(feature = "BTSCharacterMaterialSwitcher+RendererMaterialsPairs")]
impl std::ops::Deref
for crate::GlobalNamespace::BTSCharacterMaterialSwitcher_RendererMaterialsPairs {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BTSCharacterMaterialSwitcher+RendererMaterialsPairs")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BTSCharacterMaterialSwitcher_RendererMaterialsPairs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BTSCharacterMaterialSwitcher+RendererMaterialsPairs")]
impl crate::GlobalNamespace::BTSCharacterMaterialSwitcher_RendererMaterialsPairs {
    pub fn New(
        renderer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (renderer))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        renderer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (renderer))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_materialPairs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BTSCharacterMaterialSwitcher_MaterialPairs,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BTSCharacterMaterialSwitcher_MaterialPairs,
                >,
            >,
        > = __cordl_object.invoke("get_materialPairs", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_renderer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer> = __cordl_object
            .invoke("get_renderer", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BTSCharacterMaterialSwitcher+RendererMaterialsPairs")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BTSCharacterMaterialSwitcher_RendererMaterialsPairs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
