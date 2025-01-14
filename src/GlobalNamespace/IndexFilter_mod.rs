#[cfg(feature = "IndexFilter")]
#[repr(C)]
#[derive(Debug)]
pub struct IndexFilter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _random: crate::GlobalNamespace::IndexFilter_IndexFilterRandomType,
    pub _seed: i32,
    pub _groupSize: i32,
    pub _chunkSize: i32,
    pub _visibleCount: i32,
    pub _limitAlsoAffectType: crate::GlobalNamespace::IndexFilter_IndexFilterLimitAlsoAffectType,
    pub _start: i32,
    pub _step: i32,
    pub _count: i32,
}
#[cfg(feature = "IndexFilter")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::IndexFilter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "IndexFilter";
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
#[cfg(feature = "IndexFilter")]
impl std::ops::Deref for crate::GlobalNamespace::IndexFilter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IndexFilter")]
impl std::ops::DerefMut for crate::GlobalNamespace::IndexFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IndexFilter")]
impl crate::GlobalNamespace::IndexFilter {
    #[cfg(feature = "IndexFilter+IndexFilterLimitAlsoAffectType")]
    pub type IndexFilterLimitAlsoAffectType = crate::GlobalNamespace::IndexFilter_IndexFilterLimitAlsoAffectType;
    #[cfg(feature = "IndexFilter+IndexFilterRandomType")]
    pub type IndexFilterRandomType = crate::GlobalNamespace::IndexFilter_IndexFilterRandomType;
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                crate::System::ValueTuple_3<i32, i32, i32>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IEnumerator_1<
                        crate::System::ValueTuple_3<i32, i32, i32>,
                    >,
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
            crate::System::Collections::Generic::IEnumerator_1<
                crate::System::ValueTuple_3<i32, i32, i32>,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn GetValues(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IEnumerable_1<i32>,
                >,
                0usize,
            >("GetValues")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetValues", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn New_IndexFilter_IndexFilterRandomType_i32_f32_IndexFilter_IndexFilterLimitAlsoAffectType1(
        start: i32,
        end: i32,
        groupSize: i32,
        random: crate::GlobalNamespace::IndexFilter_IndexFilterRandomType,
        seed: i32,
        chunkSize: i32,
        limit: f32,
        limitAlsoAffectType: crate::GlobalNamespace::IndexFilter_IndexFilterLimitAlsoAffectType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    start,
                    end,
                    groupSize,
                    random,
                    seed,
                    chunkSize,
                    limit,
                    limitAlsoAffectType,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_IndexFilter_IndexFilterRandomType_i32_f32_IndexFilter_IndexFilterLimitAlsoAffectType0(
        start: i32,
        step: i32,
        count: i32,
        groupSize: i32,
        random: crate::GlobalNamespace::IndexFilter_IndexFilterRandomType,
        seed: i32,
        chunkSize: i32,
        limit: f32,
        limitAlsoAffectType: crate::GlobalNamespace::IndexFilter_IndexFilterLimitAlsoAffectType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    start,
                    step,
                    count,
                    groupSize,
                    random,
                    seed,
                    chunkSize,
                    limit,
                    limitAlsoAffectType,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
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
    pub fn _ctor_IndexFilter_IndexFilterRandomType_i32_f32_IndexFilter_IndexFilterLimitAlsoAffectType1(
        &mut self,
        start: i32,
        end: i32,
        groupSize: i32,
        random: crate::GlobalNamespace::IndexFilter_IndexFilterRandomType,
        seed: i32,
        chunkSize: i32,
        limit: f32,
        limitAlsoAffectType: crate::GlobalNamespace::IndexFilter_IndexFilterLimitAlsoAffectType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    i32,
                    i32,
                    crate::GlobalNamespace::IndexFilter_IndexFilterRandomType,
                    i32,
                    i32,
                    f32,
                    crate::GlobalNamespace::IndexFilter_IndexFilterLimitAlsoAffectType,
                ),
                quest_hook::libil2cpp::Void,
                8usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 8usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        start,
                        end,
                        groupSize,
                        random,
                        seed,
                        chunkSize,
                        limit,
                        limitAlsoAffectType,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_IndexFilter_IndexFilterRandomType_i32_f32_IndexFilter_IndexFilterLimitAlsoAffectType0(
        &mut self,
        start: i32,
        step: i32,
        count: i32,
        groupSize: i32,
        random: crate::GlobalNamespace::IndexFilter_IndexFilterRandomType,
        seed: i32,
        chunkSize: i32,
        limit: f32,
        limitAlsoAffectType: crate::GlobalNamespace::IndexFilter_IndexFilterLimitAlsoAffectType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    i32,
                    i32,
                    i32,
                    crate::GlobalNamespace::IndexFilter_IndexFilterRandomType,
                    i32,
                    i32,
                    f32,
                    crate::GlobalNamespace::IndexFilter_IndexFilterLimitAlsoAffectType,
                ),
                quest_hook::libil2cpp::Void,
                9usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 9usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        start,
                        step,
                        count,
                        groupSize,
                        random,
                        seed,
                        chunkSize,
                        limit,
                        limitAlsoAffectType,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
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
    pub fn get_VisibleCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_VisibleCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_VisibleCount", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_limitAlsoAffectType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::IndexFilter_IndexFilterLimitAlsoAffectType,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::GlobalNamespace::IndexFilter_IndexFilterLimitAlsoAffectType,
                0usize,
            >("get_limitAlsoAffectType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_limitAlsoAffectType", 0usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::IndexFilter_IndexFilterLimitAlsoAffectType = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_limitsDistribution(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_limitsDistribution")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_limitsDistribution", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_limitsDuration(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_limitsDuration")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_limitsDuration", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "IndexFilter")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::IndexFilter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "IndexFilter")]
impl AsRef<
    crate::System::Collections::Generic::IEnumerable_1<
        crate::System::ValueTuple_3<i32, i32, i32>,
    >,
> for crate::GlobalNamespace::IndexFilter {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEnumerable_1<
        crate::System::ValueTuple_3<i32, i32, i32>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IndexFilter")]
impl AsMut<
    crate::System::Collections::Generic::IEnumerable_1<
        crate::System::ValueTuple_3<i32, i32, i32>,
    >,
> for crate::GlobalNamespace::IndexFilter {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerable_1<
        crate::System::ValueTuple_3<i32, i32, i32>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IndexFilter")]
impl AsRef<
    crate::System::Collections::Generic::IReadOnlyCollection_1<
        crate::System::ValueTuple_3<i32, i32, i32>,
    >,
> for crate::GlobalNamespace::IndexFilter {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IReadOnlyCollection_1<
        crate::System::ValueTuple_3<i32, i32, i32>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IndexFilter")]
impl AsMut<
    crate::System::Collections::Generic::IReadOnlyCollection_1<
        crate::System::ValueTuple_3<i32, i32, i32>,
    >,
> for crate::GlobalNamespace::IndexFilter {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IReadOnlyCollection_1<
        crate::System::ValueTuple_3<i32, i32, i32>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IndexFilter")]
impl AsRef<crate::System::Collections::IEnumerable>
for crate::GlobalNamespace::IndexFilter {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IndexFilter")]
impl AsMut<crate::System::Collections::IEnumerable>
for crate::GlobalNamespace::IndexFilter {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IndexFilter+IndexFilterLimitAlsoAffectType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IndexFilter_IndexFilterLimitAlsoAffectType {
    #[default]
    Distribution = 2i32,
    Duration = 1i32,
    None = 0i32,
}
#[cfg(feature = "IndexFilter+IndexFilterLimitAlsoAffectType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::IndexFilter_IndexFilterLimitAlsoAffectType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "IndexFilter/IndexFilterLimitAlsoAffectType";
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
#[cfg(feature = "IndexFilter+IndexFilterLimitAlsoAffectType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::IndexFilter_IndexFilterLimitAlsoAffectType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "IndexFilter+IndexFilterLimitAlsoAffectType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::IndexFilter_IndexFilterLimitAlsoAffectType {
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
#[cfg(feature = "IndexFilter+IndexFilterLimitAlsoAffectType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::IndexFilter_IndexFilterLimitAlsoAffectType {
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
#[cfg(feature = "IndexFilter+IndexFilterLimitAlsoAffectType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::IndexFilter_IndexFilterLimitAlsoAffectType {
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
#[cfg(feature = "IndexFilter+IndexFilterRandomType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IndexFilter_IndexFilterRandomType {
    #[default]
    KeepOrder = 1i32,
    NoRandom = 0i32,
    RandomElements = 2i32,
}
#[cfg(feature = "IndexFilter+IndexFilterRandomType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::IndexFilter_IndexFilterRandomType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "IndexFilter/IndexFilterRandomType";
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
#[cfg(feature = "IndexFilter+IndexFilterRandomType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::IndexFilter_IndexFilterRandomType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "IndexFilter+IndexFilterRandomType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::IndexFilter_IndexFilterRandomType {
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
#[cfg(feature = "IndexFilter+IndexFilterRandomType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::IndexFilter_IndexFilterRandomType {
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
#[cfg(feature = "IndexFilter+IndexFilterRandomType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::IndexFilter_IndexFilterRandomType {
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
