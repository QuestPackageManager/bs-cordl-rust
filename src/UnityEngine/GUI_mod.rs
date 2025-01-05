#[cfg(feature = "UnityEngine+GUI")]
#[repr(C)]
#[derive(Debug)]
pub struct GUI {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+GUI")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::GUI => "UnityEngine"."GUI"
);
#[cfg(feature = "UnityEngine+GUI")]
impl std::ops::Deref for crate::UnityEngine::GUI {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUI")]
impl std::ops::DerefMut for crate::UnityEngine::GUI {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUI")]
impl crate::UnityEngine::GUI {
    #[cfg(feature = "UnityEngine+GUI+WindowFunction")]
    pub type WindowFunction = crate::UnityEngine::GUI_WindowFunction;
    pub fn BeginGroup_Rect_Gc_Gc0(
        position: crate::UnityEngine::Rect,
        content: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIContent>,
        style: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BeginGroup", (position, content, style))?;
        Ok(__cordl_ret.into())
    }
    pub fn BeginGroup_Vector2_1(
        position: crate::UnityEngine::Rect,
        content: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIContent>,
        style: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
        scrollOffset: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BeginGroup", (position, content, style, scrollOffset))?;
        Ok(__cordl_ret.into())
    }
    pub fn Box_Gc1(
        position: crate::UnityEngine::Rect,
        content: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIContent>,
        style: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Box", (position, content, style))?;
        Ok(__cordl_ret.into())
    }
    pub fn Box_Rect_Gc0(
        position: crate::UnityEngine::Rect,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Box", (position, text))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallWindowDelegate(
        func: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUI_WindowFunction>,
        id: i32,
        instanceID: i32,
        _skin: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUISkin>,
        forceRect: i32,
        width: f32,
        height: f32,
        style: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CallWindowDelegate",
                (func, id, instanceID, _skin, forceRect, width, height, style),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn DoLabel(
        position: crate::UnityEngine::Rect,
        content: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIContent>,
        style: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DoLabel", (position, content, style))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoSetSkin(
        newSkin: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUISkin>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DoSetSkin", (newSkin))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndGroup() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EndGroup", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Label_Gc1(
        position: crate::UnityEngine::Rect,
        content: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIContent>,
        style: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Label", (position, content, style))?;
        Ok(__cordl_ret.into())
    }
    pub fn Label_Rect_Gc0(
        position: crate::UnityEngine::Rect,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Label", (position, text))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_backgroundColor() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Color,
    > {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_backgroundColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_backgroundColor_Injected(
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_backgroundColor_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_changed() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_changed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_color() -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_color", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_color_Injected(
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_color_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_contentColor() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Color,
    > {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_contentColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_contentColor_Injected(
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_contentColor_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_enabled() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_enabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_matrix() -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_matrix", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_scrollViewStates() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngineInternal::GenericStack>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngineInternal::GenericStack,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_scrollViewStates", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_skin() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GUISkin>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUISkin> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_skin", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_backgroundColor(
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_backgroundColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_backgroundColor_Injected(
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_backgroundColor_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_changed(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_changed", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_color(
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_color", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_color_Injected(
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_color_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_contentColor(
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_contentColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_contentColor_Injected(
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_contentColor_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_enabled(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_enabled", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_matrix(
        value: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_matrix", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_nextScrollStepTime(
        value: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_nextScrollStepTime", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_skin(
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUISkin>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_skin", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+GUI")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::GUI {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+GUI+WindowFunction")]
#[repr(C)]
#[derive(Debug)]
pub struct GUI_WindowFunction {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
}
#[cfg(feature = "UnityEngine+GUI+WindowFunction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::GUI_WindowFunction => "UnityEngine"
    ."GUI/WindowFunction"
);
#[cfg(feature = "UnityEngine+GUI+WindowFunction")]
impl std::ops::Deref for crate::UnityEngine::GUI_WindowFunction {
    type Target = quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUI+WindowFunction")]
impl std::ops::DerefMut for crate::UnityEngine::GUI_WindowFunction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUI+WindowFunction")]
impl crate::UnityEngine::GUI_WindowFunction {
    pub fn Invoke(
        &mut self,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (id))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+GUI+WindowFunction")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::GUI_WindowFunction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
