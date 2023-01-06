#[cfg(not(feature = "safe"))]
macro_rules! get {
    ($slice:ident,$offset:expr) => {
        *unsafe { $slice.get_unchecked($offset) }
    };
}
#[cfg(not(feature = "safe"))]
macro_rules! set {
    ($slice:ident,$offset:expr,$value:expr) => {
        *unsafe { $slice.get_unchecked_mut($offset) } = $value as u8;
    };
}

#[cfg(feature = "safe")]
macro_rules! get {
    ($slice:ident,$offset:expr) => {
        $slice[$offset]
    };
}
#[cfg(feature = "safe")]
macro_rules! set {
    ($slice:ident,$offset:expr,$value:expr) => {
        $slice[$offset] = $value
    };
}

#[cfg(not(feature = "safe"))]
macro_rules! search_loop {
    ($s:ident, $r:ident, $buffer:ident) => {
        ($s..$r)
            .map(|i| (i, unsafe { $buffer.get_unchecked(i) }))
            .rev()
    };
}
#[cfg(feature = "safe")]
macro_rules! search_loop {
    ($s:ident, $r:ident, $buffer:ident) => {
        ($s..$r).zip(&$buffer[$s..$r]).rev()
    };
}

pub(crate) use get;
pub(crate) use search_loop;
pub(crate) use set;
