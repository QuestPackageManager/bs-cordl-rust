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
    pub m_Style: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
}
#[cfg(feature = "UnityEngine+GUILayoutEntry")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::GUILayoutEntry {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "GUILayoutEntry";
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
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutOption>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::GUILayoutEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutOption>,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ApplyOptions")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::GUILayoutEntry as quest_hook::libil2cpp::Type >
                    ::class(), "ApplyOptions", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (options))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ApplyStyleSettings(
        &mut self,
        style: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::GUILayoutEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ApplyStyleSettings")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::GUILayoutEntry as quest_hook::libil2cpp::Type >
                    ::class(), "ApplyStyleSettings", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (style))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CalcHeight(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::GUILayoutEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("CalcHeight")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::GUILayoutEntry as quest_hook::libil2cpp::Type >
                    ::class(), "CalcHeight", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CalcWidth(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::GUILayoutEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("CalcWidth")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::GUILayoutEntry as quest_hook::libil2cpp::Type >
                    ::class(), "CalcWidth", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New_Il2CppArray1(
        _minWidth: f32,
        _maxWidth: f32,
        _minHeight: f32,
        _maxHeight: f32,
        _style: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
        options: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutOption>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (_minWidth, _maxWidth, _minHeight, _maxHeight, _style, options),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_f32_f32_f32_f32_GUIStyle0(
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::GUILayoutEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32, f32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetHorizontal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::GUILayoutEntry as quest_hook::libil2cpp::Type >
                    ::class(), "SetHorizontal", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (x, width))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetVertical(
        &mut self,
        y: f32,
        height: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::GUILayoutEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32, f32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetVertical")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::GUILayoutEntry as quest_hook::libil2cpp::Type >
                    ::class(), "SetVertical", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (y, height))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::GUILayoutEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::GUILayoutEntry as quest_hook::libil2cpp::Type >
                    ::class(), "ToString", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray1(
        &mut self,
        _minWidth: f32,
        _maxWidth: f32,
        _minHeight: f32,
        _maxHeight: f32,
        _style: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
        options: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutOption>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::GUILayoutEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    f32,
                    f32,
                    f32,
                    f32,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GUILayoutOption,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::GUILayoutEntry as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (_minWidth, _maxWidth, _minHeight, _maxHeight, _style, options),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f32_f32_f32_f32_GUIStyle0(
        &mut self,
        _minWidth: f32,
        _maxWidth: f32,
        _minHeight: f32,
        _maxHeight: f32,
        _style: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::GUILayoutEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    f32,
                    f32,
                    f32,
                    f32,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::GUILayoutEntry as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (_minWidth, _maxWidth, _minHeight, _maxHeight, _style),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_marginBottom(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::GUILayoutEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_marginBottom")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::GUILayoutEntry as quest_hook::libil2cpp::Type >
                    ::class(), "get_marginBottom", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_marginHorizontal(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::GUILayoutEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_marginHorizontal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::GUILayoutEntry as quest_hook::libil2cpp::Type >
                    ::class(), "get_marginHorizontal", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_marginLeft(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::GUILayoutEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_marginLeft")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::GUILayoutEntry as quest_hook::libil2cpp::Type >
                    ::class(), "get_marginLeft", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_marginRight(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::GUILayoutEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_marginRight")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::GUILayoutEntry as quest_hook::libil2cpp::Type >
                    ::class(), "get_marginRight", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_marginTop(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::GUILayoutEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_marginTop")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::GUILayoutEntry as quest_hook::libil2cpp::Type >
                    ::class(), "get_marginTop", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_marginVertical(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::GUILayoutEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_marginVertical")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::GUILayoutEntry as quest_hook::libil2cpp::Type >
                    ::class(), "get_marginVertical", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_style(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::GUILayoutEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
                0usize,
            >("get_style")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::GUILayoutEntry as quest_hook::libil2cpp::Type >
                    ::class(), "get_style", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_style(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::GUILayoutEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_style")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::GUILayoutEntry as quest_hook::libil2cpp::Type >
                    ::class(), "set_style", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
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
