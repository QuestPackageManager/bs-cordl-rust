#[cfg(feature = "System+Data+FunctionId")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FunctionId {
    #[default]
    Abs = 26i32,
    Acos = 27i32,
    Ascii = 0i32,
    Avg = 31i32,
    Char = 1i32,
    Charindex = 2i32,
    Convert = 20i32,
    Count = 34i32,
    DateTimeOffset = 38i32,
    Difference = 3i32,
    Iif = 19i32,
    In = 28i32,
    IsNull = 18i32,
    LTrim = 6i32,
    Len = 4i32,
    Lower = 5i32,
    Max = 33i32,
    Min = 32i32,
    Patindex = 7i32,
    RTrim = 11i32,
    Replicate = 8i32,
    Reverse = 9i32,
    Right = 10i32,
    Soundex = 12i32,
    Space = 13i32,
    StDev = 35i32,
    Str = 14i32,
    Stuff = 15i32,
    Substring = 16i32,
    Sum = 30i32,
    Trim = 29i32,
    Upper = 17i32,
    Var = 37i32,
    cBool = 22i32,
    cDate = 23i32,
    cDbl = 24i32,
    cInt = 21i32,
    cStr = 25i32,
    none = -1i32,
}
#[cfg(feature = "System+Data+FunctionId")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Data::FunctionId => "System.Data"
    ."FunctionId"
);
