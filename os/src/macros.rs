#[macro_export]
macro_rules! cfg_if {
    // match if/else chains with a final `else`
    ($(
        if #[cfg($($meta:meta),*)] { $($it:item)* }
    ) else * else {
        $($it2:item)*
    }) => {
        cfg_if! {
            @__items
            () ;
            $( ( ($($meta),*) ($($it)*) ), )*
            ( () ($($it2)*) ),
        }
    };

    // match if/else chains lacking a final `else`
    (
        if #[cfg($($i_met:meta),*)] { $($i_it:item)* }
        $(
            else if #[cfg($($e_met:meta),*)] { $($e_it:item)* }
        )*
    ) => {
        cfg_if! {
            @__items
            () ;
            ( ($($i_met),*) ($($i_it)*) ),
            $( ( ($($e_met),*) ($($e_it)*) ), )*
            ( () () ),
        }
    };

    // Internal and recursive macro to emit all the items
    //
    // Collects all the negated `cfg`s in a list at the beginning and after the
    // semicolon is all the remaining items
    (@__items ($($not:meta,)*) ; ) => {};
    (@__items ($($not:meta,)*) ; ( ($($m:meta),*) ($($it:item)*) ),
     $($rest:tt)*) => {
        // Emit all items within one block, applying an appropriate #[cfg]. The
        // #[cfg] will require all `$m` matchers specified and must also negate
        // all previous matchers.
        cfg_if! { @__apply cfg(all($($m,)* not(any($($not),*)))), $($it)* }

        // Recurse to emit all other items in `$rest`, and when we do so add all
        // our `$m` matchers to the list of `$not` matchers as future emissions
        // will have to negate everything we just matched as well.
        cfg_if! { @__items ($($not,)* $($m,)*) ; $($rest)* }
    };

    // Internal macro to Apply a cfg attribute to a list of items
    (@__apply $m:meta, $($it:item)*) => {
        $(#[$m] $it)*
    };
}

#[macro_export]
macro_rules! s {
    ($($(#[$attr:meta])* pub $t:ident $i:ident { $($field:tt)* })*) => ($(
        s!(it: $(#[$attr])* pub $t $i { $($field)* });
    )*);
    (it: $(#[$attr:meta])* pub union $i:ident { $($field:tt)* }) => (
        compile_error!("unions cannot derive extra traits, use s_no_extra_traits instead");
    );
    (it: $(#[$attr:meta])* pub struct $i:ident { $($field:tt)* }) => (
        __item! {
            #[repr(C)]
            #[cfg_attr(feature = "extra_traits", derive(Debug, Eq, Hash, PartialEq))]
            #[allow(deprecated)]
            $(#[$attr])*
            pub struct $i { $($field)* }
        }
        #[allow(deprecated)]
        impl ::Copy for $i {}
        #[allow(deprecated)]
        impl ::Clone for $i {
            fn clone(&self) -> $i { *self }
        }
    );
}

#[macro_export]
macro_rules! s_no_extra_traits {
    ($($(#[$attr:meta])* pub $t:ident $i:ident { $($field:tt)* })*) => ($(
        s_no_extra_traits!(it: $(#[$attr])* pub $t $i { $($field)* });
    )*);
    (it: $(#[$attr:meta])* pub union $i:ident { $($field:tt)* }) => (
        __item! {
            #[repr(C)]
            $(#[$attr])*
            pub union $i { $($field)* }
        }

        impl ::Copy for $i {}
        impl ::Clone for $i {
            fn clone(&self) -> $i { *self }
        }
    );
    (it: $(#[$attr:meta])* pub struct $i:ident { $($field:tt)* }) => (
        __item! {
            #[repr(C)]
            $(#[$attr])*
            pub struct $i { $($field)* }
        }
        #[allow(deprecated)]
        impl ::Copy for $i {}
        #[allow(deprecated)]
        impl ::Clone for $i {
            fn clone(&self) -> $i { *self }
        }
    );
}

#[macro_export]
macro_rules! missing {
    ($($(#[$attr:meta])* pub enum $i:ident {})*) => ($(
        $(#[$attr])* #[allow(missing_copy_implementations)] pub enum $i { }
    )*);
}

#[macro_export]
macro_rules! e {
    ($($(#[$attr:meta])* pub enum $i:ident { $($field:tt)* })*) => ($(
        __item! {
            #[cfg_attr(feature = "extra_traits", derive(Debug, Eq, Hash, PartialEq))]
            $(#[$attr])*
            pub enum $i { $($field)* }
        }
        impl ::Copy for $i {}
        impl ::Clone for $i {
            fn clone(&self) -> $i { *self }
        }
    )*);
}

#[macro_export]
macro_rules! s_paren {
    ($($(#[$attr:meta])* pub struct $i:ident ( $($field:tt)* ); )* ) => ($(
        __item! {
            #[cfg_attr(feature = "extra_traits", derive(Debug, Eq, Hash, PartialEq))]
            $(#[$attr])*
            pub struct $i ( $($field)* );
        }
        impl ::Copy for $i {}
        impl ::Clone for $i {
            fn clone(&self) -> $i { *self }
        }
    )*);
}

macro_rules! __item {
    ($i:item) => {
        $i
    };
}

macro_rules! ptr_addr_of {
    ($place:expr) => {
        ::core::ptr::addr_of!($place)
    };
}
