#[cfg(feature = "UnityEngine+GUILayoutEntry")]
#[repr(C)]
#[derive(Debug)]
pub struct GUILayoutEntry {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub minWidth: f32,
    pub maxWidth: f32,
    pub minHeight: f32,
    pub maxHeight: f32,
    pub rect: crate::UnityEngine::Rect,
    pub stretchWidth: i32,
    pub stretchHeight: i32,
    pub consideredForMargin: bool,
    pub m_Style: *mut crate::UnityEngine::GUIStyle,
}
#[cfg(feature = "UnityEngine+GUILayoutEntry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::GUILayoutEntry => "UnityEngine"
    ."GUILayoutEntry"
);
#[cfg(feature = "UnityEngine+GUILayoutEntry")]
impl std::ops::Deref for crate::UnityEngine::GUILayoutEntry {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUILayoutEntry")]
impl std::ops::DerefMut for crate::UnityEngine::GUILayoutEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUILayoutEntry")]
impl crate::UnityEngine::GUILayoutEntry {
    pub fn ApplyOptions(
        &mut self,
        options: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::GUILayoutOption>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyOptions", (options))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyStyleSettings(
        &mut self,
        style: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyStyleSettings", (style))?;
        Ok(__cordl_ret.into())
    }
    pub fn CalcHeight(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CalcHeight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CalcWidth(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CalcWidth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        _minWidth: f32,
        _maxWidth: f32,
        _minHeight: f32,
        _maxHeight: f32,
        _style: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (_minWidth, _maxWidth, _minHeight, _maxHeight, _style),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn SetHorizontal(
        &mut self,
        x: f32,
        width: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetHorizontal", (x, width))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetVertical(
        &mut self,
        y: f32,
        height: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetVertical", (y, height))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        _minWidth: f32,
        _maxWidth: f32,
        _minHeight: f32,
        _maxHeight: f32,
        _style: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_minWidth, _maxWidth, _minHeight, _maxHeight, _style))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_marginBottom(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_marginBottom", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_marginHorizontal(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_marginHorizontal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_marginLeft(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_marginLeft", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_marginRight(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_marginRight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_marginTop(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_marginTop", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_marginVertical(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_marginVertical", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_style(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle> = __cordl_object
            .invoke("get_style", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_style(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_style", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+GUILayoutEntry")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::GUILayoutEntry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
