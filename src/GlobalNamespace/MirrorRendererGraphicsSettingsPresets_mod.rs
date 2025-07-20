#[cfg(feature = "MirrorRendererGraphicsSettingsPresets")]
#[repr(C)]
#[derive(Debug)]
pub struct MirrorRendererGraphicsSettingsPresets {
    __cordl_parent: crate::GlobalNamespace::NamedPresetsSO,
    pub _presets: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::MirrorRendererGraphicsSettingsPresets_Preset,
            >,
        >,
    >,
}
#[cfg(feature = "MirrorRendererGraphicsSettingsPresets")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MirrorRendererGraphicsSettingsPresets {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MirrorRendererGraphicsSettingsPresets";
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
#[cfg(feature = "MirrorRendererGraphicsSettingsPresets")]
impl std::ops::Deref for crate::GlobalNamespace::MirrorRendererGraphicsSettingsPresets {
    type Target = crate::GlobalNamespace::NamedPresetsSO;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MirrorRendererGraphicsSettingsPresets")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MirrorRendererGraphicsSettingsPresets {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MirrorRendererGraphicsSettingsPresets")]
impl crate::GlobalNamespace::MirrorRendererGraphicsSettingsPresets {
    #[cfg(feature = "MirrorRendererGraphicsSettingsPresets+Preset")]
    pub type Preset = crate::GlobalNamespace::MirrorRendererGraphicsSettingsPresets_Preset;
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_namedPresets(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NamedPreset>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::NamedPreset,
                                >,
                            >,
                        >,
                        0usize,
                    >("get_namedPresets")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_namedPresets", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NamedPreset>,
            >,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_presets(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MirrorRendererGraphicsSettingsPresets_Preset,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::MirrorRendererGraphicsSettingsPresets_Preset,
                                >,
                            >,
                        >,
                        0usize,
                    >("get_presets")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_presets", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MirrorRendererGraphicsSettingsPresets_Preset,
                >,
            >,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MirrorRendererGraphicsSettingsPresets")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MirrorRendererGraphicsSettingsPresets {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MirrorRendererGraphicsSettingsPresets+Preset")]
#[repr(C)]
#[derive(Debug)]
pub struct MirrorRendererGraphicsSettingsPresets_Preset {
    __cordl_parent: crate::GlobalNamespace::NamedPreset,
    pub mirrorType: crate::GlobalNamespace::Preset_MirrorRendererGraphicsSettingsPresets_MirrorType,
    pub reflectLayers: crate::UnityEngine::LayerMask,
    pub stereoTextureWidth: i32,
    pub stereoTextureHeight: i32,
    pub monoTextureWidth: i32,
    pub monoTextureHeight: i32,
    pub maxAntiAliasing: i32,
    pub enableBloomPrePassFog: bool,
}
#[cfg(feature = "MirrorRendererGraphicsSettingsPresets+Preset")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MirrorRendererGraphicsSettingsPresets_Preset {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MirrorRendererGraphicsSettingsPresets/Preset";
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
#[cfg(feature = "MirrorRendererGraphicsSettingsPresets+Preset")]
impl std::ops::Deref
for crate::GlobalNamespace::MirrorRendererGraphicsSettingsPresets_Preset {
    type Target = crate::GlobalNamespace::NamedPreset;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MirrorRendererGraphicsSettingsPresets+Preset")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MirrorRendererGraphicsSettingsPresets_Preset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MirrorRendererGraphicsSettingsPresets+Preset")]
impl crate::GlobalNamespace::MirrorRendererGraphicsSettingsPresets_Preset {
    #[cfg(feature = "MirrorRendererGraphicsSettingsPresets+Preset+MirrorType")]
    pub type MirrorType = crate::GlobalNamespace::Preset_MirrorRendererGraphicsSettingsPresets_MirrorType;
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MirrorRendererGraphicsSettingsPresets+Preset")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MirrorRendererGraphicsSettingsPresets_Preset {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MirrorRendererGraphicsSettingsPresets+Preset+MirrorType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Preset_MirrorRendererGraphicsSettingsPresets_MirrorType {
    #[default]
    FakeMirror = 1i32,
    None = 0i32,
    RenderedMirror = 2i32,
}
#[cfg(feature = "MirrorRendererGraphicsSettingsPresets+Preset+MirrorType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::Preset_MirrorRendererGraphicsSettingsPresets_MirrorType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MirrorRendererGraphicsSettingsPresets/Preset/MirrorType";
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
#[cfg(feature = "MirrorRendererGraphicsSettingsPresets+Preset+MirrorType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::Preset_MirrorRendererGraphicsSettingsPresets_MirrorType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "MirrorRendererGraphicsSettingsPresets+Preset+MirrorType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::Preset_MirrorRendererGraphicsSettingsPresets_MirrorType {
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
#[cfg(feature = "MirrorRendererGraphicsSettingsPresets+Preset+MirrorType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::Preset_MirrorRendererGraphicsSettingsPresets_MirrorType {
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
#[cfg(feature = "MirrorRendererGraphicsSettingsPresets+Preset+MirrorType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::Preset_MirrorRendererGraphicsSettingsPresets_MirrorType {
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
