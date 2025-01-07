#[cfg(feature = "EnvironmentColorManager")]
#[repr(C)]
#[derive(Debug)]
pub struct EnvironmentColorManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _defaultColorScheme: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ColorSchemeSO,
    >,
    pub _environmentColor0: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SimpleColorSO,
    >,
    pub _environmentColor1: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SimpleColorSO,
    >,
    pub _environmentColor0Boost: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SimpleColorSO,
    >,
    pub _environmentColor1Boost: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SimpleColorSO,
    >,
    pub _colorScheme: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
    pub didChangeColorEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
}
#[cfg(feature = "EnvironmentColorManager")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::EnvironmentColorManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "EnvironmentColorManager";
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
#[cfg(feature = "EnvironmentColorManager")]
impl std::ops::Deref for crate::GlobalNamespace::EnvironmentColorManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentColorManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::EnvironmentColorManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentColorManager")]
impl crate::GlobalNamespace::EnvironmentColorManager {
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitColors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitColors", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetColorScheme(
        &mut self,
        colorScheme: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColorScheme", (colorScheme))?;
        Ok(__cordl_ret.into())
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
    pub fn add_didChangeColorEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didChangeColorEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_environmentColor0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_environmentColor0", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_environmentColor0Boost(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_environmentColor0Boost", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_environmentColor1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_environmentColor1", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_environmentColor1Boost(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_environmentColor1Boost", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didChangeColorEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didChangeColorEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "EnvironmentColorManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::EnvironmentColorManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "EnvironmentColorManager")]
impl AsRef<crate::GlobalNamespace::IEnvironmentColorProvider>
for crate::GlobalNamespace::EnvironmentColorManager {
    fn as_ref(&self) -> &crate::GlobalNamespace::IEnvironmentColorProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "EnvironmentColorManager")]
impl AsMut<crate::GlobalNamespace::IEnvironmentColorProvider>
for crate::GlobalNamespace::EnvironmentColorManager {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IEnvironmentColorProvider {
        unsafe { std::mem::transmute(self) }
    }
}
