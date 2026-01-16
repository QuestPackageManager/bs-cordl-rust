#[cfg(feature = "cordl_class_UnityEngine+Rendering+DebugDisplaySettingsVolume")]
#[repr(C)]
#[derive(Debug)]
pub struct DebugDisplaySettingsVolume {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _volumeDebugSettings_k__BackingField:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::IVolumeDebugSettings>,
    pub volumeComponentEnumIndex: i32,
    pub debugState: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::VolumeComponent>,
        >,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+DebugDisplaySettingsVolume")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::DebugDisplaySettingsVolume
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "DebugDisplaySettingsVolume";
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
#[cfg(feature = "UnityEngine+Rendering+DebugDisplaySettingsVolume")]
impl std::ops::Deref for crate::UnityEngine::Rendering::DebugDisplaySettingsVolume {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+DebugDisplaySettingsVolume")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::DebugDisplaySettingsVolume {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+DebugDisplaySettingsVolume")]
impl crate::UnityEngine::Rendering::DebugDisplaySettingsVolume {
    pub const k_PanelTitle: &'static str = "Volume";
    #[cfg(feature = "UnityEngine+Rendering+DebugDisplaySettingsVolume+SettingsPanel")]
    pub type SettingsPanel =
        crate::UnityEngine::Rendering::DebugDisplaySettingsVolume_SettingsPanel;
    #[cfg(feature = "UnityEngine+Rendering+DebugDisplaySettingsVolume+Strings")]
    pub type Strings = crate::UnityEngine::Rendering::DebugDisplaySettingsVolume_Strings;
    #[cfg(feature = "UnityEngine+Rendering+DebugDisplaySettingsVolume+Styles")]
    pub type Styles = crate::UnityEngine::Rendering::DebugDisplaySettingsVolume_Styles;
    #[cfg(feature = "UnityEngine+Rendering+DebugDisplaySettingsVolume+WidgetFactory")]
    pub type WidgetFactory =
        crate::UnityEngine::Rendering::DebugDisplaySettingsVolume_WidgetFactory;
    pub fn CreatePanel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::IDebugDisplaySettingsPanelDisposable,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::IDebugDisplaySettingsPanelDisposable,
                    >, 0usize>("CreatePanel")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreatePanel",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::IDebugDisplaySettingsPanelDisposable,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        volumeDebugSettings: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::IVolumeDebugSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (volumeDebugSettings))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        volumeDebugSettings: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::IVolumeDebugSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::IVolumeDebugSettings,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (volumeDebugSettings))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_AreAnySettingsActive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_AreAnySettingsActive")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_AreAnySettingsActive",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_volumeDebugSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::IVolumeDebugSettings>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::IVolumeDebugSettings,
                        >,
                        0usize,
                    >("get_volumeDebugSettings")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_volumeDebugSettings", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::IVolumeDebugSettings,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+DebugDisplaySettingsVolume")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::DebugDisplaySettingsVolume
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Rendering+DebugDisplaySettingsVolume")]
impl AsRef<crate::UnityEngine::Rendering::IDebugDisplaySettingsData>
    for crate::UnityEngine::Rendering::DebugDisplaySettingsVolume
{
    fn as_ref(&self) -> &crate::UnityEngine::Rendering::IDebugDisplaySettingsData {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+DebugDisplaySettingsVolume")]
impl AsMut<crate::UnityEngine::Rendering::IDebugDisplaySettingsData>
    for crate::UnityEngine::Rendering::DebugDisplaySettingsVolume
{
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Rendering::IDebugDisplaySettingsData {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+DebugDisplaySettingsVolume")]
impl AsRef<crate::UnityEngine::Rendering::IDebugDisplaySettingsQuery>
    for crate::UnityEngine::Rendering::DebugDisplaySettingsVolume
{
    fn as_ref(&self) -> &crate::UnityEngine::Rendering::IDebugDisplaySettingsQuery {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+DebugDisplaySettingsVolume")]
impl AsMut<crate::UnityEngine::Rendering::IDebugDisplaySettingsQuery>
    for crate::UnityEngine::Rendering::DebugDisplaySettingsVolume
{
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Rendering::IDebugDisplaySettingsQuery {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+DebugDisplaySettingsVolume+SettingsPanel")]
#[repr(C)]
#[derive(Debug)]
pub struct DebugDisplaySettingsVolume_SettingsPanel {
    __cordl_parent: crate::UnityEngine::Rendering::DebugDisplaySettingsPanel_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugDisplaySettingsVolume>,
    >,
    pub m_VolumeTable: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugUI_Table>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+DebugDisplaySettingsVolume+SettingsPanel")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::DebugDisplaySettingsVolume_SettingsPanel
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "DebugDisplaySettingsVolume/SettingsPanel";
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
#[cfg(feature = "UnityEngine+Rendering+DebugDisplaySettingsVolume+SettingsPanel")]
impl std::ops::Deref for crate::UnityEngine::Rendering::DebugDisplaySettingsVolume_SettingsPanel {
    type Target = crate::UnityEngine::Rendering::DebugDisplaySettingsPanel_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugDisplaySettingsVolume>,
    >;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+DebugDisplaySettingsVolume+SettingsPanel")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::DebugDisplaySettingsVolume_SettingsPanel
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+DebugDisplaySettingsVolume+SettingsPanel")]
impl crate::UnityEngine::Rendering::DebugDisplaySettingsVolume_SettingsPanel {
    pub fn New(
        data: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugDisplaySettingsVolume>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (data))?;
        Ok(__cordl_object.into())
    }
    pub fn Refresh(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Refresh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Refresh",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn __ctor_b__0_0(
        &mut self,
        _cordl__: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::DebugUI_Field_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
            >,
        >,
        __: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::DebugUI_Field_1<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
                    ), quest_hook::libil2cpp::Void, 2usize>("<.ctor>b__0_0")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "<.ctor>b__0_0",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (_cordl__, __))? };
        Ok(__cordl_ret.into())
    }
    pub fn __ctor_b__0_1(
        &mut self,
        _cordl__: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugUI_Field_1<i32>>,
        __: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::DebugUI_Field_1<i32>,
                        >,
                        i32,
                    ), quest_hook::libil2cpp::Void, 2usize>("<.ctor>b__0_1")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "<.ctor>b__0_1",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (_cordl__, __))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        data: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugDisplaySettingsVolume>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::DebugDisplaySettingsVolume,
                    >), quest_hook::libil2cpp::Void, 1usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (data))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+DebugDisplaySettingsVolume+SettingsPanel")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::DebugDisplaySettingsVolume_SettingsPanel
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+DebugDisplaySettingsVolume+Strings")]
#[repr(C)]
#[derive(Debug)]
pub struct DebugDisplaySettingsVolume_Strings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+DebugDisplaySettingsVolume+Strings")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::DebugDisplaySettingsVolume_Strings
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "DebugDisplaySettingsVolume/Strings";
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
#[cfg(feature = "UnityEngine+Rendering+DebugDisplaySettingsVolume+Strings")]
impl std::ops::Deref for crate::UnityEngine::Rendering::DebugDisplaySettingsVolume_Strings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+DebugDisplaySettingsVolume+Strings")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::DebugDisplaySettingsVolume_Strings {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+DebugDisplaySettingsVolume+Strings")]
impl crate::UnityEngine::Rendering::DebugDisplaySettingsVolume_Strings {}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+DebugDisplaySettingsVolume+Strings")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::DebugDisplaySettingsVolume_Strings
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+DebugDisplaySettingsVolume+Styles")]
#[repr(C)]
#[derive(Debug)]
pub struct DebugDisplaySettingsVolume_Styles {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+DebugDisplaySettingsVolume+Styles")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::DebugDisplaySettingsVolume_Styles
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "DebugDisplaySettingsVolume/Styles";
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
#[cfg(feature = "UnityEngine+Rendering+DebugDisplaySettingsVolume+Styles")]
impl std::ops::Deref for crate::UnityEngine::Rendering::DebugDisplaySettingsVolume_Styles {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+DebugDisplaySettingsVolume+Styles")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::DebugDisplaySettingsVolume_Styles {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+DebugDisplaySettingsVolume+Styles")]
impl crate::UnityEngine::Rendering::DebugDisplaySettingsVolume_Styles {}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+DebugDisplaySettingsVolume+Styles")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::DebugDisplaySettingsVolume_Styles
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+DebugDisplaySettingsVolume+WidgetFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct DebugDisplaySettingsVolume_WidgetFactory {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+DebugDisplaySettingsVolume+WidgetFactory")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::DebugDisplaySettingsVolume_WidgetFactory
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "DebugDisplaySettingsVolume/WidgetFactory";
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
#[cfg(feature = "UnityEngine+Rendering+DebugDisplaySettingsVolume+WidgetFactory")]
impl std::ops::Deref for crate::UnityEngine::Rendering::DebugDisplaySettingsVolume_WidgetFactory {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+DebugDisplaySettingsVolume+WidgetFactory")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::DebugDisplaySettingsVolume_WidgetFactory
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+DebugDisplaySettingsVolume+WidgetFactory")]
impl crate::UnityEngine::Rendering::DebugDisplaySettingsVolume_WidgetFactory {
    #[cfg(
        feature = "UnityEngine+Rendering+DebugDisplaySettingsVolume+WidgetFactory+VolumeParameterChain"
    )]
    pub type VolumeParameterChain = crate::UnityEngine::Rendering::WidgetFactory_DebugDisplaySettingsVolume_VolumeParameterChain;
    pub fn CreateCameraSelector(
        panel: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::DebugDisplaySettingsVolume_SettingsPanel,
        >,
        refresh: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::Rendering::DebugUI_Field_1<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
                    >,
                >,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugUI_ObjectPopupField>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::DebugDisplaySettingsVolume_SettingsPanel,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_2<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::Rendering::DebugUI_Field_1<
                                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
                                    >,
                                >,
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::DebugUI_ObjectPopupField,
                    >, 2usize>("CreateCameraSelector")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateCameraSelector",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::DebugUI_ObjectPopupField,
        > = unsafe { cordl_method_info.invoke_unchecked((), (panel, refresh))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateComponentSelector(
        panel: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::DebugDisplaySettingsVolume_SettingsPanel,
        >,
        refresh: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugUI_Field_1<i32>>,
                i32,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugUI_EnumField>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::DebugDisplaySettingsVolume_SettingsPanel,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_2<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::Rendering::DebugUI_Field_1<i32>,
                                    >,
                                    i32,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::DebugUI_EnumField,
                        >,
                        2usize,
                    >("CreateComponentSelector")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateComponentSelector", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::DebugUI_EnumField,
        > = unsafe { cordl_method_info.invoke_unchecked((), (panel, refresh))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateVolumeParameterWidget(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isResultParameter: bool,
        param: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::VolumeParameter>,
        isHiddenCallback: quest_hook::libil2cpp::Gc<crate::System::Func_1<bool>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugUI_Widget>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::VolumeParameter,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Func_1<bool>>,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::DebugUI_Widget,
                        >,
                        4usize,
                    >("CreateVolumeParameterWidget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateVolumeParameterWidget", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugUI_Widget> = unsafe {
            cordl_method_info
                .invoke_unchecked((), (name, isResultParameter, param, isHiddenCallback))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateVolumeTable(
        data: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugDisplaySettingsVolume>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugUI_Table>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::DebugDisplaySettingsVolume,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::DebugUI_Table,
                        >,
                        1usize,
                    >("CreateVolumeTable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateVolumeTable", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugUI_Table> =
            unsafe { cordl_method_info.invoke_unchecked((), (data))? };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateTableColumns(
        table: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugUI_Table>,
        data: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugDisplaySettingsVolume>,
        resolutionChain: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::Rendering::WidgetFactory_DebugDisplaySettingsVolume_VolumeParameterChain,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::DebugUI_Table,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::DebugDisplaySettingsVolume,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    crate::UnityEngine::Rendering::WidgetFactory_DebugDisplaySettingsVolume_VolumeParameterChain,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("GenerateTableColumns")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GenerateTableColumns", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (table, data, resolutionChain))? };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateTableRows(
        table: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugUI_Table>,
        resolutionChain: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::Rendering::WidgetFactory_DebugDisplaySettingsVolume_VolumeParameterChain,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::DebugUI_Table,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    crate::UnityEngine::Rendering::WidgetFactory_DebugDisplaySettingsVolume_VolumeParameterChain,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("GenerateTableRows")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GenerateTableRows", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (table, resolutionChain))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetResolutionChain(
        data: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::DebugDisplaySettingsVolume,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::Rendering::WidgetFactory_DebugDisplaySettingsVolume_VolumeParameterChain,
            >,
        >,
    >{
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::DebugDisplaySettingsVolume,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::UnityEngine::Rendering::WidgetFactory_DebugDisplaySettingsVolume_VolumeParameterChain,
                            >,
                        >,
                        1usize,
                    >("GetResolutionChain")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetResolutionChain", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::Rendering::WidgetFactory_DebugDisplaySettingsVolume_VolumeParameterChain,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (data))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetSelectedVolumeComponent(
        profile: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::VolumeProfile>,
        selectedType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::VolumeComponent>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::VolumeProfile,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::VolumeComponent,
                        >,
                        2usize,
                    >("GetSelectedVolumeComponent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetSelectedVolumeComponent", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::VolumeComponent> =
            unsafe { cordl_method_info.invoke_unchecked((), (profile, selectedType))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetTableColumnVisibility(
        data: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugDisplaySettingsVolume>,
        table: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugUI_Table>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::DebugDisplaySettingsVolume,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugUI_Table>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetTableColumnVisibility"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetTableColumnVisibility",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (data, table))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+DebugDisplaySettingsVolume+WidgetFactory")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::DebugDisplaySettingsVolume_WidgetFactory
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+DebugDisplaySettingsVolume+WidgetFactory+VolumeParameterChain"
)]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct WidgetFactory_DebugDisplaySettingsVolume_VolumeParameterChain {
    pub nameAndTooltip: crate::UnityEngine::Rendering::Widget_DebugUI_NameAndTooltip,
    pub volumeProfile: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::VolumeProfile>,
    pub volumeComponent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::VolumeComponent>,
    pub volume: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Volume>,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+DebugDisplaySettingsVolume+WidgetFactory+VolumeParameterChain"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::WidgetFactory_DebugDisplaySettingsVolume_VolumeParameterChain
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str =
        "DebugDisplaySettingsVolume/WidgetFactory/VolumeParameterChain";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+DebugDisplaySettingsVolume+WidgetFactory+VolumeParameterChain"
)]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::WidgetFactory_DebugDisplaySettingsVolume_VolumeParameterChain
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+DebugDisplaySettingsVolume+WidgetFactory+VolumeParameterChain"
)]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::WidgetFactory_DebugDisplaySettingsVolume_VolumeParameterChain
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+DebugDisplaySettingsVolume+WidgetFactory+VolumeParameterChain"
)]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::WidgetFactory_DebugDisplaySettingsVolume_VolumeParameterChain
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+DebugDisplaySettingsVolume+WidgetFactory+VolumeParameterChain"
)]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::WidgetFactory_DebugDisplaySettingsVolume_VolumeParameterChain
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+DebugDisplaySettingsVolume+WidgetFactory+VolumeParameterChain"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::WidgetFactory_DebugDisplaySettingsVolume_VolumeParameterChain
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+DebugDisplaySettingsVolume+WidgetFactory+VolumeParameterChain"
)]
impl crate::UnityEngine::Rendering::WidgetFactory_DebugDisplaySettingsVolume_VolumeParameterChain {}
