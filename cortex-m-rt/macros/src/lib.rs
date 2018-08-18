#![deny(warnings)]

extern crate proc_macro;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use syn::synom::Synom;
use syn::token::{Colon, Comma, Eq, Static};
use syn::{Expr, FnArg, Ident, ItemFn, ReturnType, Type, Visibility};

use proc_macro::TokenStream;

/// Attribute to declare the entry point of the program
///
/// **NOTE** This macro must be invoked once and must be invoked from an accessible module, ideally
/// from the root of the crate.
///
/// The specified function will be called by the reset handler *after* RAM has been initialized. In
/// the case of the `thumbv7em-none-eabihf` target the FPU will also be enabled before the function
/// is called.
///
/// The type of the specified function must be `fn() -> !` (never ending function)
///
/// # Examples
///
/// ``` no_run
/// # #![no_main]
/// # use cortex_m_rt_macros::entry;
/// #[entry]
/// fn main() -> ! {
///     loop {
///         /* .. */
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn entry(args: TokenStream, input: TokenStream) -> TokenStream {
    let f: ItemFn = syn::parse(input).expect("`#[entry]` must be applied to a function");

    // check the function signature
    assert!(
        f.constness.is_none()
            && f.vis == Visibility::Inherited
            && f.unsafety.is_none()
            && f.abi.is_none()
            && f.decl.inputs.is_empty()
            && f.decl.generics.params.is_empty()
            && f.decl.generics.where_clause.is_none()
            && f.decl.variadic.is_none()
            && match f.decl.output {
                ReturnType::Default => false,
                ReturnType::Type(_, ref ty) => match **ty {
                    Type::Never(_) => true,
                    _ => false,
                },
            },
        "`#[entry]` function must have signature `fn() -> !`"
    );

    assert_eq!(
        args.to_string(),
        "",
        "`entry` attribute must have no arguments"
    );

    // XXX should we blacklist other attributes?
    let attrs = f.attrs;
    let ident = f.ident;
    let block = f.block;

    quote!(
        #[export_name = "main"]
        #(#attrs)*
        pub fn #ident() -> ! #block
    ).into()
}

struct ExceptionArgs {
    first: Ident,
    second: Option<State>,
}

impl Synom for ExceptionArgs {
    named!(parse -> Self, do_parse!(
        first: syn!(Ident) >>
            second: option!(syn!(State)) >> (
                ExceptionArgs { first, second }
            )
    ));
}

struct State {
    _comma: Comma,
    _static: Static,
    ident: Ident,
    _colon: Colon,
    ty: Type,
    _eq: Eq,
    expr: Expr,
}

impl Synom for State {
    named!(parse -> Self, do_parse!(
        _comma: punct!(,) >>
            _static: syn!(Static) >>
            ident: syn!(Ident) >>
            _colon: punct!(:) >>
            ty: syn!(Type) >>
            _eq: punct!(=) >>
            expr: syn!(Expr) >> (
                State { _comma, _static, ident, _colon, ty, _eq, expr }
            )
    ));
}

