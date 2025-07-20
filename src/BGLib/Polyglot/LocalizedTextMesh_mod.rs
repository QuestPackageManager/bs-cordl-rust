#[cfg(feature = "BGLib+Polyglot+LocalizedTextMesh")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalizedTextMesh {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub text: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextMesh>,
    pub key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "BGLib+Polyglot+LocalizedTextMesh")]
unsafe impl quest_hook::libil2cpp::Type for crate::BGLib::Polyglot::LocalizedTextMesh {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BGLib.Polyglot";
    const CLASS_NAME: &'static str = "LocalizedTextMesh";
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
#[cfg(feature = "BGLib+Polyglot+LocalizedTextMesh")]
impl std::ops::Deref for crate::BGLib::Polyglot::LocalizedTextMesh {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+Polyglot+LocalizedTextMesh")]
impl std::ops::DerefMut for crate::BGLib::Polyglot::LocalizedTextMesh {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+Polyglot+LocalizedTextMesh")]
impl crate::BGLib::Polyglot::LocalizedTextMesh {
    pub fn IsAlignmentLeft(
        &mut self,
        alignment: crate::UnityEngine::TextAlignment,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BGLib::Polyglot::LocalizedTextMesh as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::TextAlignment),
                bool,
                1usize,
            >("IsAlignmentLeft")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BGLib::Polyglot::LocalizedTextMesh as
                    quest_hook::libil2cpp::Type > ::class(), "IsAlignmentLeft", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (alignment))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsAlignmentRight(
        &mut self,
        alignment: crate::UnityEngine::TextAlignment,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BGLib::Polyglot::LocalizedTextMesh as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::TextAlignment),
                bool,
                1usize,
            >("IsAlignmentRight")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BGLib::Polyglot::LocalizedTextMesh as
                    quest_hook::libil2cpp::Type > ::class(), "IsAlignmentRight", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (alignment))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsOppositeDirection(
        &mut self,
        alignment: crate::UnityEngine::TextAlignment,
        direction: crate::BGLib::Polyglot::LanguageDirection,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BGLib::Polyglot::LocalizedTextMesh as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::TextAlignment,
                    crate::BGLib::Polyglot::LanguageDirection,
                ),
                bool,
                2usize,
            >("IsOppositeDirection")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BGLib::Polyglot::LocalizedTextMesh as
                    quest_hook::libil2cpp::Type > ::class(), "IsOppositeDirection",
                    2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (alignment, direction))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnLocalize(
        &mut self,
        localization: quest_hook::libil2cpp::Gc<
            crate::BGLib::Polyglot::LocalizationModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BGLib::Polyglot::LocalizedTextMesh as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::BGLib::Polyglot::LocalizationModel>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnLocalize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BGLib::Polyglot::LocalizedTextMesh as
                    quest_hook::libil2cpp::Type > ::class(), "OnLocalize", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (localization))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BGLib::Polyglot::LocalizedTextMesh as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Reset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BGLib::Polyglot::LocalizedTextMesh as
                    quest_hook::libil2cpp::Type > ::class(), "Reset", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BGLib::Polyglot::LocalizedTextMesh as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Start")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BGLib::Polyglot::LocalizedTextMesh as
                    quest_hook::libil2cpp::Type > ::class(), "Start", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BGLib::Polyglot::LocalizedTextMesh as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BGLib::Polyglot::LocalizedTextMesh as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Key(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BGLib::Polyglot::LocalizedTextMesh as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_Key")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BGLib::Polyglot::LocalizedTextMesh as
                    quest_hook::libil2cpp::Type > ::class(), "get_Key", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BGLib+Polyglot+LocalizedTextMesh")]
impl quest_hook::libil2cpp::ObjectType for crate::BGLib::Polyglot::LocalizedTextMesh {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BGLib+Polyglot+LocalizedTextMesh")]
impl AsRef<crate::BGLib::Polyglot::ILocalize>
for crate::BGLib::Polyglot::LocalizedTextMesh {
    fn as_ref(&self) -> &crate::BGLib::Polyglot::ILocalize {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BGLib+Polyglot+LocalizedTextMesh")]
impl AsMut<crate::BGLib::Polyglot::ILocalize>
for crate::BGLib::Polyglot::LocalizedTextMesh {
    fn as_mut(&mut self) -> &mut crate::BGLib::Polyglot::ILocalize {
        unsafe { std::mem::transmute(self) }
    }
}
