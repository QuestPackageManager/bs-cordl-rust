#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+InstantiationParameters"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct InstantiationParameters {
    pub m_Position: crate::UnityEngine::Vector3,
    pub m_Rotation: crate::UnityEngine::Quaternion,
    pub m_Parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub m_InstantiateInWorldPosition: bool,
    pub m_SetPositionRotation: bool,
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+InstantiationParameters"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ResourceManagement::ResourceProviders::InstantiationParameters {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.ResourceManagement.ResourceProviders";
    const CLASS_NAME: &'static str = "InstantiationParameters";
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
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+InstantiationParameters"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::ResourceManagement::ResourceProviders::InstantiationParameters {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+InstantiationParameters"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::ResourceManagement::ResourceProviders::InstantiationParameters {
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
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+InstantiationParameters"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::ResourceManagement::ResourceProviders::InstantiationParameters {
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
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+InstantiationParameters"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::ResourceManagement::ResourceProviders::InstantiationParameters {
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
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+InstantiationParameters"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ResourceManagement::ResourceProviders::InstantiationParameters {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+InstantiationParameters"
)]
impl crate::UnityEngine::ResourceManagement::ResourceProviders::InstantiationParameters {
    pub fn Instantiate<TObject>(
        &mut self,
        source: TObject,
    ) -> quest_hook::libil2cpp::Result<TObject>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TObject = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Instantiate",
            (source),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Transform__cordl_bool0(
        &mut self,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        instantiateInWorldSpace: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (parent, instantiateInWorldSpace),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Vector3_Quaternion_Transform1(
        &mut self,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (position, rotation, parent),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InstantiateInWorldPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_InstantiateInWorldPosition",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Parent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Parent",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Position(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Position",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Rotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Rotation",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SetPositionRotation(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_SetPositionRotation",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
