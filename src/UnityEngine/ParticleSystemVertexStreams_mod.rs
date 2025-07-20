#[cfg(feature = "UnityEngine+ParticleSystemVertexStreams")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ParticleSystemVertexStreams {
    #[default]
    All = 2147483647i32,
    CenterAndVertexID = 64i32,
    Color = 8i32,
    Custom1 = 2048i32,
    Custom2 = 4096i32,
    Lifetime = 1024i32,
    None = 0i32,
    Normal = 2i32,
    Position = 1i32,
    Random = 8192i32,
    Rotation = 256i32,
    Size = 128i32,
    Tangent = 4i32,
    UV = 16i32,
    UV2BlendAndFrame = 32i32,
    Velocity = 512i32,
}
#[cfg(feature = "UnityEngine+ParticleSystemVertexStreams")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ParticleSystemVertexStreams {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "ParticleSystemVertexStreams";
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
#[cfg(feature = "UnityEngine+ParticleSystemVertexStreams")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::ParticleSystemVertexStreams {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+ParticleSystemVertexStreams")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::ParticleSystemVertexStreams {
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
#[cfg(feature = "UnityEngine+ParticleSystemVertexStreams")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::ParticleSystemVertexStreams {
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
#[cfg(feature = "UnityEngine+ParticleSystemVertexStreams")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::ParticleSystemVertexStreams {
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
