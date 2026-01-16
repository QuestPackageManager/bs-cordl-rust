#[cfg(feature = "cordl_class_UnityEngine+ICanvasRaycastFilter")]
#[derive(Debug)]
#[repr(C)]
pub struct ICanvasRaycastFilter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+ICanvasRaycastFilter")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::ICanvasRaycastFilter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "ICanvasRaycastFilter";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "UnityEngine+ICanvasRaycastFilter")]
impl std::ops::Deref for crate::UnityEngine::ICanvasRaycastFilter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ICanvasRaycastFilter")]
impl std::ops::DerefMut for crate::UnityEngine::ICanvasRaycastFilter {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ICanvasRaycastFilter")]
impl crate::UnityEngine::ICanvasRaycastFilter {
    pub fn IsRaycastLocationValid(
        &mut self,
        sp: crate::UnityEngine::Vector2,
        eventCamera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Vector2,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                    ), bool, 2usize>("IsRaycastLocationValid")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IsRaycastLocationValid",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (sp, eventCamera))? };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(object_param: *mut quest_hook::libil2cpp::Il2CppObject) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+ICanvasRaycastFilter")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ICanvasRaycastFilter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
