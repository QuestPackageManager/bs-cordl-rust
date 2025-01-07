#[cfg(feature = "TMPro+TMP_DefaultControls")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_DefaultControls {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "TMPro+TMP_DefaultControls")]
unsafe impl quest_hook::libil2cpp::Type for crate::TMPro::TMP_DefaultControls {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "TMPro";
    const CLASS_NAME: &'static str = "TMP_DefaultControls";
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
#[cfg(feature = "TMPro+TMP_DefaultControls")]
impl std::ops::Deref for crate::TMPro::TMP_DefaultControls {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_DefaultControls")]
impl std::ops::DerefMut for crate::TMPro::TMP_DefaultControls {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_DefaultControls")]
impl crate::TMPro::TMP_DefaultControls {
    pub const kThickHeight: f32 = 30f32;
    pub const kThinHeight: f32 = 20f32;
    pub const kWidth: f32 = 160f32;
    #[cfg(feature = "TMPro+TMP_DefaultControls+Resources")]
    pub type Resources = crate::TMPro::TMP_DefaultControls_Resources;
    pub fn CreateButton(
        resources: crate::TMPro::TMP_DefaultControls_Resources,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateButton", (resources))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateDropdown(
        resources: crate::TMPro::TMP_DefaultControls_Resources,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateDropdown", (resources))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateInputField(
        resources: crate::TMPro::TMP_DefaultControls_Resources,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateInputField", (resources))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateScrollbar(
        resources: crate::TMPro::TMP_DefaultControls_Resources,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateScrollbar", (resources))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateText(
        resources: crate::TMPro::TMP_DefaultControls_Resources,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateText", (resources))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateUIElementRoot(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_size: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateUIElementRoot", (name, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateUIObject(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateUIObject", (name, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDefaultColorTransitionValues(
        slider: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Selectable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetDefaultColorTransitionValues", (slider))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDefaultTextValues(
        lbl: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetDefaultTextValues", (lbl))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLayerRecursively(
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        layer: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetLayerRecursively", (go, layer))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetParentAndAlign(
        child: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetParentAndAlign", (child, parent))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TMPro+TMP_DefaultControls")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_DefaultControls {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TMPro+TMP_DefaultControls+Resources")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TMP_DefaultControls_Resources {
    pub standard: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub background: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub inputField: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub knob: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub checkmark: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub dropdown: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub mask: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
}
#[cfg(feature = "TMPro+TMP_DefaultControls+Resources")]
unsafe impl quest_hook::libil2cpp::Type for crate::TMPro::TMP_DefaultControls_Resources {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "TMPro";
    const CLASS_NAME: &'static str = "Resources";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "TMPro+TMP_DefaultControls+Resources")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::TMPro::TMP_DefaultControls_Resources {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "TMPro+TMP_DefaultControls+Resources")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::TMPro::TMP_DefaultControls_Resources {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "TMPro+TMP_DefaultControls+Resources")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::TMPro::TMP_DefaultControls_Resources {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "TMPro+TMP_DefaultControls+Resources")]
unsafe impl quest_hook::libil2cpp::Return
for crate::TMPro::TMP_DefaultControls_Resources {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "TMPro+TMP_DefaultControls+Resources")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::TMPro::TMP_DefaultControls_Resources {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+TMP_DefaultControls+Resources")]
impl crate::TMPro::TMP_DefaultControls_Resources {}
