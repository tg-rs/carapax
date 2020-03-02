use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream},
    FnArg, Ident, ItemFn, Result as SynResult, ReturnType, Type, Visibility,
};

pub(crate) struct HandlerMeta {
    pub(super) vis: Visibility,
    pub(super) ident: Ident,
    pub(super) ident_inner: Ident,
    pub(super) handler: ItemFn,
    pub(super) context: Box<Type>,
    pub(super) input: Box<Type>,
    pub(super) output: Option<Box<Type>>,
}

impl Parse for HandlerMeta {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let mut handler = input.parse::<ItemFn>()?;
        if handler.sig.asyncness.is_none() {
            return Err(input.error("function must be async"));
        }
        let context = match handler.sig.inputs.first() {
            Some(FnArg::Typed(typed)) => match &*typed.ty {
                Type::Reference(reference) => reference.elem.clone(),
                _ => return Err(input.error("context must be a reference")),
            },
            _ => return Err(input.error("first argument must be context type")),
        };
        let input = match handler.sig.inputs.last() {
            Some(FnArg::Typed(arg)) => arg.ty.clone(),
            _ => return Err(input.error("unable to detect handler input")),
        };
        let output = match &handler.sig.output {
            ReturnType::Default => None,
            ReturnType::Type(_, output) => Some(output.clone()),
        };
        let vis = handler.vis.clone();
        let ident = handler.sig.ident.clone();
        let ident_inner = Ident::new(&format!("__carapax_{}", ident), Span::call_site());
        handler.sig.ident = ident_inner.clone();
        Ok(HandlerMeta {
            vis,
            handler,
            ident,
            ident_inner,
            context,
            input,
            output,
        })
    }
}
