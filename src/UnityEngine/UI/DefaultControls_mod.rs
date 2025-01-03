#[cfg(feature = "UnityEngine+UI+DefaultControls")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultControls {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UI+DefaultControls")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::DefaultControls =>
    "UnityEngine.UI"."DefaultControls"
);
#[cfg(feature = "UnityEngine+UI+DefaultControls")]
impl std::ops::Deref for crate::UnityEngine::UI::DefaultControls {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+DefaultControls")]
impl std::ops::DerefMut for crate::UnityEngine::UI::DefaultControls {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+DefaultControls")]
impl crate::UnityEngine::UI::DefaultControls {
    pub const kThickHeight: f32 = 30f32;
    pub const kThinHeight: f32 = 20f32;
    pub const kWidth: f32 = 160f32;
    #[cfg(feature = "UnityEngine+UI+DefaultControls+DefaultRuntimeFactory")]
    pub type DefaultRuntimeFactory = crate::UnityEngine::UI::DefaultControls_DefaultRuntimeFactory;
    #[cfg(feature = "UnityEngine+UI+DefaultControls+IFactoryControls")]
    type IFactoryControls = crate::UnityEngine::UI::DefaultControls_IFactoryControls;
    #[cfg(feature = "UnityEngine+UI+DefaultControls+Resources")]
    pub type Resources = crate::UnityEngine::UI::DefaultControls_Resources;
    pub fn CreateButton(
        resources: crate::UnityEngine::UI::DefaultControls_Resources,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateButton", (resources))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateDropdown(
        resources: crate::UnityEngine::UI::DefaultControls_Resources,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateDropdown", (resources))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateImage(
        resources: crate::UnityEngine::UI::DefaultControls_Resources,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateImage", (resources))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateInputField(
        resources: crate::UnityEngine::UI::DefaultControls_Resources,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateInputField", (resources))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreatePanel(
        resources: crate::UnityEngine::UI::DefaultControls_Resources,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreatePanel", (resources))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateRawImage(
        resources: crate::UnityEngine::UI::DefaultControls_Resources,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateRawImage", (resources))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateScrollView(
        resources: crate::UnityEngine::UI::DefaultControls_Resources,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateScrollView", (resources))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateScrollbar(
        resources: crate::UnityEngine::UI::DefaultControls_Resources,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateScrollbar", (resources))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateSlider(
        resources: crate::UnityEngine::UI::DefaultControls_Resources,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateSlider", (resources))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateText(
        resources: crate::UnityEngine::UI::DefaultControls_Resources,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateText", (resources))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateToggle(
        resources: crate::UnityEngine::UI::DefaultControls_Resources,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateToggle", (resources))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateUIElementRoot(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_size: crate::UnityEngine::Vector2,
        components: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateUIElementRoot", (name, _cordl_size, components))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateUIObject(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        components: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateUIObject", (name, parent, components))?;
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
        lbl: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Text>,
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
    pub fn get_factory() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UI::DefaultControls_IFactoryControls,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UI::DefaultControls_IFactoryControls,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_factory", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UI+DefaultControls")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UI::DefaultControls {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UI+DefaultControls+DefaultRuntimeFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultControls_DefaultRuntimeFactory {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UI+DefaultControls+DefaultRuntimeFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UI::DefaultControls_DefaultRuntimeFactory => "UnityEngine.UI"
    ."DefaultControls/DefaultRuntimeFactory"
);
#[cfg(feature = "UnityEngine+UI+DefaultControls+DefaultRuntimeFactory")]
impl std::ops::Deref for crate::UnityEngine::UI::DefaultControls_DefaultRuntimeFactory {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+DefaultControls+DefaultRuntimeFactory")]
impl std::ops::DerefMut
for crate::UnityEngine::UI::DefaultControls_DefaultRuntimeFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+DefaultControls+DefaultRuntimeFactory")]
impl crate::UnityEngine::UI::DefaultControls_DefaultRuntimeFactory {
    pub fn CreateGameObject(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        components: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = __cordl_object
            .invoke("CreateGameObject", (name, components))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UI+DefaultControls+DefaultRuntimeFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UI::DefaultControls_DefaultRuntimeFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UI+DefaultControls+DefaultRuntimeFactory")]
impl AsRef<crate::UnityEngine::UI::DefaultControls_IFactoryControls>
for crate::UnityEngine::UI::DefaultControls_DefaultRuntimeFactory {
    fn as_ref(&self) -> &crate::UnityEngine::UI::DefaultControls_IFactoryControls {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+DefaultControls+DefaultRuntimeFactory")]
impl AsMut<crate::UnityEngine::UI::DefaultControls_IFactoryControls>
for crate::UnityEngine::UI::DefaultControls_DefaultRuntimeFactory {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::UI::DefaultControls_IFactoryControls {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+DefaultControls+IFactoryControls")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultControls_IFactoryControls {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UI+DefaultControls+IFactoryControls")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UI::DefaultControls_IFactoryControls => "UnityEngine.UI"
    ."DefaultControls/IFactoryControls"
);
#[cfg(feature = "UnityEngine+UI+DefaultControls+IFactoryControls")]
impl std::ops::Deref for crate::UnityEngine::UI::DefaultControls_IFactoryControls {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+DefaultControls+IFactoryControls")]
impl std::ops::DerefMut for crate::UnityEngine::UI::DefaultControls_IFactoryControls {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+DefaultControls+IFactoryControls")]
impl crate::UnityEngine::UI::DefaultControls_IFactoryControls {
    pub fn CreateGameObject(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        components: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = __cordl_object
            .invoke("CreateGameObject", (name, components))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+UI+DefaultControls+IFactoryControls")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UI::DefaultControls_IFactoryControls {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UI+DefaultControls+Resources")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DefaultControls_Resources {
    pub standard: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub background: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub inputField: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub knob: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub checkmark: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub dropdown: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub mask: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
}
#[cfg(feature = "UnityEngine+UI+DefaultControls+Resources")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::DefaultControls_Resources =>
    "UnityEngine.UI"."DefaultControls/Resources"
);
#[cfg(feature = "UnityEngine+UI+DefaultControls+Resources")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UI::DefaultControls_Resources {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UI+DefaultControls+Resources")]
impl crate::UnityEngine::UI::DefaultControls_Resources {}