/// Attribute to declare an exception handler
///
/// **NOTE** This macro must be invoked from an accessible module, ideally from the root of the
/// crate.
///
/// # Syntax
///
/// ```
/// # use cortex_m_rt_macros::exception;
/// #[exception(SysTick, static COUNT: u32 = 0)]
/// fn handler() {
///     // ..
/// }
///
/// # fn main() {}
/// ```
///
/// where the first argument can be one of:
///
/// - `DefaultHandler`
/// - `NonMaskableInt`
/// - `HardFault`
/// - `MemoryManagement` (a)
/// - `BusFault` (a)
/// - `UsageFault` (a)
/// - `SecureFault` (b)
/// - `SVCall`
/// - `DebugMonitor` (a)
/// - `PendSV`
/// - `SysTick`
///
/// and the second is optional.
///
/// (a) Not available on Cortex-M0 variants (`thumbv6m-none-eabi`)
///
/// (b) Only available on ARMv8-M
///
/// # Usage
///
/// `#[exception(HardFault)]` sets the hard fault handler. The handler must have signature
/// `fn(&ExceptionFrame) -> !`. This handler is not allowed to return as that can cause undefined
/// behavior.
///
/// `#[exception(DefaultHandler)]` sets the *default* handler. All exceptions which have not been
/// assigned a handler will be serviced by this handler. This handler must have signature `fn(irqn:
/// i16)`. `irqn` is the IRQ number (See CMSIS); `irqn` will be a negative number when the handler
/// is servicing a core exception; `irqn` will be a positive number when the handler is servicing a
/// device specific exception (interrupt).
///
/// `#[exception(Name)]` overrides the default handler for the exception with the given `Name`.
///
/// # Examples
///
/// - Setting the `HardFault` handler
///
/// ```
/// # extern crate cortex_m_rt;
/// # extern crate cortex_m_rt_macros;
/// # use cortex_m_rt_macros::exception;
/// #[exception(HardFault)]
/// fn hard_fault(ef: &cortex_m_rt::ExceptionFrame) -> ! {
///     // prints the exception frame as a panic message
///     panic!("{:#?}", ef);
/// }
///
/// # fn main() {}
/// ```
///
/// - Setting the default handler
///
/// ```
/// # use cortex_m_rt_macros::exception;
/// #[exception(DefaultHandler)]
/// fn default_handler(irqn: i16) {
///     println!("IRQn = {}", irqn);
/// }
///
/// # fn main() {}
/// ```
///
/// - Overriding the `SysTick` handler
///
/// ```
/// extern crate cortex_m_rt as rt;
///
/// use rt::exception;
///
/// #[exception(SysTick, static COUNT: i32 = 0)]
/// fn sys_tick() {
///     *COUNT += 1;
///
///     println!("{}", COUNT);
/// }
///
/// # fn main() {}
/// ```
#[proc_macro_attribute]
pub fn exception(args: TokenStream, input: TokenStream) -> TokenStream {
    let f: ItemFn = syn::parse(input).expect("`#[exception]` must be applied to a function");
    let args: ExceptionArgs = syn::parse(args).expect(
        "`exception` attribute expects the exception name as its argument. \
         e.g. `#[exception(HardFault)]`",
    );
    let name = args.first;
    let name_s = name.to_string();

    enum Exception {
        DefaultHandler,
        HardFault,
        Other,
    }

    // first validation of the exception name
    let exn = match &*name_s {
        "DefaultHandler" => Exception::DefaultHandler,
        "HardFault" => Exception::HardFault,
        // NOTE that at this point we don't check if the exception is available on the target (e.g.
        // MemoryManagement is not available on Cortex-M0)
        "NonMaskableInt" | "MemoryManagement" | "BusFault" | "UsageFault" | "SecureFault"
        | "SVCall" | "DebugMonitor" | "PendSV" | "SysTick" => Exception::Other,
        _ => panic!("{} is not a valid exception name", name_s),
    };

    // XXX should we blacklist other attributes?
    let attrs = f.attrs;
    let ident = f.ident;
    let block = f.block;
    let stmts = &block.stmts;

    match exn {
        Exception::DefaultHandler => {
            assert!(
                f.constness.is_none()
                    && f.vis == Visibility::Inherited
                    && f.unsafety.is_none()
                    && f.abi.is_none()
                    && f.decl.inputs.len() == 1
                    && f.decl.generics.params.is_empty()
                    && f.decl.generics.where_clause.is_none()
                    && f.decl.variadic.is_none()
                    && match f.decl.output {
                        ReturnType::Default => true,
                        ReturnType::Type(_, ref ty) => match **ty {
                            Type::Tuple(ref tuple) => tuple.elems.is_empty(),
                            _ => false,
                        },
                    },
                "`#[exception(DefaultHandler)]` function must have signature `fn(i16)`"
            );

            assert!(
                args.second.is_none(),
                "`#[exception(DefaultHandler)]` takes no additional arguments"
            );

            let arg = match f.decl.inputs[0] {
                FnArg::Captured(ref arg) => arg,
                _ => unreachable!(),
            };

            quote!(
                #[export_name = #name_s]
                #(#attrs)*
                pub fn #ident() {
                    extern crate core;

                    const SCB_ICSR: *const u32 = 0xE000_ED04 as *const u32;

                    let #arg = unsafe { core::ptr::read(SCB_ICSR) as u8 as i16 - 16 };

                    #(#stmts)*
                }
            ).into()
        }
        Exception::HardFault => {
            assert!(
                f.constness.is_none()
                    && f.vis == Visibility::Inherited
                    && f.unsafety.is_none()
                    && f.abi.is_none()
                    && f.decl.inputs.len() == 1
                    && match f.decl.inputs[0] {
                        FnArg::Captured(ref arg) => match arg.ty {
                            Type::Reference(ref r) => {
                                r.lifetime.is_none() && r.mutability.is_none()
                            }
                            _ => false,
                        },
                        _ => false,
                    }
                    && f.decl.generics.params.is_empty()
                    && f.decl.generics.where_clause.is_none()
                    && f.decl.variadic.is_none()
                    && match f.decl.output {
                        ReturnType::Default => false,
                        ReturnType::Type(_, ref ty) => match **ty {
                            Type::Never(_) => true,
                            _ => false,
                        },
                    },
                "`#[exception(HardFault)]` function must have signature `fn(&ExceptionFrame) -> !`"
            );

            assert!(
                args.second.is_none(),
                "`#[exception(HardFault)]` takes no additional arguments"
            );

            let arg = match f.decl.inputs[0] {
                FnArg::Captured(ref arg) => arg,
                _ => unreachable!(),
            };

            let pat = &arg.pat;

            quote!(
                #[export_name = "UserHardFault"]
                #(#attrs)*
                pub unsafe extern "C" fn #ident(#arg) -> ! {
                    extern crate cortex_m_rt;

                    // further type check of the input argument
                    let #pat: &cortex_m_rt::ExceptionFrame = #pat;

                    #(#stmts)*
                }
            ).into()
        }
        Exception::Other => {
            assert!(
                f.constness.is_none()
                    && f.vis == Visibility::Inherited
                    && f.unsafety.is_none()
                    && f.abi.is_none()
                    && f.decl.inputs.is_empty()
                    && f.decl.generics.params.is_empty()
                    && f.decl.generics.where_clause.is_none()
                    && f.decl.variadic.is_none()
                    && match f.decl.output {
                        ReturnType::Default => true,
                        ReturnType::Type(_, ref ty) => match **ty {
                            Type::Tuple(ref tuple) => tuple.elems.is_empty(),
                            _ => false,
                        },
                    },
                "`#[exception]` functions must have signature `fn()`"
            );

            if let Some(second) = args.second {
                let ty = second.ty;
                let expr = second.expr;
                let state = second.ident;

                quote!(
                    #[export_name = #name_s]
                    #(#attrs)*
                    pub fn #ident() {
                        extern crate cortex_m_rt;

                        cortex_m_rt::Exception::#name;

                        static mut __STATE__: #ty = #expr;

                        #[allow(non_snake_case)]
                        let #state: &mut #ty = unsafe { &mut __STATE__ };

                        #(#stmts)*
                    }
                ).into()
            } else {
                quote!(
                    #[export_name = #name_s]
                    #(#attrs)*
                    pub fn #ident() {
                        extern crate cortex_m_rt;

                        cortex_m_rt::Exception::#name;

                        #(#stmts)*
                    }
                ).into()
            }
        }
    }
}

