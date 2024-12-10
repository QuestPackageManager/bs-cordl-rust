#[cfg(feature = "UnityEngine+GUIClip")]
#[repr(C)]
#[derive(Debug)]
pub struct GUIClip {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+GUIClip")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::GUIClip => "UnityEngine"."GUIClip"
);
#[cfg(feature = "UnityEngine+GUIClip")]
impl std::ops::Deref for crate::UnityEngine::GUIClip {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUIClip")]
impl std::ops::DerefMut for crate::UnityEngine::GUIClip {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUIClip")]
impl crate::UnityEngine::GUIClip {
    #[cfg(feature = "UnityEngine+GUIClip+ParentClipScope")]
    pub type ParentClipScope = crate::UnityEngine::GUIClip_ParentClipScope;
}
#[cfg(feature = "UnityEngine+GUIClip")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::GUIClip {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+GUIClip+ParentClipScope")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct GUIClip_ParentClipScope {
    pub m_Disposed: bool,
}
#[cfg(feature = "UnityEngine+GUIClip+ParentClipScope")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::GUIClip_ParentClipScope =>
    "UnityEngine"."GUIClip/ParentClipScope"
);
#[cfg(feature = "UnityEngine+GUIClip+ParentClipScope")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::GUIClip_ParentClipScope {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+GUIClip+ParentClipScope")]
impl crate::UnityEngine::GUIClip_ParentClipScope {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        objectTransform: crate::UnityEngine::Matrix4x4,
        clipRect: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (objectTransform, clipRect),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+GUIClip+ParentClipScope")]
impl AsRef<crate::System::IDisposable> for crate::UnityEngine::GUIClip_ParentClipScope {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+GUIClip+ParentClipScope")]
impl AsMut<crate::System::IDisposable> for crate::UnityEngine::GUIClip_ParentClipScope {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
