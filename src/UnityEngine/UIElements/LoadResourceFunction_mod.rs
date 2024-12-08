#[cfg(feature = "UnityEngine+UIElements+LoadResourceFunction")]
#[repr(C)]
#[derive(Debug)]
pub struct LoadResourceFunction {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "UnityEngine+UIElements+LoadResourceFunction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::LoadResourceFunction =>
    "UnityEngine.UIElements"."LoadResourceFunction"
);
#[cfg(feature = "UnityEngine+UIElements+LoadResourceFunction")]
impl std::ops::Deref for crate::UnityEngine::UIElements::LoadResourceFunction {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+LoadResourceFunction")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::LoadResourceFunction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+LoadResourceFunction")]
impl crate::UnityEngine::UIElements::LoadResourceFunction {
    pub fn Invoke(
        &mut self,
        pathName: *mut crate::System::String,
        _cordl_type: *mut crate::System::Type,
        dpiScaling: f32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Object = __cordl_object
            .invoke("Invoke", (pathName, _cordl_type, dpiScaling))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+LoadResourceFunction")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::LoadResourceFunction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
