#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+TouchHistory")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TouchHistory {
    pub m_History: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1<
            crate::UnityEngine::InputSystem::LowLevel::TouchState,
        >,
    >,
    pub m_Finger: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::EnhancedTouch::Finger,
    >,
    pub m_Count: i32,
    pub m_StartIndex: i32,
    pub m_Version: u32,
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+TouchHistory")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::EnhancedTouch::TouchHistory {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.EnhancedTouch";
    const CLASS_NAME: &'static str = "TouchHistory";
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
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+TouchHistory")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::EnhancedTouch::TouchHistory {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+TouchHistory")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::EnhancedTouch::TouchHistory {
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
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+TouchHistory")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::EnhancedTouch::TouchHistory {
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
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+TouchHistory")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::EnhancedTouch::TouchHistory {
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
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+TouchHistory")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::EnhancedTouch::TouchHistory {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+TouchHistory")]
impl crate::UnityEngine::InputSystem::EnhancedTouch::TouchHistory {
    #[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+TouchHistory+Enumerator")]
    pub type Enumerator = crate::UnityEngine::InputSystem::EnhancedTouch::TouchHistory_Enumerator;
    pub fn CheckValid(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CheckValid",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                crate::UnityEngine::InputSystem::EnhancedTouch::Touch,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                crate::UnityEngine::InputSystem::EnhancedTouch::Touch,
            >,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.IEnumerable.GetEnumerator",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        finger: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::EnhancedTouch::Finger,
        >,
        history: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1<
                crate::UnityEngine::InputSystem::LowLevel::TouchState,
            >,
        >,
        startIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (finger, history, startIndex, count),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Count",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::EnhancedTouch::Touch,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::EnhancedTouch::Touch = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Item",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+TouchHistory")]
impl AsRef<
    crate::System::Collections::Generic::IEnumerable_1<
        crate::UnityEngine::InputSystem::EnhancedTouch::Touch,
    >,
> for crate::UnityEngine::InputSystem::EnhancedTouch::TouchHistory {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEnumerable_1<
        crate::UnityEngine::InputSystem::EnhancedTouch::Touch,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+TouchHistory")]
impl AsMut<
    crate::System::Collections::Generic::IEnumerable_1<
        crate::UnityEngine::InputSystem::EnhancedTouch::Touch,
    >,
> for crate::UnityEngine::InputSystem::EnhancedTouch::TouchHistory {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerable_1<
        crate::UnityEngine::InputSystem::EnhancedTouch::Touch,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+TouchHistory")]
impl AsRef<
    crate::System::Collections::Generic::IReadOnlyCollection_1<
        crate::UnityEngine::InputSystem::EnhancedTouch::Touch,
    >,
> for crate::UnityEngine::InputSystem::EnhancedTouch::TouchHistory {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IReadOnlyCollection_1<
        crate::UnityEngine::InputSystem::EnhancedTouch::Touch,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+TouchHistory")]
impl AsMut<
    crate::System::Collections::Generic::IReadOnlyCollection_1<
        crate::UnityEngine::InputSystem::EnhancedTouch::Touch,
    >,
> for crate::UnityEngine::InputSystem::EnhancedTouch::TouchHistory {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IReadOnlyCollection_1<
        crate::UnityEngine::InputSystem::EnhancedTouch::Touch,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+TouchHistory")]
impl AsRef<
    crate::System::Collections::Generic::IReadOnlyList_1<
        crate::UnityEngine::InputSystem::EnhancedTouch::Touch,
    >,
> for crate::UnityEngine::InputSystem::EnhancedTouch::TouchHistory {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IReadOnlyList_1<
        crate::UnityEngine::InputSystem::EnhancedTouch::Touch,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+TouchHistory")]
impl AsMut<
    crate::System::Collections::Generic::IReadOnlyList_1<
        crate::UnityEngine::InputSystem::EnhancedTouch::Touch,
    >,
> for crate::UnityEngine::InputSystem::EnhancedTouch::TouchHistory {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IReadOnlyList_1<
        crate::UnityEngine::InputSystem::EnhancedTouch::Touch,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+TouchHistory")]
impl AsRef<crate::System::Collections::IEnumerable>
for crate::UnityEngine::InputSystem::EnhancedTouch::TouchHistory {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+TouchHistory")]
impl AsMut<crate::System::Collections::IEnumerable>
for crate::UnityEngine::InputSystem::EnhancedTouch::TouchHistory {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+TouchHistory+Enumerator")]
#[repr(C)]
#[derive(Debug)]
pub struct TouchHistory_Enumerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Owner: crate::UnityEngine::InputSystem::EnhancedTouch::TouchHistory,
    pub m_Index: i32,
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+TouchHistory+Enumerator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::EnhancedTouch::TouchHistory_Enumerator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.EnhancedTouch";
    const CLASS_NAME: &'static str = "Enumerator";
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
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+TouchHistory+Enumerator")]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::EnhancedTouch::TouchHistory_Enumerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+TouchHistory+Enumerator")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::EnhancedTouch::TouchHistory_Enumerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+TouchHistory+Enumerator")]
impl crate::UnityEngine::InputSystem::EnhancedTouch::TouchHistory_Enumerator {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveNext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        owner: crate::UnityEngine::InputSystem::EnhancedTouch::TouchHistory,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (owner))?;
        Ok(__cordl_object.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerator_get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("System.Collections.IEnumerator.get_Current", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        owner: crate::UnityEngine::InputSystem::EnhancedTouch::TouchHistory,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (owner))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::EnhancedTouch::Touch,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::EnhancedTouch::Touch = __cordl_object
            .invoke("get_Current", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+TouchHistory+Enumerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::EnhancedTouch::TouchHistory_Enumerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+TouchHistory+Enumerator")]
impl AsRef<
    crate::System::Collections::Generic::IEnumerator_1<
        crate::UnityEngine::InputSystem::EnhancedTouch::Touch,
    >,
> for crate::UnityEngine::InputSystem::EnhancedTouch::TouchHistory_Enumerator {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEnumerator_1<
        crate::UnityEngine::InputSystem::EnhancedTouch::Touch,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+TouchHistory+Enumerator")]
impl AsMut<
    crate::System::Collections::Generic::IEnumerator_1<
        crate::UnityEngine::InputSystem::EnhancedTouch::Touch,
    >,
> for crate::UnityEngine::InputSystem::EnhancedTouch::TouchHistory_Enumerator {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerator_1<
        crate::UnityEngine::InputSystem::EnhancedTouch::Touch,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+TouchHistory+Enumerator")]
impl AsRef<crate::System::Collections::IEnumerator>
for crate::UnityEngine::InputSystem::EnhancedTouch::TouchHistory_Enumerator {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+TouchHistory+Enumerator")]
impl AsMut<crate::System::Collections::IEnumerator>
for crate::UnityEngine::InputSystem::EnhancedTouch::TouchHistory_Enumerator {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+TouchHistory+Enumerator")]
impl AsRef<crate::System::IDisposable>
for crate::UnityEngine::InputSystem::EnhancedTouch::TouchHistory_Enumerator {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+TouchHistory+Enumerator")]
impl AsMut<crate::System::IDisposable>
for crate::UnityEngine::InputSystem::EnhancedTouch::TouchHistory_Enumerator {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
