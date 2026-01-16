#[cfg(feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LayoutComputedData {
    pub Position: crate::UnityEngine::UIElements::Layout::LayoutComputedData__Position_e__FixedBuffer,
    pub Dimensions: crate::UnityEngine::UIElements::Layout::LayoutComputedData__Dimensions_e__FixedBuffer,
    pub Margin: crate::UnityEngine::UIElements::Layout::LayoutComputedData__Margin_e__FixedBuffer,
    pub Border: crate::UnityEngine::UIElements::Layout::LayoutComputedData__Border_e__FixedBuffer,
    pub Padding: crate::UnityEngine::UIElements::Layout::LayoutComputedData__Padding_e__FixedBuffer,
    pub Direction: crate::UnityEngine::UIElements::Layout::LayoutDirection,
    pub ComputedFlexBasisGeneration: u32,
    pub ComputedFlexBasis: f32,
    pub HadOverflow: bool,
    pub GenerationCount: u32,
    pub LastParentDirection: crate::UnityEngine::UIElements::Layout::LayoutDirection,
    pub LastPointScaleFactor: f32,
    pub MeasuredDimensions: crate::UnityEngine::UIElements::Layout::LayoutComputedData__MeasuredDimensions_e__FixedBuffer,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::Layout::LayoutComputedData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.Layout";
    const CLASS_NAME: &'static str = "LayoutComputedData";
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::Layout::LayoutComputedData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::Layout::LayoutComputedData {
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::Layout::LayoutComputedData {
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::Layout::LayoutComputedData {
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::Layout::LayoutComputedData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Layout+LayoutComputedData")]
impl crate::UnityEngine::UIElements::Layout::LayoutComputedData {
    #[cfg(
        feature = "UnityEngine+UIElements+Layout+LayoutComputedData+_Border_e__FixedBuffer"
    )]
    pub type _Border_e__FixedBuffer = crate::UnityEngine::UIElements::Layout::LayoutComputedData__Border_e__FixedBuffer;
    #[cfg(
        feature = "UnityEngine+UIElements+Layout+LayoutComputedData+_Dimensions_e__FixedBuffer"
    )]
    pub type _Dimensions_e__FixedBuffer = crate::UnityEngine::UIElements::Layout::LayoutComputedData__Dimensions_e__FixedBuffer;
    #[cfg(
        feature = "UnityEngine+UIElements+Layout+LayoutComputedData+_Margin_e__FixedBuffer"
    )]
    pub type _Margin_e__FixedBuffer = crate::UnityEngine::UIElements::Layout::LayoutComputedData__Margin_e__FixedBuffer;
    #[cfg(
        feature = "UnityEngine+UIElements+Layout+LayoutComputedData+_MeasuredDimensions_e__FixedBuffer"
    )]
    pub type _MeasuredDimensions_e__FixedBuffer = crate::UnityEngine::UIElements::Layout::LayoutComputedData__MeasuredDimensions_e__FixedBuffer;
    #[cfg(
        feature = "UnityEngine+UIElements+Layout+LayoutComputedData+_Padding_e__FixedBuffer"
    )]
    pub type _Padding_e__FixedBuffer = crate::UnityEngine::UIElements::Layout::LayoutComputedData__Padding_e__FixedBuffer;
    #[cfg(
        feature = "UnityEngine+UIElements+Layout+LayoutComputedData+_Position_e__FixedBuffer"
    )]
    pub type _Position_e__FixedBuffer = crate::UnityEngine::UIElements::Layout::LayoutComputedData__Position_e__FixedBuffer;
    pub fn get_BorderBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        0usize,
                    >("get_BorderBuffer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_BorderBuffer", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Default() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Layout::LayoutComputedData,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::UnityEngine::UIElements::Layout::LayoutComputedData,
                        0usize,
                    >("get_Default")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Default", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Layout::LayoutComputedData = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_MarginBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        0usize,
                    >("get_MarginBuffer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_MarginBuffer", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_PaddingBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        0usize,
                    >("get_PaddingBuffer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_PaddingBuffer", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_Border_e__FixedBuffer"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LayoutComputedData__Border_e__FixedBuffer {
    pub FixedElementField: f32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_Border_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::Layout::LayoutComputedData__Border_e__FixedBuffer {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.Layout";
    const CLASS_NAME: &'static str = "LayoutComputedData/<Border>e__FixedBuffer";
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
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_Border_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::Layout::LayoutComputedData__Border_e__FixedBuffer {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_Border_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::Layout::LayoutComputedData__Border_e__FixedBuffer {
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
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_Border_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::Layout::LayoutComputedData__Border_e__FixedBuffer {
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
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_Border_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::Layout::LayoutComputedData__Border_e__FixedBuffer {
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
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_Border_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::Layout::LayoutComputedData__Border_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+Layout+LayoutComputedData+_Border_e__FixedBuffer"
)]
impl crate::UnityEngine::UIElements::Layout::LayoutComputedData__Border_e__FixedBuffer {}
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_Dimensions_e__FixedBuffer"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LayoutComputedData__Dimensions_e__FixedBuffer {
    pub FixedElementField: f32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_Dimensions_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::Layout::LayoutComputedData__Dimensions_e__FixedBuffer {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.Layout";
    const CLASS_NAME: &'static str = "LayoutComputedData/<Dimensions>e__FixedBuffer";
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
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_Dimensions_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::Layout::LayoutComputedData__Dimensions_e__FixedBuffer {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_Dimensions_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::Layout::LayoutComputedData__Dimensions_e__FixedBuffer {
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
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_Dimensions_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::Layout::LayoutComputedData__Dimensions_e__FixedBuffer {
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
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_Dimensions_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::Layout::LayoutComputedData__Dimensions_e__FixedBuffer {
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
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_Dimensions_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::Layout::LayoutComputedData__Dimensions_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+Layout+LayoutComputedData+_Dimensions_e__FixedBuffer"
)]
impl crate::UnityEngine::UIElements::Layout::LayoutComputedData__Dimensions_e__FixedBuffer {}
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_Margin_e__FixedBuffer"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LayoutComputedData__Margin_e__FixedBuffer {
    pub FixedElementField: f32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_Margin_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::Layout::LayoutComputedData__Margin_e__FixedBuffer {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.Layout";
    const CLASS_NAME: &'static str = "LayoutComputedData/<Margin>e__FixedBuffer";
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
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_Margin_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::Layout::LayoutComputedData__Margin_e__FixedBuffer {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_Margin_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::Layout::LayoutComputedData__Margin_e__FixedBuffer {
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
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_Margin_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::Layout::LayoutComputedData__Margin_e__FixedBuffer {
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
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_Margin_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::Layout::LayoutComputedData__Margin_e__FixedBuffer {
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
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_Margin_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::Layout::LayoutComputedData__Margin_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+Layout+LayoutComputedData+_Margin_e__FixedBuffer"
)]
impl crate::UnityEngine::UIElements::Layout::LayoutComputedData__Margin_e__FixedBuffer {}
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_MeasuredDimensions_e__FixedBuffer"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LayoutComputedData__MeasuredDimensions_e__FixedBuffer {
    pub FixedElementField: f32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_MeasuredDimensions_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::Layout::LayoutComputedData__MeasuredDimensions_e__FixedBuffer {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.Layout";
    const CLASS_NAME: &'static str = "LayoutComputedData/<MeasuredDimensions>e__FixedBuffer";
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
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_MeasuredDimensions_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::Layout::LayoutComputedData__MeasuredDimensions_e__FixedBuffer {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_MeasuredDimensions_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::Layout::LayoutComputedData__MeasuredDimensions_e__FixedBuffer {
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
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_MeasuredDimensions_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::Layout::LayoutComputedData__MeasuredDimensions_e__FixedBuffer {
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
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_MeasuredDimensions_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::Layout::LayoutComputedData__MeasuredDimensions_e__FixedBuffer {
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
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_MeasuredDimensions_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::Layout::LayoutComputedData__MeasuredDimensions_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+Layout+LayoutComputedData+_MeasuredDimensions_e__FixedBuffer"
)]
impl crate::UnityEngine::UIElements::Layout::LayoutComputedData__MeasuredDimensions_e__FixedBuffer {}
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_Padding_e__FixedBuffer"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LayoutComputedData__Padding_e__FixedBuffer {
    pub FixedElementField: f32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_Padding_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::Layout::LayoutComputedData__Padding_e__FixedBuffer {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.Layout";
    const CLASS_NAME: &'static str = "LayoutComputedData/<Padding>e__FixedBuffer";
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
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_Padding_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::Layout::LayoutComputedData__Padding_e__FixedBuffer {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_Padding_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::Layout::LayoutComputedData__Padding_e__FixedBuffer {
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
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_Padding_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::Layout::LayoutComputedData__Padding_e__FixedBuffer {
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
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_Padding_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::Layout::LayoutComputedData__Padding_e__FixedBuffer {
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
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_Padding_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::Layout::LayoutComputedData__Padding_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+Layout+LayoutComputedData+_Padding_e__FixedBuffer"
)]
impl crate::UnityEngine::UIElements::Layout::LayoutComputedData__Padding_e__FixedBuffer {}
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_Position_e__FixedBuffer"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LayoutComputedData__Position_e__FixedBuffer {
    pub FixedElementField: f32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_Position_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::Layout::LayoutComputedData__Position_e__FixedBuffer {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.Layout";
    const CLASS_NAME: &'static str = "LayoutComputedData/<Position>e__FixedBuffer";
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
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_Position_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::Layout::LayoutComputedData__Position_e__FixedBuffer {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_Position_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::Layout::LayoutComputedData__Position_e__FixedBuffer {
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
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_Position_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::Layout::LayoutComputedData__Position_e__FixedBuffer {
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
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_Position_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::Layout::LayoutComputedData__Position_e__FixedBuffer {
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
    feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutComputedData+_Position_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::Layout::LayoutComputedData__Position_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+Layout+LayoutComputedData+_Position_e__FixedBuffer"
)]
impl crate::UnityEngine::UIElements::Layout::LayoutComputedData__Position_e__FixedBuffer {}
