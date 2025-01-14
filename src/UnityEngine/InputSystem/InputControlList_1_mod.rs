#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct InputControlList_1<TControl: quest_hook::libil2cpp::Type> {
    pub m_Count: i32,
    pub m_Indices: crate::Unity::Collections::NativeArray_1<u64>,
    pub m_Allocator: crate::Unity::Collections::Allocator,
    __cordl_phantom_TControl: std::marker::PhantomData<TControl>,
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1")]
unsafe impl<TControl: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::InputControlList_1<TControl> {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "InputControlList`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "UnityEngine.InputSystem",
                        "InputControlList`1",
                    )
                    .unwrap()
                    .make_generic::<(TControl)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1")]
unsafe impl<TControl: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::InputControlList_1<TControl> {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1")]
unsafe impl<TControl: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::InputControlList_1<TControl> {
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
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1")]
unsafe impl<TControl: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::InputControlList_1<TControl> {
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
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1")]
unsafe impl<TControl: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::InputControlList_1<TControl> {
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
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1")]
unsafe impl<TControl: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputControlList_1<TControl> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1")]
impl<
    TControl: quest_hook::libil2cpp::Type,
> crate::UnityEngine::InputSystem::InputControlList_1<TControl> {
    pub const kInvalidIndex: u64 = 18446744073709551615u64;
    #[cfg(feature = "UnityEngine+InputSystem+InputControlList_1+Enumerator")]
    pub type Enumerator = crate::UnityEngine::InputSystem::InputControlList_1_Enumerator<
        TControl,
    >;
    pub fn Add(
        &mut self,
        item: TControl,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(TControl), quest_hook::libil2cpp::Void, 1usize>("Add")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Add", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (item))
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddRange(
        &mut self,
        list: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TControl>,
        >,
        count: i32,
        destinationIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IEnumerable_1<TControl>,
                    >,
                    i32,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("AddRange")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddRange", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (list, count, destinationIndex))
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddSlice<TList>(
        &mut self,
        list: TList,
        count: i32,
        destinationIndex: i32,
        sourceIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (TList, i32, i32, i32),
                quest_hook::libil2cpp::Void,
                4usize,
            >("AddSlice")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddSlice", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (list, count, destinationIndex, sourceIndex))
        };
        Ok(__cordl_ret.into())
    }
    pub fn AppendTo(
        &mut self,
        array: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TControl>>,
        >,
        count: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<TControl>,
                        >,
                    >,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("AppendTo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AppendTo", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (array, count))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Clear(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Clear")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Clear", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn Contains_TControl0(
        &mut self,
        item: TControl,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(TControl), bool, 1usize>("Contains")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Contains", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (item)) };
        Ok(__cordl_ret.into())
    }
    pub fn Contains_i32_i32_1(
        &mut self,
        item: TControl,
        startIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(TControl, i32, i32), bool, 3usize>("Contains")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Contains", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (item, startIndex, count))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CopyTo(
        &mut self,
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TControl>>,
        arrayIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<TControl>,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("CopyTo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CopyTo", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (array, arrayIndex))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Dispose", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromIndex(index: u64) -> quest_hook::libil2cpp::Result<TControl>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), TControl, 1usize>("FromIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FromIndex", 1usize
                )
            });
        let __cordl_ret: TControl = unsafe { method.invoke_unchecked((), (index)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<TControl>,
        >,
    >
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IEnumerator_1<TControl>,
                >,
                0usize,
            >("GetEnumerator")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetEnumerator", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<TControl>,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf_TControl0(
        &mut self,
        item: TControl,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(TControl), i32, 1usize>("IndexOf")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IndexOf", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (item)) };
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf_i32_i32_1(
        &mut self,
        item: TControl,
        startIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(TControl, i32, i32), i32, 3usize>("IndexOf")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IndexOf", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (item, startIndex, count))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Insert(
        &mut self,
        index: i32,
        item: TControl,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, TControl),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Insert")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Insert", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (index, item))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Remove(&mut self, item: TControl) -> quest_hook::libil2cpp::Result<bool>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(TControl), bool, 1usize>("Remove")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Remove", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (item)) };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveAt(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("RemoveAt")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RemoveAt", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (index))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Resize(
        &mut self,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("Resize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Resize", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (_cordl_size))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Sort<TCompare>(
        &mut self,
        startIndex: i32,
        count: i32,
        comparer: TCompare,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TCompare: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, TCompare),
                quest_hook::libil2cpp::Void,
                3usize,
            >("Sort")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Sort", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (startIndex, count, comparer))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SwapElements(
        &mut self,
        index1: i32,
        index2: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SwapElements")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SwapElements", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (index1, index2))
        };
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    >
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
                0usize,
            >("System.Collections.IEnumerable.GetEnumerator")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "System.Collections.IEnumerable.GetEnumerator", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ToArray(
        &mut self,
        dispose: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TControl>>,
    >
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TControl>>,
                1usize,
            >("ToArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToArray", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<TControl>,
        > = unsafe { method.invoke_unchecked(self, (dispose)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToIndex(control: TControl) -> quest_hook::libil2cpp::Result<u64>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(TControl), u64, 1usize>("ToIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToIndex", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (control)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Allocator_i32_0(
        &mut self,
        allocator: crate::Unity::Collections::Allocator,
        initialCapacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::Unity::Collections::Allocator, i32),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (allocator, initialCapacity))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IEnumerable_1_Allocator1(
        &mut self,
        values: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TControl>,
        >,
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IEnumerable_1<TControl>,
                    >,
                    crate::Unity::Collections::Allocator,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (values, allocator))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray2(
        &mut self,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TControl>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<TControl>,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (values))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Capacity(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_Capacity")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Capacity", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_Count")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Count", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsReadOnly(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_IsReadOnly")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_IsReadOnly", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(&mut self, index: i32) -> quest_hook::libil2cpp::Result<TControl>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), TControl, 1usize>("get_Item")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Item", 1usize
                )
            });
        let __cordl_ret: TControl = unsafe { method.invoke_unchecked(self, (index)) };
        Ok(__cordl_ret.into())
    }
    pub fn set_Capacity(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("set_Capacity")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_Capacity", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_Item(
        &mut self,
        index: i32,
        value: TControl,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, TControl),
                quest_hook::libil2cpp::Void,
                2usize,
            >("set_Item")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_Item", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (index, value))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1")]
impl<
    TControl: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::ICollection_1<TControl>>
for crate::UnityEngine::InputSystem::InputControlList_1<TControl> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::ICollection_1<TControl> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1")]
impl<
    TControl: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::ICollection_1<TControl>>
for crate::UnityEngine::InputSystem::InputControlList_1<TControl> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::ICollection_1<TControl> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1")]
impl<
    TControl: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IEnumerable_1<TControl>>
for crate::UnityEngine::InputSystem::InputControlList_1<TControl> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::IEnumerable_1<TControl> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1")]
impl<
    TControl: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IEnumerable_1<TControl>>
for crate::UnityEngine::InputSystem::InputControlList_1<TControl> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerable_1<TControl> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1")]
impl<
    TControl: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IList_1<TControl>>
for crate::UnityEngine::InputSystem::InputControlList_1<TControl> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::IList_1<TControl> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1")]
impl<
    TControl: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IList_1<TControl>>
for crate::UnityEngine::InputSystem::InputControlList_1<TControl> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::Generic::IList_1<TControl> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1")]
impl<
    TControl: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IReadOnlyCollection_1<TControl>>
for crate::UnityEngine::InputSystem::InputControlList_1<TControl> {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IReadOnlyCollection_1<TControl> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1")]
impl<
    TControl: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IReadOnlyCollection_1<TControl>>
for crate::UnityEngine::InputSystem::InputControlList_1<TControl> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IReadOnlyCollection_1<TControl> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1")]
impl<
    TControl: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IReadOnlyList_1<TControl>>
for crate::UnityEngine::InputSystem::InputControlList_1<TControl> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::IReadOnlyList_1<TControl> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1")]
impl<
    TControl: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IReadOnlyList_1<TControl>>
for crate::UnityEngine::InputSystem::InputControlList_1<TControl> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IReadOnlyList_1<TControl> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1")]
impl<
    TControl: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::IEnumerable>
for crate::UnityEngine::InputSystem::InputControlList_1<TControl> {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1")]
impl<
    TControl: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::IEnumerable>
for crate::UnityEngine::InputSystem::InputControlList_1<TControl> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1")]
impl<TControl: quest_hook::libil2cpp::Type> AsRef<crate::System::IDisposable>
for crate::UnityEngine::InputSystem::InputControlList_1<TControl> {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1")]
impl<TControl: quest_hook::libil2cpp::Type> AsMut<crate::System::IDisposable>
for crate::UnityEngine::InputSystem::InputControlList_1<TControl> {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1+Enumerator")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct InputControlList_1_Enumerator<TControl: quest_hook::libil2cpp::Type> {
    pub m_Indices: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub m_Count: i32,
    pub m_Current: i32,
    __cordl_phantom_TControl: std::marker::PhantomData<TControl>,
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1+Enumerator")]
unsafe impl<TControl: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::InputControlList_1_Enumerator<TControl> {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "InputControlList`1/Enumerator";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "UnityEngine.InputSystem",
                        "InputControlList`1/Enumerator",
                    )
                    .unwrap()
                    .make_generic::<(TControl)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1+Enumerator")]
unsafe impl<TControl: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::InputControlList_1_Enumerator<TControl> {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1+Enumerator")]
unsafe impl<TControl: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::InputControlList_1_Enumerator<TControl> {
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
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1+Enumerator")]
unsafe impl<TControl: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::InputControlList_1_Enumerator<TControl> {
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
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1+Enumerator")]
unsafe impl<TControl: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::InputControlList_1_Enumerator<TControl> {
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
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1+Enumerator")]
unsafe impl<TControl: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputControlList_1_Enumerator<TControl> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1+Enumerator")]
impl<
    TControl: quest_hook::libil2cpp::Type,
> crate::UnityEngine::InputSystem::InputControlList_1_Enumerator<TControl> {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Dispose", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("MoveNext")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "MoveNext", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn Reset(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Reset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Reset", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerator_get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                0usize,
            >("System.Collections.IEnumerator.get_Current")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "System.Collections.IEnumerator.get_Current", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        list: crate::UnityEngine::InputSystem::InputControlList_1<TControl>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::InputSystem::InputControlList_1<TControl>),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (list))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Current(&mut self) -> quest_hook::libil2cpp::Result<TControl>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), TControl, 0usize>("get_Current")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Current", 0usize
                )
            });
        let __cordl_ret: TControl = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1+Enumerator")]
impl<
    TControl: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IEnumerator_1<TControl>>
for crate::UnityEngine::InputSystem::InputControlList_1_Enumerator<TControl> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::IEnumerator_1<TControl> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1+Enumerator")]
impl<
    TControl: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IEnumerator_1<TControl>>
for crate::UnityEngine::InputSystem::InputControlList_1_Enumerator<TControl> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerator_1<TControl> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1+Enumerator")]
impl<
    TControl: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::IEnumerator>
for crate::UnityEngine::InputSystem::InputControlList_1_Enumerator<TControl> {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerator {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1+Enumerator")]
impl<
    TControl: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::IEnumerator>
for crate::UnityEngine::InputSystem::InputControlList_1_Enumerator<TControl> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerator {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1+Enumerator")]
impl<TControl: quest_hook::libil2cpp::Type> AsRef<crate::System::IDisposable>
for crate::UnityEngine::InputSystem::InputControlList_1_Enumerator<TControl> {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1+Enumerator")]
impl<TControl: quest_hook::libil2cpp::Type> AsMut<crate::System::IDisposable>
for crate::UnityEngine::InputSystem::InputControlList_1_Enumerator<TControl> {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