/// Attribute to mark which function will be called at the beginning of the reset handler.
///
/// The function must have the signature of `unsafe fn()`.
///
/// The function passed will be called before static variables are initialized. Any access of static
/// variables will result in undefined behavior.
///
/// # Examples
///
/// ```
/// # use cortex_m_rt_macros::pre_init;
/// #[pre_init]
/// unsafe fn before_main() {
///     // do something here
/// }
///
/// # fn main() {}
/// ```
#[proc_macro_attribute]
pub fn pre_init(args: TokenStream, input: TokenStream) -> TokenStream {
    let f: ItemFn = syn::parse(input).expect("`#[pre_init]` must be applied to a function");

    // check the function signature
    assert!(
        f.constness.is_none()
            && f.vis == Visibility::Inherited
            && f.unsafety.is_some()
            && f.abi.is_none()
            && f.decl.inputs.is_empty()
            && f.decl.generics.params.is_empty()
            && f.decl.generics.where_clause.is_none()
            && f.decl.variadic.is_none()
            && match f.decl.output {
                ReturnType::Default => true,
                ReturnType::Type(_, ref ty) => match **ty {
                    Type::Tuple(ref tuple) => tuple.elems.is_empty(),
                    _ => false,
                },
            },
        "`#[pre_init]` function must have signature `unsafe fn()`"
    );

    assert_eq!(
        args.to_string(),
        "",
        "`pre_init` attribute must have no arguments"
    );

    // XXX should we blacklist other attributes?
    let attrs = f.attrs;
    let ident = f.ident;
    let block = f.block;

    quote!(
        #[export_name = "__pre_init"]
        #(#attrs)*
        pub unsafe fn #ident() #block
    ).into()
}
