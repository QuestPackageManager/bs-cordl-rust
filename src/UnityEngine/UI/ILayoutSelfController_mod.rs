#[cfg(feature = "cordl_class_UnityEngine+UI+ILayoutSelfController")]
#[repr(C)]
#[derive(Debug)]
pub struct ILayoutSelfController {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+UI+ILayoutSelfController")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UI::ILayoutSelfController {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UI";
    const CLASS_NAME: &'static str = "ILayoutSelfController";
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
#[cfg(feature = "UnityEngine+UI+ILayoutSelfController")]
impl std::ops::Deref for crate::UnityEngine::UI::ILayoutSelfController {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+ILayoutSelfController")]
impl std::ops::DerefMut for crate::UnityEngine::UI::ILayoutSelfController {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+ILayoutSelfController")]
impl crate::UnityEngine::UI::ILayoutSelfController {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UI+ILayoutSelfController")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UI::ILayoutSelfController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UI+ILayoutSelfController")]
impl AsRef<crate::UnityEngine::UI::ILayoutController>
for crate::UnityEngine::UI::ILayoutSelfController {
    fn as_ref(&self) -> &crate::UnityEngine::UI::ILayoutController {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+ILayoutSelfController")]
impl AsMut<crate::UnityEngine::UI::ILayoutController>
for crate::UnityEngine::UI::ILayoutSelfController {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UI::ILayoutController {
        unsafe { std::mem::transmute(self) }
    }
}
