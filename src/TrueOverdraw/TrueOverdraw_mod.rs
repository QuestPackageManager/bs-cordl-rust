#[cfg(feature = "TrueOverdraw+TrueOverdraw")]
#[repr(C)]
#[derive(Debug)]
pub struct TrueOverdraw {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _overdrawType_k__BackingField: crate::TrueOverdraw::TrueOverdraw_OverdrawType,
    pub _renderers: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
        >,
    >,
    pub _cachedSharedMaterials: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                >,
            >,
        >,
    >,
    pub _cachedMaterialInstances: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                >,
            >,
        >,
    >,
}
#[cfg(feature = "TrueOverdraw+TrueOverdraw")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TrueOverdraw::TrueOverdraw => "TrueOverdraw"
    ."TrueOverdraw"
);
#[cfg(feature = "TrueOverdraw+TrueOverdraw")]
impl std::ops::Deref for crate::TrueOverdraw::TrueOverdraw {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TrueOverdraw+TrueOverdraw")]
impl std::ops::DerefMut for crate::TrueOverdraw::TrueOverdraw {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TrueOverdraw+TrueOverdraw")]
impl crate::TrueOverdraw::TrueOverdraw {
    pub const kOverdrawViewKeyword: &'static str = "OVERDRAW_VIEW";
    #[cfg(feature = "TrueOverdraw+TrueOverdraw+OverdrawType")]
    pub type OverdrawType = crate::TrueOverdraw::TrueOverdraw_OverdrawType;
    pub fn Disable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Disable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Enable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Enable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        renderers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (renderers))?;
        Ok(__cordl_object.into())
    }
    pub fn ResolveRendererColor(
        renderer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ResolveRendererColor", (renderer, material))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalFloats(
        trueOverdraw: f32,
        opaque: f32,
        transparent: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalFloats", (trueOverdraw, opaque, transparent))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMaterialValues(
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetMaterialValues", (material, color))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetOverdrawValues(
        opaque: f32,
        transparent: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetOverdrawValues", (opaque, transparent))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShowEverything(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowEverything", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ShowOnlyOpaque(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowOnlyOpaque", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ShowOnlyTransparent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowOnlyTransparent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        renderers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (renderers))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_overdrawType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::TrueOverdraw::TrueOverdraw_OverdrawType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::TrueOverdraw::TrueOverdraw_OverdrawType = __cordl_object
            .invoke("get_overdrawType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_renderersLength(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_renderersLength", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_overdrawType(
        &mut self,
        value: crate::TrueOverdraw::TrueOverdraw_OverdrawType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_overdrawType", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TrueOverdraw+TrueOverdraw")]
impl quest_hook::libil2cpp::ObjectType for crate::TrueOverdraw::TrueOverdraw {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TrueOverdraw+TrueOverdraw")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::TrueOverdraw::TrueOverdraw {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "TrueOverdraw+TrueOverdraw")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::TrueOverdraw::TrueOverdraw {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "TrueOverdraw+TrueOverdraw+OverdrawType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TrueOverdraw_OverdrawType {
    #[default]
    Everything = 3i32,
    None = 0i32,
    Opaque = 2i32,
    Transparent = 1i32,
}
#[cfg(feature = "TrueOverdraw+TrueOverdraw+OverdrawType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TrueOverdraw::TrueOverdraw_OverdrawType =>
    "TrueOverdraw"."TrueOverdraw/OverdrawType"
);
