#[cfg(feature = "UnityEngine+UIElements+DefaultGroupManager")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultGroupManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_GroupOptions: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IGroupBoxOption>,
        >,
    >,
    pub m_SelectedOption: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::IGroupBoxOption,
    >,
    pub m_GroupBox: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IGroupBox>,
}
#[cfg(feature = "UnityEngine+UIElements+DefaultGroupManager")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::DefaultGroupManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "DefaultGroupManager";
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
#[cfg(feature = "UnityEngine+UIElements+DefaultGroupManager")]
impl std::ops::Deref for crate::UnityEngine::UIElements::DefaultGroupManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DefaultGroupManager")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::DefaultGroupManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DefaultGroupManager")]
impl crate::UnityEngine::UIElements::DefaultGroupManager {
    pub fn Init(
        &mut self,
        groupBox: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IGroupBox>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::DefaultGroupManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IGroupBox>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Init")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::DefaultGroupManager as
                    quest_hook::libil2cpp::Type > ::class(), "Init", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (groupBox))?
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
    pub fn OnOptionSelectionChanged(
        &mut self,
        selectedOption: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IGroupBoxOption,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::DefaultGroupManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::IGroupBoxOption,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnOptionSelectionChanged")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::DefaultGroupManager as
                    quest_hook::libil2cpp::Type > ::class(), "OnOptionSelectionChanged",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (selectedOption))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterOption(
        &mut self,
        option: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IGroupBoxOption,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::DefaultGroupManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::IGroupBoxOption,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RegisterOption")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::DefaultGroupManager as
                    quest_hook::libil2cpp::Type > ::class(), "RegisterOption", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (option))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterOption(
        &mut self,
        option: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IGroupBoxOption,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::DefaultGroupManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::IGroupBoxOption,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("UnregisterOption")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::DefaultGroupManager as
                    quest_hook::libil2cpp::Type > ::class(), "UnregisterOption", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (option))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::DefaultGroupManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::DefaultGroupManager as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+DefaultGroupManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::DefaultGroupManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+DefaultGroupManager")]
impl AsRef<crate::UnityEngine::UIElements::IGroupManager>
for crate::UnityEngine::UIElements::DefaultGroupManager {
    fn as_ref(&self) -> &crate::UnityEngine::UIElements::IGroupManager {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DefaultGroupManager")]
impl AsMut<crate::UnityEngine::UIElements::IGroupManager>
for crate::UnityEngine::UIElements::DefaultGroupManager {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UIElements::IGroupManager {
        unsafe { std::mem::transmute(self) }
    }
}
