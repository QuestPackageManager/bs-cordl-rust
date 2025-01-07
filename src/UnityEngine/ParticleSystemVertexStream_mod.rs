#[cfg(feature = "UnityEngine+ParticleSystemVertexStream")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ParticleSystemVertexStream {
    #[default]
    AgePercent = 21i32,
    AnimBlend = 8i32,
    AnimFrame = 9i32,
    Center = 10i32,
    Color = 3i32,
    ColorPackedAsTwoFloats = 47i32,
    Custom1X = 31i32,
    Custom1XY = 32i32,
    Custom1XYZ = 33i32,
    Custom1XYZW = 34i32,
    Custom2X = 35i32,
    Custom2XY = 36i32,
    Custom2XYZ = 37i32,
    Custom2XYZW = 38i32,
    InvStartLifetime = 22i32,
    MeshAxisOfRotation = 48i32,
    MeshIndex = 45i32,
    NextTrailCenter = 49i32,
    NoiseImpulseX = 42i32,
    NoiseImpulseXY = 43i32,
    NoiseImpulseXYZ = 44i32,
    NoiseSumX = 39i32,
    NoiseSumXY = 40i32,
    NoiseSumXYZ = 41i32,
    Normal = 1i32,
    ParticleIndex = 46i32,
    PercentageAlongTrail = 51i32,
    Position = 0i32,
    PreviousTrailCenter = 50i32,
    Rotation = 15i32,
    Rotation3D = 16i32,
    RotationSpeed = 17i32,
    RotationSpeed3D = 18i32,
    SizeX = 12i32,
    SizeXY = 13i32,
    SizeXYZ = 14i32,
    Speed = 20i32,
    StableRandomX = 23i32,
    StableRandomXY = 24i32,
    StableRandomXYZ = 25i32,
    StableRandomXYZW = 26i32,
    Tangent = 2i32,
    TrailWidth = 52i32,
    UV = 4i32,
    UV2 = 5i32,
    UV3 = 6i32,
    UV4 = 7i32,
    VaryingRandomX = 27i32,
    VaryingRandomXY = 28i32,
    VaryingRandomXYZ = 29i32,
    VaryingRandomXYZW = 30i32,
    Velocity = 19i32,
    VertexID = 11i32,
}
#[cfg(feature = "UnityEngine+ParticleSystemVertexStream")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ParticleSystemVertexStream {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "ParticleSystemVertexStream";
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
for crate::UnityEngine::ParticleSystemVertexStream {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::ParticleSystemVertexStream {
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
for crate::UnityEngine::ParticleSystemVertexStream {
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
for crate::UnityEngine::ParticleSystemVertexStream {
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
