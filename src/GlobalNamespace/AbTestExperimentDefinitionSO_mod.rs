#[cfg(feature = "AbTestExperimentDefinitionSO")]
#[repr(C)]
#[derive(Debug)]
pub struct AbTestExperimentDefinitionSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _experimentName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _test1GroupSize: f32,
    pub _test2GroupSize: f32,
    pub _controlGroupSize: f32,
    pub _salt: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _currentUserTreatmentGroup: crate::GlobalNamespace::AbTestExperimentDefinitionSO_Group,
}
#[cfg(feature = "AbTestExperimentDefinitionSO")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::AbTestExperimentDefinitionSO {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "AbTestExperimentDefinitionSO";
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
#[cfg(feature = "AbTestExperimentDefinitionSO")]
impl std::ops::Deref for crate::GlobalNamespace::AbTestExperimentDefinitionSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AbTestExperimentDefinitionSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::AbTestExperimentDefinitionSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AbTestExperimentDefinitionSO")]
impl crate::GlobalNamespace::AbTestExperimentDefinitionSO {
    #[cfg(feature = "AbTestExperimentDefinitionSO+Group")]
    pub type Group = crate::GlobalNamespace::AbTestExperimentDefinitionSO_Group;
    pub fn AbSplit(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::AbTestExperimentDefinitionSO_Group,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::AbTestExperimentDefinitionSO as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                crate::GlobalNamespace::AbTestExperimentDefinitionSO_Group,
                1usize,
            >("AbSplit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::AbTestExperimentDefinitionSO as
                    quest_hook::libil2cpp::Type > ::class(), "AbSplit", 1usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::AbTestExperimentDefinitionSO_Group = unsafe {
            method.invoke_unchecked(self, (userId))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeCurrentUserTreatment(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::AbTestExperimentDefinitionSO as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ComputeCurrentUserTreatment")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::AbTestExperimentDefinitionSO as
                    quest_hook::libil2cpp::Type > ::class(),
                    "ComputeCurrentUserTreatment", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (userId))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ForceSetTreatmentGroup(
        &mut self,
        group: crate::GlobalNamespace::AbTestExperimentDefinitionSO_Group,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::AbTestExperimentDefinitionSO as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::GlobalNamespace::AbTestExperimentDefinitionSO_Group),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ForceSetTreatmentGroup")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::AbTestExperimentDefinitionSO as
                    quest_hook::libil2cpp::Type > ::class(), "ForceSetTreatmentGroup",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (group))?
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
    pub fn OnValidate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::AbTestExperimentDefinitionSO as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnValidate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::AbTestExperimentDefinitionSO as
                    quest_hook::libil2cpp::Type > ::class(), "OnValidate", 0usize
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::AbTestExperimentDefinitionSO as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::AbTestExperimentDefinitionSO as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_currentUserTreatmentGroup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::AbTestExperimentDefinitionSO_Group,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::AbTestExperimentDefinitionSO as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::GlobalNamespace::AbTestExperimentDefinitionSO_Group,
                0usize,
            >("get_currentUserTreatmentGroup")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::AbTestExperimentDefinitionSO as
                    quest_hook::libil2cpp::Type > ::class(),
                    "get_currentUserTreatmentGroup", 0usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::AbTestExperimentDefinitionSO_Group = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_experimentName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::AbTestExperimentDefinitionSO as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_experimentName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::AbTestExperimentDefinitionSO as
                    quest_hook::libil2cpp::Type > ::class(), "get_experimentName", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_test1GroupSize(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::AbTestExperimentDefinitionSO as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_test1GroupSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::AbTestExperimentDefinitionSO as
                    quest_hook::libil2cpp::Type > ::class(), "get_test1GroupSize", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_test2GroupSize(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::AbTestExperimentDefinitionSO as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_test2GroupSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::AbTestExperimentDefinitionSO as
                    quest_hook::libil2cpp::Type > ::class(), "get_test2GroupSize", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "AbTestExperimentDefinitionSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::AbTestExperimentDefinitionSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "AbTestExperimentDefinitionSO+Group")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AbTestExperimentDefinitionSO_Group {
    #[default]
    Control = 0i32,
    Test1 = 1i32,
    Test2 = 2i32,
}
#[cfg(feature = "AbTestExperimentDefinitionSO+Group")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::AbTestExperimentDefinitionSO_Group {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "AbTestExperimentDefinitionSO/Group";
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
#[cfg(feature = "AbTestExperimentDefinitionSO+Group")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::AbTestExperimentDefinitionSO_Group {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "AbTestExperimentDefinitionSO+Group")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::AbTestExperimentDefinitionSO_Group {
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
#[cfg(feature = "AbTestExperimentDefinitionSO+Group")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::AbTestExperimentDefinitionSO_Group {
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
#[cfg(feature = "AbTestExperimentDefinitionSO+Group")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::AbTestExperimentDefinitionSO_Group {
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
