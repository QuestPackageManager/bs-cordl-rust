#[cfg(feature = "UnityEngine+UI+Shadow")]
#[repr(C)]
#[derive(Debug)]
pub struct Shadow {
    __cordl_parent: crate::UnityEngine::UI::BaseMeshEffect,
    pub m_EffectColor: crate::UnityEngine::Color,
    pub m_EffectDistance: crate::UnityEngine::Vector2,
    pub m_UseGraphicAlpha: bool,
}
#[cfg(feature = "UnityEngine+UI+Shadow")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::Shadow => "UnityEngine.UI"
    ."Shadow"
);
#[cfg(feature = "UnityEngine+UI+Shadow")]
impl std::ops::Deref for crate::UnityEngine::UI::Shadow {
    type Target = crate::UnityEngine::UI::BaseMeshEffect;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+Shadow")]
impl std::ops::DerefMut for crate::UnityEngine::UI::Shadow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+Shadow")]
impl crate::UnityEngine::UI::Shadow {
    pub const kMaxEffectDistance: f32 = 600f32;
    pub fn ApplyShadow(
        &mut self,
        verts: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIVertex,
        >,
        color: crate::UnityEngine::Color32,
        start: i32,
        end: i32,
        x: f32,
        y: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyShadow", (verts, color, start, end, x, y))?;
        Ok(__cordl_ret)
    }
    pub fn ApplyShadowZeroAlloc(
        &mut self,
        verts: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIVertex,
        >,
        color: crate::UnityEngine::Color32,
        start: i32,
        end: i32,
        x: f32,
        y: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyShadowZeroAlloc", (verts, color, start, end, x, y))?;
        Ok(__cordl_ret)
    }
    pub fn ModifyMesh(
        &mut self,
        vh: *mut crate::UnityEngine::UI::VertexHelper,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ModifyMesh", (vh))?;
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
    pub fn get_effectColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_effectColor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_effectDistance(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("get_effectDistance", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_useGraphicAlpha(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_useGraphicAlpha", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_effectColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_effectColor", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_effectDistance(
        &mut self,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_effectDistance", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_useGraphicAlpha(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_useGraphicAlpha", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UI+Shadow")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UI::Shadow {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
