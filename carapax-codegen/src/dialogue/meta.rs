use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream},
    FnArg, GenericArgument, Ident, ItemFn, Path, PathArguments, PathSegment, Result as SynResult, ReturnType, Type,
    Visibility,
};

pub(crate) struct DialogueMeta {
    pub(super) vis: Visibility,
    pub(super) ident: Ident,
    pub(super) ident_inner: Ident,
    pub(super) handler: ItemFn,
    pub(super) state: Path,
    pub(super) context: Box<Type>,
    pub(super) input: Box<Type>,
    pub(super) error: Type,
}

impl Parse for DialogueMeta {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let mut handler = input.parse::<ItemFn>()?;
        let vis = handler.vis.clone();
        let ident = handler.sig.ident.clone();
        let ident_inner = Ident::new(&format!("__carapax_{}", ident), Span::call_site());
        handler.sig.ident = ident_inner.clone();

        if handler.sig.asyncness.is_none() {
            return Err(input.error("function must be async"));
        }
        let (state_type, context_type, input_type) = {
            let mut inputs = handler.sig.inputs.iter();
            let state_type = match inputs.next() {
                Some(FnArg::Typed(typed)) => match &*typed.ty {
                    Type::Path(type_path) => type_path.path.clone(),
                    _ => return Err(input.error("state must be a path")),
                },
                _ => return Err(input.error("unable to detect dialogue state")),
            };
            let context_type = match inputs.next() {
                Some(FnArg::Typed(typed)) => match &*typed.ty {
                    Type::Reference(reference) => reference.elem.clone(),
                    _ => return Err(input.error("context must be a reference")),
                },
                _ => return Err(input.error("unable to detect handler context")),
            };
            let input_type = match inputs.next() {
                Some(FnArg::Typed(arg)) => arg.ty.clone(),
                _ => return Err(input.error("unable to detect handler input")),
            };
            (state_type, context_type, input_type)
        };
        let mut error_type = None;
        if let ReturnType::Type(_, ref path) = handler.sig.output {
            if let Type::Path(type_path) = &**path {
                if let Some(PathSegment {
                    arguments: PathArguments::AngleBracketed(bracketed),
                    ..
                }) = type_path.path.segments.last()
                {
                    if let Some(GenericArgument::Type(ty)) = bracketed.args.last() {
                        error_type = Some(ty.clone());
                    }
                }
            }
        }
        let error_type = error_type.ok_or_else(|| input.error("Unable to detect handler error type"))?;
        Ok(DialogueMeta {
            vis,
            ident,
            ident_inner,
            handler,
            state: state_type,
            context: context_type,
            input: input_type,
            error: error_type,
        })
    }
}
