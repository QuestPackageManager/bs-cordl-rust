#[cfg(feature = "UnityEngine+InputSystem+InputActionProperty")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct InputActionProperty {
    pub m_UseReference: bool,
    pub m_Action: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputAction,
    >,
    pub m_Reference: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputActionReference,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionProperty")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::InputActionProperty {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "InputActionProperty";
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
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::InputActionProperty {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::InputActionProperty {
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
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::InputActionProperty {
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
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::InputActionProperty {
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
#[cfg(feature = "UnityEngine+InputSystem+InputActionProperty")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputActionProperty {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionProperty")]
impl crate::UnityEngine::InputSystem::InputActionProperty {
    pub fn Equals_Il2CppObject3(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_InputAction1(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_InputActionProperty0(
        &mut self,
        other: crate::UnityEngine::InputSystem::InputActionProperty,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_InputActionReference2(
        &mut self,
        other: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionReference,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_InputAction0(
        &mut self,
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (action),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_InputActionReference1(
        &mut self,
        reference: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionReference,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (reference),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_action(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputAction,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_action", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_reference(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionReference>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionReference,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_reference", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_serializedAction(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputAction,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_serializedAction",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_serializedReference(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionReference>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionReference,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_serializedReference",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        left: crate::UnityEngine::InputSystem::InputActionProperty,
        right: crate::UnityEngine::InputSystem::InputActionProperty,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        left: crate::UnityEngine::InputSystem::InputActionProperty,
        right: crate::UnityEngine::InputSystem::InputActionProperty,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (left, right))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionProperty")]
impl AsRef<
    crate::System::IEquatable_1<crate::UnityEngine::InputSystem::InputActionProperty>,
> for crate::UnityEngine::InputSystem::InputActionProperty {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        crate::UnityEngine::InputSystem::InputActionProperty,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionProperty")]
impl AsMut<
    crate::System::IEquatable_1<crate::UnityEngine::InputSystem::InputActionProperty>,
> for crate::UnityEngine::InputSystem::InputActionProperty {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::InputSystem::InputActionProperty,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionProperty")]
impl AsRef<
    crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    >,
> for crate::UnityEngine::InputSystem::InputActionProperty {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionProperty")]
impl AsMut<
    crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    >,
> for crate::UnityEngine::InputSystem::InputActionProperty {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionProperty")]
impl AsRef<
    crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionReference>,
    >,
> for crate::UnityEngine::InputSystem::InputActionProperty {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionReference>,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionProperty")]
impl AsMut<
    crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionReference>,
    >,
> for crate::UnityEngine::InputSystem::InputActionProperty {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionReference>,
    > {
        todo!()
    }
}
